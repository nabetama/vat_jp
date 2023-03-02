use chrono::NaiveDate;
use vat_jp;

#[test]
fn test_amout_with_tax() {
    let mut today = NaiveDate::from_ymd_opt(1989, 3, 31).unwrap();

    assert_eq!(100, vat_jp::amount_with_tax::<NaiveDate>(100, None));

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
