#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
  /// Host address in the format `IP_OR_DOMAIN`:`PORT`, e.g., `example.com:3000`
  pub host: String
}