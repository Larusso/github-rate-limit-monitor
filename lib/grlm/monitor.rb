require "octokit"
require "ruby-progressbar"

module GRLM
  class Monitor

    GREEN  = "\e[0;92m"
    RED    = "\e[0;91m"
    YELLOW = "\e[0;93m"
    RESET  = "\e[0m"

    def initialize credentials
      @client = Octokit::Client.new(**credentials)
      @rateLimit = nil
      format = "%t %c \e[0;93m%b\u{15E7} %i\e[0m of %C"
      @progressbar = ProgressBar.create(title: 'API requests',
                                        starting_at: 0,
                                        total: nil,
                                        format: format,
                                        progress_mark: ' ',
                                        remainder_mark: "\u{FF65}")
    end

    def update()
      @rateLimit = @client.rate_limit!
      @progressbar.total = @rateLimit.limit
      @progressbar.progress = @rateLimit.limit - @rateLimit.remaining

      remaining = @rateLimit.remaining.to_f
      limit = @rateLimit.limit.to_f
      reset = @rateLimit.resets_in

      character_color = YELLOW
      requests_color = RESET
      bar_color = YELLOW
      reset_color = RESET

      if remaining / limit >= 0.5
        requests_color = GREEN
      elsif remaining / limit < 0.5
        requests_color = YELLOW
      elsif remaining / limit <= 0.04
        requests_color = RED
        character_color = RED
        bar_color = RED
      end

      if reset < 120
        reset_color = GREEN
      end

      @progressbar.format = "%t #{requests_color}%c\e[0m #{character_color}%b\u{15E7}\e[0m#{bar_color} %i\e[0m of %C reset in #{reset_color}#{reset.to_s.rjust(4)}\e[0m"
    end
  end
end
