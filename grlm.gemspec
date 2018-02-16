# coding: utf-8
lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'grlm/version'

Gem::Specification.new do |spec|
  spec.name          = "grlm"
  spec.version       = GRLM::VERSION
  spec.authors       = ["Manfred Endres"]
  spec.email         = ["manfred.endres@tslarusso.de"]

  spec.summary       = %q{Monitor the github rate limit value of a user}
  spec.description   = %q{A small tool that lets you monitor the github rate limit value of a user}
  spec.homepage      = "https://github.com/Larusso/github-rate-limit-monitor"
  spec.license       = "MIT"

  spec.files         = `git ls-files -z`.split("\x0").reject do |f|
    f.match(%r{^(test|spec|features)/})
  end
  spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.add_runtime_dependency "docopt", "~> 0.5"
  spec.add_runtime_dependency "ruby-progressbar", "~> 1"

  spec.add_development_dependency "bundler", "~> 1.14"
  spec.add_development_dependency "autotest", "~> 4.4"
  spec.add_development_dependency "rake", "~> 10.0"
  spec.add_development_dependency "rspec", "~> 3.5"
  spec.add_development_dependency "rspec-autotest", "~> 1"
  spec.add_development_dependency "octokit", "~> 4.3"
end
