use chrono::{DateTime, Local, NaiveDate};

#[test]
fn test_amout_with_tax() {
    let mut today = NaiveDate::from_ymd_opt(1989, 3, 31).unwrap();

    assert_eq!(110, vat_jp::amount_with_tax::<NaiveDate>(100, None));

    assert_eq!(
        110,
        vat_jp::amount_with_tax::<DateTime<Local>>(100, Some(Local::now()))
    );

    assert_eq!(100, vat_jp::amount_with_tax::<NaiveDate>(100, Some(today)));
    assert_eq!(
        -100,
        vat_jp::amount_with_tax::<NaiveDate>(-100, Some(today))
    );
    assert_eq!(
        4200,
        vat_jp::amount_with_tax::<NaiveDate>(4200, Some(today))
    );
    assert_eq!(
        -100,
        vat_jp::amount_with_tax::<NaiveDate>(-100, Some(today))
    );

    today = NaiveDate::from_ymd_opt(1989, 4, 1).unwrap();

    assert_eq!(103, vat_jp::amount_with_tax::<NaiveDate>(100, Some(today)));
    assert_eq!(
        -100,
        vat_jp::amount_with_tax::<NaiveDate>(-100, Some(today))
    );
    assert_eq!(
        4326,
        vat_jp::amount_with_tax::<NaiveDate>(4200, Some(today))
    );

    today = NaiveDate::from_ymd_opt(1997, 3, 31).unwrap();

    assert_eq!(103, vat_jp::amount_with_tax::<NaiveDate>(100, Some(today)));
    assert_eq!(
        -100,
        vat_jp::amount_with_tax::<NaiveDate>(-100, Some(today))
    );
    assert_eq!(
        4326,
        vat_jp::amount_with_tax::<NaiveDate>(4200, Some(today))
    );

    today = NaiveDate::from_ymd_opt(1997, 4, 1).unwrap();

    assert_eq!(105, vat_jp::amount_with_tax::<NaiveDate>(100, Some(today)));
    assert_eq!(
        -100,
        vat_jp::amount_with_tax::<NaiveDate>(-100, Some(today))
    );
    assert_eq!(
        4410,
        vat_jp::amount_with_tax::<NaiveDate>(4200, Some(today))
    );

    today = NaiveDate::from_ymd_opt(2014, 3, 31).unwrap();

    assert_eq!(105, vat_jp::amount_with_tax::<NaiveDate>(100, Some(today)));
    assert_eq!(
        -100,
        vat_jp::amount_with_tax::<NaiveDate>(-100, Some(today))
    );
    assert_eq!(
        4410,
        vat_jp::amount_with_tax::<NaiveDate>(4200, Some(today))
    );

    today = NaiveDate::from_ymd_opt(2014, 4, 1).unwrap();

    assert_eq!(108, vat_jp::amount_with_tax::<NaiveDate>(100, Some(today)));
    assert_eq!(
        -100,
        vat_jp::amount_with_tax::<NaiveDate>(-100, Some(today))
    );
    assert_eq!(
        4536,
        vat_jp::amount_with_tax::<NaiveDate>(4200, Some(today))
    );

    today = NaiveDate::from_ymd_opt(2019, 9, 30).unwrap();

    assert_eq!(108, vat_jp::amount_with_tax::<NaiveDate>(100, Some(today)));
    assert_eq!(
        -100,
        vat_jp::amount_with_tax::<NaiveDate>(-100, Some(today))
    );
    assert_eq!(
        4536,
        vat_jp::amount_with_tax::<NaiveDate>(4200, Some(today))
    );

    today = NaiveDate::from_ymd_opt(2019, 10, 1).unwrap();

    assert_eq!(110, vat_jp::amount_with_tax::<NaiveDate>(100, Some(today)));
    assert_eq!(
        -100,
        vat_jp::amount_with_tax::<NaiveDate>(-100, Some(today))
    );
    assert_eq!(
        4620,
        vat_jp::amount_with_tax::<NaiveDate>(4200, Some(today))
    );
}

#[test]
fn test_amout_with_tax_fraction() {
    #[derive(Debug)]
    struct TestCase {
        input: i64,
        expected: i64,
    }

    // VAT:8%
    let today = NaiveDate::from_ymd_opt(2014, 4, 1).unwrap();

    let test_cases = [
        TestCase {
            input: 0,
            expected: 0,
        },
        TestCase {
            input: 1,
            expected: 1,
        },
        TestCase {
            input: 5,
            expected: 5,
        },
        TestCase {
            input: 6,
            expected: 6,
        },
        TestCase {
            input: 10,
            expected: 10,
        },
        TestCase {
            input: 13,
            expected: 14,
        },
        TestCase {
            input: 20,
            expected: 21,
        },
        TestCase {
            input: 29,
            expected: 31,
        },
    ];
    for tc in test_cases {
        assert_eq!(
            tc.expected,
            vat_jp::amount_with_tax::<NaiveDate>(tc.input, Some(today))
        );
    }
}

#[test]
fn test_vat_builder_amount_with_tax() {
    let mut vat_jp = vat_jp::VatBuilder::<i32, i32>::default()
        .amount(100)
        .build()
        .unwrap();

    assert_eq!(
        vat_jp,
        vat_jp::Vat {
            amount: 100,
            year: None,
            month: None,
            day: None
        }
    );

    vat_jp = vat_jp::VatBuilder::<i32, i32>::default()
        .amount(100)
        .year(1999)
        .build()
        .unwrap();

    assert_eq!(
        vat_jp,
        vat_jp::Vat {
            amount: 100,
            year: Some(1999),
            month: None,
            day: None
        }
    );

    vat_jp = vat_jp::VatBuilder::<i32, i32>::default()
        .amount(100)
        .year(1999)
        .month(12)
        .day(31)
        .build()
        .unwrap();

    assert_eq!(
        vat_jp,
        vat_jp::Vat {
            amount: 100,
            year: Some(1999),
            month: Some(12),
            day: Some(31)
        }
    );
}
