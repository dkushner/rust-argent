extern crate argent;

use argent::{Currency, Amount};

#[test]
fn test_display() {
    let currency = Currency::simple("$");
    let amount = Amount::of(&currency, 10000);
    let display = format!("{}", amount);
    assert_eq!("$100.00", display);
}

#[test]
fn test_custom_currency() {
    let currency = Currency {
        iso_code: Some("USD".to_string()),
        iso_numeric: Some(840),
        name: Some("United States Dollaredoo".to_string()),
        symbol: "$".to_string(),
        minor_ratio: 1000,
        symbol_first: true,
        decimal_mark: '.',
        group_mark: ','
    };
    let amount = Amount::of(&currency, 100000);
    let display = format!("{}", amount);
    assert_eq!("$100.000", display);
}

#[test]
fn test_grouping() {
    let currency = Currency::simple("$");
    let amount = Amount::of(&currency, 1000000000);
    let display = format!("{}", amount);
    assert_eq!("$10,000,000.00", display);
}

#[test]
fn test_addition() {
    let currency = Currency::simple("$");
    let first = Amount::of(&currency, 545);
    let second = Amount::of(&currency, 455);
    let result = first + second;
    assert_eq!(result.quantity, 1000);
}

#[test]
fn test_moot_addition() {
    let currency_a = Currency::simple("$");
    let currency_b = Currency::simple("&");
    let first = Amount::of(&currency_a, 545);
    let second = Amount::of(&currency_b, 455);
    let result = first + second;
    assert_eq!(result.quantity, 545);
}

#[test]
fn test_subtraction() {
    let currency = Currency::simple("$");
    let first = Amount::of(&currency, 545);
    let second = Amount::of(&currency, 455);
    let result = first - second;
    assert_eq!(result.quantity, 90);
}

#[test]
fn test_moot_subtraction() {
    let currency_a = Currency::simple("$");
    let currency_b = Currency::simple("&");
    let first = Amount::of(&currency_a, 545);
    let second = Amount::of(&currency_b, 455);
    let result = first - second;
    assert_eq!(result.quantity, 545);
}




