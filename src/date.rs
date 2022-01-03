// Get current date from the UNIX timestep.
// stripped from https://gist.github.com/sadikovi/b708ed51f479d7b9e8b03515756c6d78

use std::time::SystemTime;

fn is_leap_year(year: i32) -> bool {
  year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn timestep_to_utc(ts: i64) -> String {
  static DAYS: [[i64; 12]; 2] = [
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
  ];
  let mut year = 1970;
  let mut month = 0;
  // seconds in a day
  // + 1 here since ts is a difference
  let mut day = ts / 86400 + 1;

  loop {
    let days_in_a_year = if is_leap_year(year) { 366 } else { 365 };
    if day > days_in_a_year {
      day -= days_in_a_year;
      year += 1;
    } else {
      break;
    }
  }

  loop {
    let leap = if is_leap_year(year) { 1 } else { 0 };
    let days_in_a_month = DAYS[leap][month];
    if day > days_in_a_month {
      day -= days_in_a_month;
      month += 1;
    } else {
      break;
    }
  }

  format!("{:04}-{:02}-{:02}", year, month + 1, day)
}

pub fn today() -> String {
  match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(n) => timestep_to_utc(n.as_secs() as i64),
    Err(_) => {
      eprintln!(
        "Could not get today date from the system. Please manually \
        update the date in the changelog."
      );
      "0000-00-00".to_owned()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_lear_year() {
    assert!(is_leap_year(2000));
    assert!(is_leap_year(2012));
    assert!(is_leap_year(2024));
  }

  #[test]
  fn test_timestep_to_utc() {
    assert_eq!(timestep_to_utc(1263081600), "2010-01-10");
    assert_eq!(timestep_to_utc(1298851200), "2011-02-28");
    assert_eq!(timestep_to_utc(1362441600), "2013-03-05");
    assert_eq!(timestep_to_utc(1398816000), "2014-04-30");
    assert_eq!(timestep_to_utc(1430611200), "2015-05-03");
    assert_eq!(timestep_to_utc(1466899200), "2016-06-26");
    assert_eq!(timestep_to_utc(1501459200), "2017-07-31");
    assert_eq!(timestep_to_utc(1535414400), "2018-08-28");
    assert_eq!(timestep_to_utc(1568246400), "2019-09-12");
    assert_eq!(timestep_to_utc(1602633600), "2020-10-14");
    assert_eq!(timestep_to_utc(1635724800), "2021-11-01");
    assert_eq!(timestep_to_utc(1672185600), "2022-12-28");
  }
}
