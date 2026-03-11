use std::time::Duration;

use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct DurationString(String);

#[derive(Debug, Error)]
pub enum ParseDurationError {
    #[error("Not digit at pos {0}")]
    NotDigit(usize),

    #[error("Empty string not allowed")]
    Empty,

    #[error("Parse number error: {0} at pos {1}")]
    ParseNumber(std::num::ParseIntError, usize),

    #[error("Invalid character {0} at pos {1}")]
    InvalidCharacter(char, usize),

    #[error("Unit out of order: {0}")]
    UnitOutOfOrder(String),

    #[error("Duplicate unit: {0}")]
    DuplicateUnit(String),
}

impl TryFrom<Duration> for DurationString {
    type Error = ParseDurationError;
    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        let mut res = String::new();
        let mut secs = value.as_secs();

        let days = secs / (24 * 60 * 60);
        if days > 0 {
            secs -= days * 24 * 60 * 60;
            res += &format!("{}d", days);
        }
        let hours = secs / (60 * 60);
        if hours > 0 {
            secs -= hours * 60 * 60;
            res += &format!("{}h", hours);
        }
        let minutes = secs / 60;
        if minutes > 0 {
            secs -= minutes * 60;
            res += &format!("{}m", minutes);
        }
        if secs > 0 {
            res += &format!("{}s", secs);
        }
        Ok(DurationString(res))
    }
}

impl TryFrom<DurationString> for Duration {
    type Error = ParseDurationError;

    fn try_from(value: DurationString) -> Result<Self, Self::Error> {
        if value.0.is_empty() {
            return Err(ParseDurationError::Empty);
        }

        let mut total_secs = 0;

        let s = value.0.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut seen = [false; 4];
        let mut last_unit_index = 0;

        let mut currrent_num_str = String::new();

        while i < s.len() {
            let c = s[i];

            match c {
                '0'..='9' => {
                    currrent_num_str.push(c);
                    i += 1;
                }
                'd' | 'h' | 'm' | 's' => {
                    let num: u64 = currrent_num_str
                        .parse()
                        .map_err(|e| ParseDurationError::ParseNumber(e, i))?;

                    let unit_idx = match c {
                        'd' => 0,
                        'h' => 1,
                        'm' => 2,
                        's' => 3,
                        _ => unreachable!(),
                    };

                    // gereteen unit index
                    if last_unit_index > unit_idx {
                        return Err(ParseDurationError::UnitOutOfOrder(c.to_string()));
                    }

                    // no duplicate unit
                    if seen[unit_idx] {
                        return Err(ParseDurationError::DuplicateUnit(c.to_string()));
                    }
                    seen[unit_idx] = true;
                    last_unit_index = unit_idx;

                    total_secs += match c {
                        'd' => num * 24 * 60 * 60,
                        'h' => num * 60 * 60,
                        'm' => num * 60,
                        's' => num,
                        _ => unreachable!(),
                    };
                    currrent_num_str.clear();
                    i += 1;
                }
                c if c.is_whitespace() => {
                    i += 1;
                }
                _ => {
                    return Err(ParseDurationError::InvalidCharacter(c, i));
                }
            }
        }

        // for c in s.into_iter() {}

        if !currrent_num_str.is_empty() {
            return Err(ParseDurationError::NotDigit(i));
        }

        Ok(Duration::from_secs(total_secs))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::DurationString;
    use eyre::Result;

    #[test]
    fn test_des() -> Result<()> {
        let dur = Duration::try_from(DurationString("1d".to_string()));
        assert!(dur.is_ok());
        assert_eq!(dur.unwrap(), Duration::from_hours(24));

        let dur = Duration::try_from(DurationString("1".to_string()));
        assert!(dur.is_err());

        Ok(())
    }
}
