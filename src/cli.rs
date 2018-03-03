use github::AuthType;

use docopt::Docopt;
use libc;
use std::convert::From;
use std::fmt;

#[derive(Debug, Deserialize)]
struct Arguments {
    flag_login: Option<String>,
    flag_password: Option<String>,
    flag_access_token: Option<String>,
    flag_frequency: u64,
    flag_short: bool,
    flag_resource: Resource,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Resource { Core, Search, Graphql }

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
          &Resource::Core => write!(f, "core"),
          &Resource::Search => write!(f, "search"),
          &Resource::Graphql => write!(f, "graphql"),
        }
    }
}

#[derive(Debug)]
pub struct Options {
    pub frequency: u64,
    pub auth: AuthType,
    pub resource: Resource,
    pub is_tty: bool,
}

impl From<Arguments> for Options {
    fn from(item: Arguments) -> Self {
        let auth = match item {
            Arguments { flag_access_token: Some(token), .. } => {
                AuthType::Token(token)
            },
            Arguments { flag_login: Some(login), flag_password: Some(password), ..} => {
                AuthType::Login {
                    login: login,
                    password: password
                }
            },
            _ => AuthType::Anonymos
        };

        Options {
          resource: item.flag_resource,
          frequency: item.flag_frequency,
          auth: auth,
          is_tty: is_tty(),
        }
    }
}

fn is_tty() -> bool {
    let tty = unsafe { libc::isatty(libc::STDOUT_FILENO as i32) } != 0;
    tty
}

pub fn get_options(usage: &str) -> Option<Options> {
    let version = format!("{}.{}.{}{}",
                     env!("CARGO_PKG_VERSION_MAJOR"),
                     env!("CARGO_PKG_VERSION_MINOR"),
                     env!("CARGO_PKG_VERSION_PATCH"),
                     option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));

    let args: Arguments = Docopt::new(usage)
                              .and_then(|d| Ok(d.version(Some(version))))
                              .and_then(|d| d.deserialize())
                              .unwrap_or_else(|e| e.exit());
    Some(args.into())
}