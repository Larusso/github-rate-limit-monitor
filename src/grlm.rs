#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate docopt;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate failure;
extern crate indicatif;
extern crate parking_lot;
extern crate libc;

mod github;
mod output;

pub mod cli;
use self::cli::Resource;
use self::github::{AuthType, RateLimitResult, RateLimit, fetch_rate_limit};
use self::output::Output;

use parking_lot::{RwLock};

use std::thread;
use std::sync::Arc;
use std::time::{Duration, Instant};

struct MonitorState {
    output: Output,
    rate_limit: Option<RateLimitResult>,
    poll_frequency: Duration,
    last_update: Instant,
    auth: AuthType,
    resource: Resource,
}

pub struct Monitor {
    state: Arc<RwLock<MonitorState>>,
}

impl RateLimit {
    fn progress_chars(&self) -> &'static str {
        match self.remaining {
            x if x == 0 => "#####",
            _ => " \u{15E7}\u{FF65}",
        }
    }

    fn rate_color(&self) -> &'static str {
        match (self.remaining as f64) / (self.limit as f64) {
            x if x <= 0.08 => "red",
            x if x <= 0.5 => "yellow",
            _ => "green"
        }
    }

    fn message_color(&self) -> &'static str {
        match self.resets_in() {
            x if x < 120 => "green",
            _ => "white"
        }
    }
}

impl Monitor {
    pub fn new(args : cli::Options) -> Monitor {
        let f = args.frequency;
        let auth = args.auth;
        let resource = args.resource;
        let initial_length = match auth {
            AuthType::Anonymos => 60,
            _ => 5000,
        };

        let output = Output::new(args.output_style, initial_length, &resource);

        Monitor {
            state: Arc::new(RwLock::new(MonitorState {
                output: output,
                rate_limit: None,
                poll_frequency: f,
                last_update: Instant::now() - (f * 2),
                auth: auth,
                resource: resource,
            })),
        }
    }

    pub fn tick(&self, force_update : bool) {
        if force_update || self.state.read().last_update.elapsed() >= self.state.read().poll_frequency {
            let mut state = self.state.write();
            match fetch_rate_limit(&state.auth) {
                Ok(r) => state.rate_limit = Some(r),
                Err(e) => println!("Error {}", e),
            }
            state.last_update = Instant::now();
        }
        if let Some(ref r) = self.state.read().rate_limit {
            let rate = match self.state.read().resource {
                Resource::Core => &r.resources.core,
                Resource::Search => &r.resources.search,
                Resource::Graphql => &r.resources.graphql,
            };

            let ref output = self.state.read().output;
            output.update(rate);
        }
    }

    pub fn start_ticker(&self) {
        loop {
            self.tick(false);
            thread::sleep(Duration::from_millis(1000/30));
        }
    }

    pub fn start(args : cli::Options) {
        let m = Monitor::new(args);
        m.start_ticker();
    }
}