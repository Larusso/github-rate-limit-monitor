use cli::Resource;
use github::RateLimit;
use indicatif::{ProgressBar,ProgressDrawTarget,ProgressStyle};

#[derive(Debug)]
pub enum OutputStyle {
    Progress, Short
}

pub struct Output {
    bar: ProgressBar,
    style: OutputStyle
}

impl Output {
    pub fn new(style: OutputStyle, initial_length :u64, resource: &Resource) -> Output {

        let bar = ProgressBar::new(initial_length);
        bar.set_draw_target(ProgressDrawTarget::stderr_nohz());

        match style {
            OutputStyle::Short => {
                bar.set_style(ProgressStyle::default_bar()
                .template(&format!("{{prefix:.bold}} {{pos}}/{{len}} {{msg.{}}} ", "yellow"))
                .progress_chars(" \u{15E7}\u{FF65}"));
            },

            OutputStyle::Progress => {
                bar.set_style(ProgressStyle::default_bar()
                .template(&format!("{{prefix:.bold}} {{pos}} {{wide_bar:.{}}} of {{len}} {{msg.{}}} ", "yellow", "yellow"))
                .progress_chars(" \u{15E7}\u{FF65}"));
            }
        };

        bar.set_prefix(&format!("Requests {}:", resource));
        Output {bar: bar, style: style}
    }

    pub fn update(&self, rate: &RateLimit) {
        let ref bar = self.bar;
        bar.set_length(rate.limit);
        bar.set_message(&format!("resets in {}",rate.resets_in()));
        bar.set_position(rate.limit - rate.remaining);
        match self.style {
            OutputStyle::Short => {
                bar.set_style(ProgressStyle::default_bar()
                .template(&format!("{{prefix:.bold}} {{pos:.{}}}/{{len}} {{msg:.{}}} ", rate.rate_color(), rate.message_color()))
                .progress_chars(" \u{15E7}\u{FF65}"));
            },
            OutputStyle::Progress => {
                bar.set_style(ProgressStyle::default_bar()
               .template(&format!("{{prefix:.bold}} {{pos:.{}}} {{wide_bar:.{}}} of {{len}} {{msg:.{}}} ", rate.rate_color(), "yellow", rate.message_color()))
               .progress_chars(rate.progress_chars()));
            },
        };
    }
}