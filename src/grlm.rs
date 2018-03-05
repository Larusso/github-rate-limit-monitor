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

pub mod cli;
use self::cli::Resource;
use self::github::{AuthType, RateLimitResult, RateLimit, fetch_rate_limit};

use indicatif::ProgressBar;
use indicatif::ProgressDrawTarget;
use indicatif::ProgressStyle;

use parking_lot::{RwLock};

use std::thread;
use std::sync::Arc;
use std::time::{Duration, Instant};

struct MonitorState {
    bar: ProgressBar,
    rate_limit: Option<RateLimitResult>,
    poll_frequency: Duration,
    last_update: Instant,
    auth: AuthType,
    short: bool,
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
        let bar = ProgressBar::new(initial_length);
        bar.set_draw_target(ProgressDrawTarget::stderr_nohz());
        if args.short {
            bar.set_style(ProgressStyle::default_bar()
            .template(&format!("{{prefix:.bold}} {{pos}}/{{len}} {{msg.{}}} ", "yellow"))
            .progress_chars(" \u{15E7}\u{FF65}"));
        }
        else {
            bar.set_style(ProgressStyle::default_bar()
            .template(&format!("{{prefix:.bold}} {{pos}} {{wide_bar:.{}}} of {{len}} {{msg.{}}} ", "yellow", "yellow"))
            .progress_chars(" \u{15E7}\u{FF65}"));
        }

        bar.set_prefix(&format!("Requests {}:", resource));
        Monitor {
            state: Arc::new(RwLock::new(MonitorState {
                bar: bar,
                rate_limit: None,
                poll_frequency: f,
                last_update: Instant::now() - (f * 2),
                auth: auth,
                resource: resource,
                short: args.short,
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

            let ref bar = self.state.read().bar;
            bar.set_length(rate.limit);
            bar.set_message(&format!("resets in {}",rate.resets_in()));
            bar.set_position(rate.limit - rate.remaining);
            if self.state.read().short {
                bar.set_style(ProgressStyle::default_bar()
                .template(&format!("{{prefix:.bold}} {{pos:.{}}}/{{len}} {{msg:.{}}} ", rate.rate_color(), rate.message_color()))
                .progress_chars(" \u{15E7}\u{FF65}"));
            }
            else {
                bar.set_style(ProgressStyle::default_bar()
               .template(&format!("{{prefix:.bold}} {{pos:.{}}} {{wide_bar:.{}}} of {{len}} {{msg:.{}}} ", rate.rate_color(), "yellow", rate.message_color()))
               .progress_chars(rate.progress_chars()));
            }
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