extern crate grlm;

const USAGE: &'static str = "
grlm - github rate limit monitor

Usage:
  grlm [(-l <user> -p <password> | -t <token>)] [--short | --progress] [options]
  grlm --version
  grlm -h | --help

Options:
  -l <user>, --login <user>                the github username
  -p <password>, --password <password>     the user password
  -t <token>, --access-token <token>       an github accesstoken
  -f <frequency>, --frequency <frequency>  refresh freqency [default: 10]
  -r <resource>, --resource <resource>     define which github resource to show
                                           Valid values: core, search, graphql [default: core]
  --short                                  display rate limit in short format.
  --progress                               display rate limit as progressbar [default].
  -V, --version                            print version
  -h, --help                               show this help message and exit
";

fn main() {
  let options = grlm::cli::get_options(USAGE);
  if let Some(o) = options {
    grlm::Monitor::start(o);
  }
}
