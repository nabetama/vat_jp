//! # vat_jp
//!
//! vat_jp calculates consumption tax (incl. sales tax, VAT, excise duty, etc.) in Japan.
//!
//! ## Usage
//!
//! With vat_jp, you can know the price including VAT and the VAT rate at any point in time.
//!
//! ```
//! use chrono::{DateTime, Local, NaiveDate};
//! use vat_jp;
//!
//! // To find out the current price including tax,
//! // pass the amount as the first argument and `None` as the second argument.
//! assert_eq!(110, vat_jp::amount_with_tax::<NaiveDate>(100, None));
//! assert_eq!(
//!     110,
//!     vat_jp::amount_with_tax::<DateTime<Local>>(100, Some(Local::now()))
//! );
//! // When there was no VAT...
//! let mut today = NaiveDate::from_ymd_opt(1989, 3, 31).unwrap();
//! assert_eq!(100, vat_jp::amount_with_tax::<NaiveDate>(100, Some(today)));
//!
//! // VAT rate
//! assert_eq!(1.1, vat_jp::get_rate::<DateTime<Local>>(Local::now()));
//! ```
//!
use chrono::{DateTime, Datelike, Local, NaiveDate};

const RATE100: f64 = 1.0_f64;
const RATE103: f64 = 1.03_f64;
const RATE105: f64 = 1.05_f64;
const RATE108: f64 = 1.08_f64;
const RATE110: f64 = 1.1_f64;

/// returns amount with tax
///
/// ## Example
///
/// ```
/// use chrono::{DateTime, Local, NaiveDate};
///
/// assert_eq!(110, vat_jp::amount_with_tax::<NaiveDate>(100, None));
/// assert_eq!(
///     110,
///     vat_jp::amount_with_tax::<DateTime<Local>>(100, Some(Local::now()))
///     );
///
/// let mut today = NaiveDate::from_ymd_opt(1989, 3, 31).unwrap();  // 0% VAT
/// assert_eq!(100, vat_jp::amount_with_tax::<NaiveDate>(100, Some(today)));
///
/// ```
pub fn amount_with_tax<T>(amount: i64, today: Option<T>) -> i64
where
    T: Datelike,
{
    if amount < 0 {
        return amount;
    }

    match today {
        Some(date) => {
            let res = get_rate::<T>(date) * amount as f64;
            res as i64
        }
        None => {
            let res = get_rate::<DateTime<Local>>(Local::now()) * amount as f64;
            res as i64
        }
    }
}

/// returns amout without tax
///
/// **Warning**
/// fractions of division are rounded up.
///
/// ## Example
///
/// ```
/// use chrono::{DateTime, Local};
///
///
/// fn test_amount_without_tax() {
///     assert_eq!(13637, vat_jp::amount_without_tax::<DateTime<Local>>(15000, None));
/// }
/// ```
pub fn amount_without_tax<T>(amount_with_tax: i64, today: Option<T>) -> i64
where
    T: Datelike,
{
    let tax_rate = match today {
        Some(date) => get_rate(date),
        None => get_rate(Local::now()),
    };

    let amount_without_tax = amount_with_tax as f64 / tax_rate;
    amount_without_tax.ceil() as i64 // fractions of division are rounded up
}

/// returns VAT rate at any point in time
///
/// ## Example
///
/// ```
/// use chrono::NaiveDate;
///
/// let mut today = NaiveDate::from_ymd_opt(1989, 3, 31).unwrap();
/// assert_eq!(1.0, vat_jp::get_rate(today));
/// ```
pub fn get_rate<T>(date: T) -> f64
where
    T: Datelike,
{
    let days = date.num_days_from_ce();

    if nd_from_ce(1989, 4, 1) <= days && days <= nd_from_ce(1997, 3, 31) {
        return RATE103;
    }
    if nd_from_ce(1997, 4, 1) <= days && days <= nd_from_ce(2014, 3, 31) {
        return RATE105;
    }
    if nd_from_ce(2014, 4, 1) <= days && days <= nd_from_ce(2019, 9, 30) {
        return RATE108;
    }
    if nd_from_ce(2019, 10, 1) <= days {
        return RATE110;
    }

    RATE100
}

fn nd_from_ce(y: i32, m: u32, d: u32) -> i32 {
    let days = NaiveDate::from_ymd_opt(y, m, d).unwrap();
    days.num_days_from_ce()
}
#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_rate() {
        let mut today = NaiveDate::from_ymd_opt(1989, 3, 31).unwrap();
        assert_eq!(RATE100, get_rate(today));

        today = NaiveDate::from_ymd_opt(1989, 4, 1).unwrap();
        assert_eq!(RATE103, get_rate(today));

        today = NaiveDate::from_ymd_opt(1997, 3, 31).unwrap();
        assert_eq!(RATE103, get_rate(today));

        today = NaiveDate::from_ymd_opt(1997, 4, 1).unwrap();
        assert_eq!(RATE105, get_rate(today));

        today = NaiveDate::from_ymd_opt(2014, 3, 31).unwrap();
        assert_eq!(RATE105, get_rate(today));

        today = NaiveDate::from_ymd_opt(2014, 4, 1).unwrap();
        assert_eq!(RATE108, get_rate(today));

        today = NaiveDate::from_ymd_opt(2019, 9, 30).unwrap();
        assert_eq!(RATE108, get_rate(today));

        today = NaiveDate::from_ymd_opt(2019, 10, 1).unwrap();
        assert_eq!(RATE110, get_rate(today));
    }

    #[test]
    fn test_nd_from_ce() {
        assert_eq!(-365, nd_from_ce(0, 1, 1));
        assert_eq!(1, nd_from_ce(1, 1, 1));
        assert_eq!(366, nd_from_ce(2, 1, 1));
        assert_eq!(719_163, nd_from_ce(1970, 1, 1));
    }

    #[test]
    fn test_amount_without_tax() {
        assert_eq!(13637, amount_without_tax::<DateTime<Local>>(15000, None));
    }
}
