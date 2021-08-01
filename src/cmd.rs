use crate::parse::parse_duration;
use crate::timer::TimeoutDuration;
/// Command line parse the time
use structopt::StructOpt;

#[derive(StructOpt, Debug, Default)]
pub struct TimerOpt {
    #[structopt(parse(try_from_str = parse_duration))]
    pub duration: TimeoutDuration,
    pub title: String,
}
