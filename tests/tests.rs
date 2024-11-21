use saturating_add_assign::saturating_add_assign;

#[test]
fn test_saturating_add_assign() {
    let mut i = 0u64;
    let v = 1;
    saturating_add_assign!(i, v);
    assert_eq!(i, 1);

    i = u64::MAX;
    saturating_add_assign!(i, v);
    assert_eq!(i, u64::MAX);
}
