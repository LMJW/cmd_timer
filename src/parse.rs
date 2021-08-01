use crate::timer::TimeoutDuration;
use crate::Result;
use anyhow::anyhow;

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
