require "octokit"
require "ruby-progressbar"
require 'ruby-progressbar/output'

module GRLM
  class Out < ProgressBar::Output
    DEFAULT_FORMAT_STRING = '%t: |%B|'.freeze
    alias refresh_with_format_change with_refresh

    def stream
      $stdout
    end

    def clear
      stream.print clear_string
      stream.print "\r"
    end

    def bar_update_string
      bar.to_s
    end

    def default_format
      DEFAULT_FORMAT_STRING
    end

    def resolve_format(other_format)
      other_format || default_format
    end

    def eol
      "\r"
    end
  end

  class Monitor

    GREEN  = "\e[0;92m"
    RED    = "\e[0;91m"
    YELLOW = "\e[0;93m"
    RESET  = "\e[0m"

    def initialize credentials
      @client = Octokit::Client.new(**credentials)
      @rateLimit = nil
      @reached_zero = false
      format = "%t %c \e[0;93m%b\u{15E7} %i\e[0m of %C"
      @progressbar = ProgressBar.create(title: 'Requests',
                                        output: Out,
                                        starting_at: 0,
                                        total: nil,
                                        format: format,
                                        progress_mark: ' ',
                                        :unknown_progress_animation_steps => ["\u{FF65}"],
                                        remainder_mark: "\u{FF65}")
    end

    def update()
      @rateLimit = @client.rate_limit!
      remaining = @rateLimit.remaining
      limit = @rateLimit.limit
      reset = @rateLimit.resets_in

      if remaining == 0.0
        @progressbar.pause
      end

      character_color = YELLOW
      requests_color = RESET
      bar_color = YELLOW
      reset_color = RESET

      if remaining.to_f / limit.to_f >= 0.5
        requests_color = GREEN
      elsif remaining / limit < 0.5
        requests_color = YELLOW
      end

      if remaining.to_f / limit.to_f <= 0.08
        requests_color = RED
        character_color = RED
        bar_color = RED
      end

      if reset < 120
        reset_color = GREEN
      end

      @progressbar.total = limit
      @progressbar.format = "%t #{requests_color}%c\e[0m #{character_color}%b\u{15E7}\e[0m#{bar_color} %i\e[0m of %C reset in #{reset_color}#{reset.to_s.rjust(4)}\e[0m"
      @progressbar.progress = limit - remaining
    end
  end
end
