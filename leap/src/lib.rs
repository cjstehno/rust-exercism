
/// Determines whether or not the provided `year` is a "leap year."
///
/// A "leap year" in the Gregorian calendar occurs:
/// * on every year that is evenly divisible by 4
/// * except every year that is evenly divisible by 100
/// * unless the year is also evenly divisible by 400
///
pub fn is_leap_year(year : u16) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 && !(year % 400 == 0) {
            return false;
        } else {
            return true;
        }
    }
    return false;
}
