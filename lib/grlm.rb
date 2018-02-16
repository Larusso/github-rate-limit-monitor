#!/usr/bin/env ruby

require_relative "grlm/version"
require_relative "grlm/monitor"

module GRLM
  def self.dispatch options
    credentials = {}
    credentials[:access_token] = options['--accesstoken']
    credentials[:login] = options['--login']
    credentials[:password] = options['--password']

    monitor = Monitor.new credentials
    trap("SIGINT") { exit! }
    while 0 do
      monitor.update()
      sleep(options['--frequency'].to_f)
    end
  end
end