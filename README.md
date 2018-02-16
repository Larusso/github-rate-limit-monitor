Github Rate Limit Monitor
=========================

`grlm` is a small and funny commandline tool written in ruby.
It polls the github api [rateLimit](https://developer.github.com/v3/rate_limit/) endpoint and fills a nice little progressbar.

![screencast](docs/grlm.gif)

Usage
-----

```bash
Usage:
  grlm (-l <user> -p <password> | -t <token>) [-f <frequency>]
  grlm --version
  grlm -h | --help

Options:
-l <user>, --login <user>               the github username
-p <password>, --password <password>    the user password
-t <token>, --accesstoken <token>       an github accesstoken
-f <frequency>, --frequency <frequency  refresh freqency [default: 10]
--version                               print version
-h, --help                              show this help message and exit
```

Installation
------------

### From Homebrew

```
brew tap wooga/tools
brew install grlm
```

### From Source

1. Git clone the repo and `cd` into the directory. 
2. `bundle install`
3. `gem build grlm.gemspec`
4. `gem install grlm-1.0.0.gem`

License
-------

[MIT License](http://opensource.org/licenses/MIT).

