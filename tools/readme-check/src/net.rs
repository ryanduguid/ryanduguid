//! HTTP fetching with the retry policy of the website's link checker
//! (`site-tools check_links` in ryanduguid/ryanduguid.github.io): up to 5
//! attempts; timeouts, transport failures, HTTP 429 and 5xx retry, 429 and 503
//! honouring a Retry-After in seconds, otherwise backing off 1, 2, 4 and 8
//! seconds, with at most 30 seconds of waiting per URL. Other HTTP 4xx
//! responses do not retry.
//!
//! A URL that still fails with a timeout, 429 or 5xx is inconclusive: badge and
//! credential hosts rate-limit runners, and a brief outage is not a dead link.
//! An HTTP 999 response does not retry and is a refusal: reported, never a
//! finding. Other 4xx, a DNS failure or another transport failure is a dead link.

use std::time::Duration;

pub const MAX_ATTEMPTS: u32 = 5;
const MAX_WAIT_SECONDS: u64 = 30;
const BODY_LIMIT_BYTES: u64 = 16 * 1024 * 1024;
const USER_AGENT: &str = "ryanduguid-readme-check/1.0 (+https://github.com/ryanduguid/ryanduguid)";

/// One HTTP attempt as the transport saw it.
#[derive(Debug, Clone)]
pub enum Reply {
    Status {
        code: u16,
        retry_after: Option<u64>,
        body: String,
    },
    Timeout,
    Transport(String),
}

pub trait Transport {
    fn get(&mut self, url: &str) -> Reply;
    fn sleep(&mut self, seconds: u64);
}

/// The result of fetching a URL with retries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Ok(String),
    Dead(String),
    Transient(String),
    Refused(String),
}

pub fn fetch(transport: &mut dyn Transport, url: &str) -> Outcome {
    let mut waited = 0;
    let mut last = Outcome::Transient(String::from("not attempted"));
    for attempt in 1..=MAX_ATTEMPTS {
        let hint = match transport.get(url) {
            Reply::Status { code, body, .. } if code < 400 => return Outcome::Ok(body),
            Reply::Status { code: 999, .. } => {
                return Outcome::Refused("HTTP 999 (the host refuses automated clients)".into());
            }
            Reply::Status {
                code, retry_after, ..
            } if code == 429 || code >= 500 => {
                last = Outcome::Transient(format!("HTTP {code} after {attempt} attempts"));
                retry_after
            }
            Reply::Status { code, .. } => return Outcome::Dead(format!("HTTP {code}")),
            Reply::Timeout => {
                last = Outcome::Transient(format!("timed out after {attempt} attempts"));
                None
            }
            Reply::Transport(message) => {
                last = Outcome::Dead(message);
                None
            }
        };
        let wait = hint.unwrap_or(1 << (attempt - 1));
        if attempt == MAX_ATTEMPTS || waited + wait > MAX_WAIT_SECONDS {
            break;
        }
        transport.sleep(wait);
        waited += wait;
    }
    last
}

const GITHUB_API: &str = "https://api.github.com/";

/// The real network. The token, when present, is sent only to the GitHub API.
pub struct Network {
    agent: ureq::Agent,
    bearer: Option<String>,
    api: String,
}

impl Network {
    pub fn new(bearer: Option<String>) -> Self {
        Self::with(bearer, GITHUB_API, Duration::from_secs(30))
    }

    /// A network that sends `bearer` only to URLs under `api`, giving up on a
    /// request after `timeout`.
    fn with(bearer: Option<String>, api: &str, timeout: Duration) -> Self {
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(timeout))
            .http_status_as_error(false)
            // The token must never follow a redirect to another host.
            .redirect_auth_headers(ureq::config::RedirectAuthHeaders::Never)
            .build()
            .into();
        Self {
            agent,
            bearer,
            api: api.to_owned(),
        }
    }
}

impl Transport for Network {
    fn get(&mut self, url: &str) -> Reply {
        let mut request = self.agent.get(url).header("User-Agent", USER_AGENT);
        if let Some(bearer) = self.bearer.as_deref()
            && url.starts_with(&self.api)
        {
            request = request.header("Authorization", format!("Bearer {bearer}"));
        }
        let mut response = match request.call() {
            Ok(response) => response,
            Err(error) => return transport_error(error),
        };
        let code = response.status().as_u16();
        let retry_after = response
            .headers()
            .get("Retry-After")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.trim().parse().ok());
        let body = if code < 400 {
            match response
                .body_mut()
                .with_config()
                .limit(BODY_LIMIT_BYTES)
                .read_to_vec()
            {
                Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                Err(error) => return transport_error(error),
            }
        } else {
            String::new()
        };
        Reply::Status {
            code,
            retry_after,
            body,
        }
    }

    fn sleep(&mut self, seconds: u64) {
        std::thread::sleep(Duration::from_secs(seconds));
    }
}

fn transport_error(error: ureq::Error) -> Reply {
    match error {
        ureq::Error::Timeout(_) => Reply::Timeout,
        other => Reply::Transport(other.to_string()),
    }
}

#[cfg(test)]
pub mod fake {
    use super::{Reply, Transport};
    use std::collections::HashMap;

    /// Scripted replies per URL; the last reply repeats. Unscripted URLs answer 200.
    #[derive(Default)]
    pub struct Fake {
        pub replies: HashMap<String, Vec<Reply>>,
        pub calls: Vec<String>,
        pub slept: u64,
    }

    impl Fake {
        pub fn ok(mut self, url: &str, body: &str) -> Self {
            self.replies.insert(url.into(), vec![status(200, body)]);
            self
        }

        pub fn script(mut self, url: &str, replies: Vec<Reply>) -> Self {
            self.replies.insert(url.into(), replies);
            self
        }
    }

    pub fn status(code: u16, body: &str) -> Reply {
        Reply::Status {
            code,
            retry_after: None,
            body: body.into(),
        }
    }

    impl Transport for Fake {
        fn get(&mut self, url: &str) -> Reply {
            let n = self.calls.iter().filter(|seen| *seen == url).count();
            self.calls.push(url.into());
            match self.replies.get(url) {
                Some(replies) => replies[n.min(replies.len() - 1)].clone(),
                None => status(200, ""),
            }
        }

        fn sleep(&mut self, seconds: u64) {
            self.slept += seconds;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fake::{Fake, status};
    use super::{MAX_ATTEMPTS, Outcome, Reply, fetch};

    const URL: &str = "https://example.test/";

    #[test]
    fn a_404_is_dead_without_retrying() {
        let mut net = Fake::default().script(URL, vec![status(404, "")]);
        assert_eq!(fetch(&mut net, URL), Outcome::Dead("HTTP 404".into()));
        assert_eq!(net.calls.len(), 1);
    }

    #[test]
    fn a_999_is_refused_without_retrying() {
        let mut net = Fake::default().script(URL, vec![status(999, "")]);
        assert!(matches!(fetch(&mut net, URL), Outcome::Refused(_)));
        assert_eq!(net.calls.len(), 1);
    }

    #[test]
    fn a_rate_limit_that_clears_passes() {
        let mut net = Fake::default().script(URL, vec![status(429, ""), status(200, "ok")]);
        assert_eq!(fetch(&mut net, URL), Outcome::Ok("ok".into()));
        assert_eq!(net.slept, 1);
    }

    #[test]
    fn a_persistent_rate_limit_is_inconclusive() {
        let mut net = Fake::default().script(URL, vec![status(429, "")]);
        assert!(matches!(fetch(&mut net, URL), Outcome::Transient(_)));
        assert_eq!(net.calls.len(), MAX_ATTEMPTS as usize);
        assert_eq!(net.slept, 1 + 2 + 4 + 8);
    }

    #[test]
    fn a_long_retry_after_stops_retrying_within_the_wait_cap() {
        let reply = Reply::Status {
            code: 503,
            retry_after: Some(120),
            body: String::new(),
        };
        let mut net = Fake::default().script(URL, vec![reply]);
        assert!(matches!(fetch(&mut net, URL), Outcome::Transient(_)));
        assert_eq!((net.calls.len(), net.slept), (1, 0));
    }

    #[test]
    fn a_dns_failure_that_persists_is_dead() {
        let mut net = Fake::default().script(URL, vec![Reply::Transport("dns".into())]);
        assert_eq!(fetch(&mut net, URL), Outcome::Dead("dns".into()));
        assert_eq!(net.calls.len(), MAX_ATTEMPTS as usize);
    }

    #[test]
    fn a_timeout_that_persists_is_inconclusive() {
        let mut net = Fake::default().script(URL, vec![Reply::Timeout]);
        assert!(matches!(fetch(&mut net, URL), Outcome::Transient(_)));
    }
}

#[cfg(test)]
mod network_tests {
    use super::{Network, Reply, Transport};
    use std::io::{Read, Write};
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    /// Reads one request's head from `stream`, lowercased.
    fn request_head(stream: &mut TcpStream) -> String {
        let mut head = Vec::new();
        let mut byte = [0; 1];
        while !head.ends_with(b"\r\n\r\n") && stream.read(&mut byte).unwrap_or(0) == 1 {
            head.push(byte[0]);
        }
        String::from_utf8_lossy(&head).to_lowercase()
    }

    /// A local server answering each connection with the next of `responses`
    /// (raw HTTP), sending back the request heads it read.
    fn serve(responses: Vec<&'static str>) -> (SocketAddr, mpsc::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let (heads, received) = mpsc::channel();
        thread::spawn(move || {
            for response in responses {
                let (mut stream, _) = listener.accept().unwrap();
                heads.send(request_head(&mut stream)).unwrap();
                stream.write_all(response.as_bytes()).unwrap();
            }
        });
        (addr, received)
    }

    fn network(addr: SocketAddr, timeout: Duration) -> Network {
        Network::with(
            Some("t0ken".into()),
            &format!("http://{addr}/api/"),
            timeout,
        )
    }

    #[test]
    fn the_token_goes_only_to_the_api_and_bodies_are_read() {
        const OK: &str = "HTTP/1.1 200 OK\r\nContent-Length: 5\r\nConnection: close\r\n\r\nhello";
        let (addr, heads) = serve(vec![OK, OK]);
        let mut net = network(addr, Duration::from_secs(5));
        for (path, sends_token) in [("api/repos", true), ("elsewhere", false)] {
            let reply = net.get(&format!("http://{addr}/{path}"));
            assert!(
                matches!(reply, Reply::Status { code: 200, retry_after: None, ref body } if body == "hello")
            );
            let head = heads.recv().unwrap();
            assert!(head.starts_with(&format!("get /{path} ")));
            assert!(head.contains("user-agent: ryanduguid-readme-check/"));
            assert_eq!(head.contains("authorization: bearer t0ken"), sends_token);
        }
    }

    #[test]
    fn an_error_status_keeps_retry_after_and_drops_the_body() {
        const BUSY: &str = "HTTP/1.1 503 Service Unavailable\r\nRetry-After: 7\r\nContent-Length: 4\r\nConnection: close\r\n\r\nbusy";
        let (addr, _heads) = serve(vec![BUSY]);
        let reply = network(addr, Duration::from_secs(5)).get(&format!("http://{addr}/"));
        assert!(
            matches!(reply, Reply::Status { code: 503, retry_after: Some(7), ref body } if body.is_empty())
        );
    }

    #[test]
    fn a_body_cut_short_is_a_transport_failure() {
        const SHORT: &str =
            "HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\nhello";
        let (addr, _heads) = serve(vec![SHORT]);
        let reply = network(addr, Duration::from_secs(5)).get(&format!("http://{addr}/"));
        assert!(matches!(reply, Reply::Transport(_)), "{reply:?}");
    }

    #[test]
    fn a_refused_connection_is_a_transport_failure() {
        let addr = TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap();
        let reply = network(addr, Duration::from_secs(5)).get(&format!("http://{addr}/"));
        assert!(matches!(reply, Reply::Transport(_)), "{reply:?}");
    }

    #[test]
    fn a_server_that_never_answers_times_out() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let mut net = network(addr, Duration::from_millis(300));
        let reply = net.get(&format!("http://{addr}/"));
        assert!(matches!(reply, Reply::Timeout), "{reply:?}");
        drop(listener);
        net.sleep(0);
    }
}
