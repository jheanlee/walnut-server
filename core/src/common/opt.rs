#[derive(clap::Parser)]
pub struct Args {
  #[arg(short = 'd', long, default_value_t = {"sqlite://./walnut.sqlite?mode=rwc".to_string()})]
  pub database: String,
  #[arg(short = 'j', long, default_value_t = {"./credentials".to_string()})]
  pub jwt_credentials_dir: String,
  #[arg(long, default_value_t = false)]
  pub self_signup: bool,
  #[arg(short = 'v', long, default_value_t = 20)]
  pub verbose: u8,
}

impl Args {
  pub fn verbose_level(&self) -> log::LevelFilter {
    match self.verbose {
      0..=9 => {
        log::LevelFilter::Trace
      }
      10..=19 => {
        log::LevelFilter::Debug
      }
      20..=29 => {
        log::LevelFilter::Info
      }
      30..=39 => {
        log::LevelFilter::Warn
      }
      40.. => {
        log::LevelFilter::Error
      }
    }
  }
}