use chrono::{Datelike, NaiveDate};

const RATE100: f64 = 1.0_f64;
const RATE103: f64 = 1.03_f64;
const RATE105: f64 = 1.05_f64;
const RATE108: f64 = 1.08_f64;
const RATE110: f64 = 1.1_f64;

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
            return res as i64;
        }
        None => return amount,
    }
}

fn get_rate<T>(date: T) -> f64
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
