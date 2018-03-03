use std::io;

use futures::{Future, Stream};
use hyper::Client;
use hyper::header::{Authorization, UserAgent, Bearer, Basic};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

use hyper;
use serde_json;

use std::time::{SystemTime, UNIX_EPOCH};

use std::result::{Result as R};
use failure::Error;

#[derive(Debug)]
pub enum AuthType {
    Anonymos,
    Token(String),
    Login { login: String, password: String }
}

pub type Result<T> = R<T, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct RateLimitResult {
    pub resources: GithubRateLimit,
    pub rate: RateLimit
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubRateLimit {
    pub core: RateLimit,
    pub search: RateLimit,
    pub graphql: RateLimit
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RateLimit {
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64
}

impl RateLimit {
    pub fn resets_in(&self) -> i64 {
        if self.remaining == self.limit {
            self.remaining as i64
        }
        else {
            let utc_secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
            self.reset as i64 - utc_secs
        }
    }
}

pub fn fetch_rate_limit(auth : &AuthType ) -> Result<RateLimitResult> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
                 .connector(HttpsConnector::new(4, &handle)?)
                 .build(&handle);

    let uri = "https://api.github.com/rate_limit".parse()?;

    let mut req = hyper::Request::new(hyper::Method::Get, uri);

    if let &AuthType::Token(ref token) = auth {
        req.headers_mut().set(Authorization(Bearer {token: token.to_owned()}));
    }

    if let &AuthType::Login {ref login, ref password} = auth {
        req.headers_mut().set(Authorization(Basic { username: login.to_owned(), password: Some(password.to_owned())}));
    }

    req.headers_mut().set(UserAgent::new("curl/7.54.0"));

    let work = client.request(req).and_then(|res| {
        res.body().concat2().and_then(move |body| {
            let v: RateLimitResult = serde_json::from_slice(&body).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    e
                )
            })?;
            Ok(v)
        })
    });
    let result = core.run(work)?;
    Ok(result)
}