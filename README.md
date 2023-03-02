# vat_jp

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/nabetama/vat_jp/build-test.yml?branch=main)](https://github.com/nabetama/vat_jp/actions)
[![GitHub](https://img.shields.io/github/license/nabetama/vat_jp)](https://github.com/nabetama/vat_jp/blob/main/LICENSE)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/m/nabetama/vat_jp)](https://github.com/nabetama/vat_jp/pulse)
[![GitHub last commit](https://img.shields.io/github/last-commit/nabetama/vat_jp)](https://github.com/nabetama/vat_jp/commits/main)

with vat_jp calculates consumption tax (incl. sales tax, VAT, excise duty, etc.) in Japan.

## Usage

With vat_jp, you can know the price including VAT and the VAT rate at any point in time.

```rs
use chrono::NaiveDate;
use vat_jp;

// To find out the current price including tax, pass the amount as the first argument and `None` as the second argument.
assert_eq!(110, vat_jp::amount_with_tax::<NaiveDate>(100, None));

// When there was no VAT...
let mut today = NaiveDate::from_ymd_opt(1989, 3, 31).unwrap();
assert_eq!(100, vat_jp::amount_with_tax::<NaiveDate>(100, Some(today)));

// VAT rate
assert_eq!(1.1, vat_jp::get_rate(today));
```
