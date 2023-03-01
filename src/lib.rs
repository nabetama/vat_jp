use std::time::SystemTime;

// const RATE100: f32 = 1.0 / 1.0;
// const RATE103: f32 = 1.03;
// const RATE105: f32 = 1.05;
// const RATE108: f32 = 1.08;
const RATE110: f32 = 1.1_f32;

pub fn amount_with_tax(amount: i64) -> i64 {
    if amount < 0 {
        return amount;
    }

    let res = RATE110 * amount as f32;
    res as i64
}
