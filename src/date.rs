// Get current date from the UNIX timestep.
// stripped from https://gist.github.com/sadikovi/b708ed51f479d7b9e8b03515756c6d78

use std::time::SystemTime;

fn is_leap_year(year: i32) -> bool {
  year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub fn today() -> String {
  static MONTHS: [[i64; 12]; 2] = [
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
  ];

  match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(n) => {
      let mut year = 1970;
      let mut month = 0;
      // seconds in a day
      let mut day = n.as_secs() as i64 / 86400;

      loop {
        let days_in_a_year = if is_leap_year(year) { 366 } else { 365 };
        if day >= days_in_a_year {
          day -= days_in_a_year;
          year += 1;
        } else {
          break;
        }
      }

      loop {
        let idx = if is_leap_year(year) { 1 } else { 0 };
        let days_in_a_month = MONTHS[idx][month];
        if day >= days_in_a_month {
          day -= days_in_a_month;
          month += 1;
        } else {
          break;
        }
      }

      format!("{:04}-{:02}-{:02}", year, month + 1, day + 1)
    }
    Err(_) => {
      eprintln!(
        "Could not get today date from the system. Please manually \
        update the date in the changelog."
      );
      "0000-00-00".to_owned()
    }
  }
}
