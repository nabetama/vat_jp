use std::time::SystemTime;

use vat_jp;

#[test]
fn test_vat_jp() {
    assert_eq!(110, vat_jp::amount_with_tax(100));
}
