pub fn is_leap_year(year: i32) -> bool {

  // on every year that is evenly divisible by 4
  //   except every year that is evenly divisible by 100
  //     unless the year is also evenly divisible by 400
  (divisible_by_four_hundred(year)) || (divisible_by_four(year) && !divisible_by_hundred(year))
}

fn divisible_by_four(year: i32) -> bool {
    year % 4 == 0
}
fn divisible_by_hundred(year: i32) -> bool {
    year % 100 == 0
}
fn divisible_by_four_hundred(year: i32) -> bool {
    year % 400 == 0
}
