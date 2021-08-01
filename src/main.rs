use structopt::StructOpt;
use tracing_subscriber;
mod cmd;
mod parse;
pub mod timer;
mod ui;

pub type Result<T> = anyhow::Result<T>;

fn main() {
    tracing_subscriber::fmt::init();
    let opt = cmd::TimerOpt::from_args();
    // TODO: fix the warnings
    ui::start(opt);
}
