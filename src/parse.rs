use crate::Result;
use anyhow::anyhow;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TimeoutDuration {
    hours: i32,
    minutes: i32,
    seconds: i32,
    // track the progress of the timeout
    progress: i32,
}

impl TimeoutDuration {
    pub fn total_seconds(&self) -> i32 {
        (self.hours * 60 + self.minutes) * 60 + self.seconds
    }

    pub fn is_zero(&self) -> bool {
        self.total_seconds() == 0
    }

    pub fn is_valid(self) -> Result<Self> {
        if self.total_seconds() > 24 * 60 * 60 {
            return Err(anyhow!(
                "Invalid: Cannot have a time that is greater than 24 hours"
            ));
        }
        Ok(self)
    }

    pub fn new(hours: i32, minutes: i32, seconds: i32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
            progress: 0,
        }
    }

    pub fn tick(&mut self) {
        self.progress += 1
    }

    pub fn is_timeout(&self) -> bool {
        self.progress > self.total_seconds()
    }

    pub fn summary(&self) -> String {
        format!(
            "Total time: {} Hour {} Minutes {} Seconds",
            self.hours, self.minutes, self.seconds
        )
    }
}

impl Into<String> for &TimeoutDuration {
    fn into(self) -> String {
        let current = self.total_seconds() - self.progress;
        let sign = if current > 0 { "" } else { "-" };
        let hour = current.abs() / 3600;
        let minute = (current.abs() / 60) % 60;
        let seconds = current.abs() % 60;
        format!("{}{:02}:{:02}:{:02}", &sign, hour, minute, seconds)
    }
}

#[derive(Debug)]
enum Token {
    Number(i32),
    Unit(TimeUnit),
}

#[derive(Debug)]
enum TimeUnit {
    Hour,
    Minute,
    Second,
}

struct SimpleDurationParser {
    tokens: Vec<Token>,
}

impl SimpleDurationParser {
    fn new(raw: &str) -> Result<Self> {
        let mut tokens = Vec::new();
        let mut num = 0;
        for c in raw.trim().chars() {
            if c.is_ascii_digit() {
                num = num * 10 + c.to_digit(10).unwrap();
            } else if c == 'h' || c == 'H' {
                tokens.push(Token::Number(num as i32));
                tokens.push(Token::Unit(TimeUnit::Hour));
                num = 0;
            } else if c == 'm' || c == 'M' {
                tokens.push(Token::Number(num as i32));
                tokens.push(Token::Unit(TimeUnit::Minute));
                num = 0;
            } else if c == 's' || c == 'S' {
                tokens.push(Token::Number(num as i32));
                tokens.push(Token::Unit(TimeUnit::Second));
                num = 0;
            } else {
                return Err(anyhow!(
                    "unknown character `{}` when try to parse `{}`",
                    c,
                    raw
                ));
            }
        }

        Ok(Self { tokens })
    }

    fn parse(&mut self) -> Result<TimeoutDuration> {
        let mut ret = TimeoutDuration::default();
        let mut buffer = Vec::new();
        for token in &self.tokens {
            match token {
                Token::Unit(t) => {
                    if let Some(num) = buffer.pop() {
                        match t {
                            TimeUnit::Hour => {
                                if ret.hours != 0 {
                                    return Err(anyhow!(
                                        "Parse Error: Should not have more than one hour block"
                                    ));
                                } else {
                                    ret.hours = num;
                                }
                            }
                            TimeUnit::Minute => {
                                if ret.minutes != 0 {
                                    return Err(anyhow!(
                                        "Parse Error: Should not have more than one hour block"
                                    ));
                                } else {
                                    ret.minutes = num;
                                }
                            }
                            TimeUnit::Second => {
                                if ret.seconds != 0 {
                                    return Err(anyhow!(
                                        "Parse Error: Should not have more than one hour block"
                                    ));
                                } else {
                                    ret.seconds = num;
                                }
                            }
                        }
                    } else {
                        return Err(anyhow!(
                            "Parse Error: A number should added before `{:?}`",
                            t
                        ));
                    }
                }
                Token::Number(num) => {
                    buffer.push(*num);
                }
            }
        }
        if ret.is_zero() {
            return Err(anyhow!("Invalid: initial duration should not be zero"));
        }
        ret.is_valid()
    }
}

pub fn parse_duration(duration: &str) -> Result<TimeoutDuration> {
    let mut parser = SimpleDurationParser::new(duration)?;
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_duration;

    #[test]
    fn test_success() {
        assert_eq!(parse_duration("1h").unwrap(), TimeoutDuration::new(1, 0, 0));
        assert_eq!(parse_duration("1m").unwrap(), TimeoutDuration::new(0, 1, 0));
        assert_eq!(parse_duration("1s").unwrap(), TimeoutDuration::new(0, 0, 1));
        assert_eq!(
            parse_duration("1h15m").unwrap(),
            TimeoutDuration::new(1, 15, 0)
        );
        assert_eq!(
            parse_duration("1h15m60s").unwrap(),
            TimeoutDuration::new(1, 15, 60)
        );
    }

    #[test]
    fn test_fail() {
        assert!(parse_duration("h").is_err());
        assert!(parse_duration("30h").is_err());
        assert!(parse_duration("1h1h").is_err());
        assert!(parse_duration("0h0s").is_err());
    }
}
