/// Utilities for managing things like spacing

/// Approximate the numerator of a rational number p/q equal to frac with a given base
pub fn approximate(frac: f64, base: f64) -> i64{
    (frac*base).floor() as i64
}

/// For the remainder of a set of widths in approximate. Evenly space out components in that case.
/// Return is of the form (interval, remainder)
pub fn even_pad(intervals: Vec<i64>, total: i64) -> (i64, i64){
    let difference = total-intervals.iter().sum::<i64>();
    let spacing =  difference / intervals.len() as i64;
    let remainder = difference % intervals.len() as i64;
    (spacing, remainder)
}

pub fn term_width() -> i64 {
    match termsize::get() {
        Some(size) => size.rows.into(),
        None => 80,
    }
}

pub fn term_height() -> i64 {
    match termsize::get() {
        Some(size) => size.cols.into(),
        None => 200,
    }
}
