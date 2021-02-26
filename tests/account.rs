extern crate exgine;

use account::*;
use asset;
use exgine::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub enum Stuff {
    Coupon,
    Token,
    Prediction,
    Policy,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub enum TestAsset {
    Something(Stuff),
}

impl asset::Asset for TestAsset {}

type Asset = TestAsset;

#[test]
fn accounts_equal_exisiting_assets() {
    let lhs = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(500),
        Asset::Something(Stuff::Token) => Quantity(10000),
        Asset::Something(Stuff::Prediction) => Quantity(800),
        Asset::Something(Stuff::Policy) => Quantity(100),
    ]);
    let rhs = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(500),
        Asset::Something(Stuff::Prediction) => Quantity(800),
        Asset::Something(Stuff::Policy) => Quantity(100),
        Asset::Something(Stuff::Token) => Quantity(10000),
    ]);
    assert_eq!(lhs, rhs);
}

#[test]
fn accounts_equal_missing_assets() {
    let lhs = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(0),
        Asset::Something(Stuff::Token) => Quantity(10000),
        Asset::Something(Stuff::Policy) => Quantity(100),
    ]);
    let rhs = Account(hashmap![
        Asset::Something(Stuff::Prediction) => Quantity(0),
        Asset::Something(Stuff::Policy) => Quantity(100),
        Asset::Something(Stuff::Token) => Quantity(10000),
    ]);
    assert_eq!(lhs, rhs);
}

#[test]
fn accounts_not_equal_existing() {
    let lhs = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(500),
        Asset::Something(Stuff::Token) => Quantity(10000),
        Asset::Something(Stuff::Prediction) => Quantity(800),
        Asset::Something(Stuff::Policy) => Quantity(100),
    ]);
    let rhs = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(500),
        Asset::Something(Stuff::Token) => Quantity(10000),
        Asset::Something(Stuff::Prediction) => Quantity(800),
        Asset::Something(Stuff::Policy) => Quantity(10),
    ]);
    assert!(lhs != rhs);
}

#[test]
fn accounts_sub_existing_assets() {
    let lhs = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(500),
        Asset::Something(Stuff::Token) => Quantity(10000),
        Asset::Something(Stuff::Prediction) => Quantity(800),
        Asset::Something(Stuff::Policy) => Quantity(100),
    ]);
    let rhs = Account(hashmap![
        Asset::Something(Stuff::Token) => Quantity(10000),
        Asset::Something(Stuff::Coupon) => Quantity(250),
        Asset::Something(Stuff::Policy) => Quantity(200),
        Asset::Something(Stuff::Prediction) => Quantity(700),
    ]);
    let res = Account(hashmap![
        Asset::Something(Stuff::Prediction) => Quantity(100),
        Asset::Something(Stuff::Coupon) => Quantity(250),
        Asset::Something(Stuff::Token) => Quantity(0),
        Asset::Something(Stuff::Policy) => Quantity(-100),
    ]);
    assert_eq!(&lhs - &rhs, res);
}

#[test]
fn accounts_sub_missing_assets() {
    let lhs = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(500),
        Asset::Something(Stuff::Prediction) => Quantity(800),
        Asset::Something(Stuff::Policy) => Quantity(100),
    ]);
    let rhs = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(250),
        Asset::Something(Stuff::Token) => Quantity(10000),
        Asset::Something(Stuff::Prediction) => Quantity(700),
    ]);
    let res = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(250),
        Asset::Something(Stuff::Token) => Quantity(-10000),
        Asset::Something(Stuff::Prediction) => Quantity(100),
        Asset::Something(Stuff::Policy) => Quantity(100),
    ]);
    assert_eq!(&lhs - &rhs, res);
}

#[test]
fn accounts_add_existing_assets() {
    let lhs = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(250),
        Asset::Something(Stuff::Token) => Quantity(100),
        Asset::Something(Stuff::Prediction) => Quantity(800),
        Asset::Something(Stuff::Policy) => Quantity(400),
    ]);
    let rhs = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(250),
        Asset::Something(Stuff::Token) => Quantity(200),
        Asset::Something(Stuff::Policy) => Quantity(200),
        Asset::Something(Stuff::Prediction) => Quantity(700),
    ]);
    let res = Account(hashmap![
        Asset::Something(Stuff::Prediction) => Quantity(1500),
        Asset::Something(Stuff::Coupon) => Quantity(500),
        Asset::Something(Stuff::Token) => Quantity(300),
        Asset::Something(Stuff::Policy) => Quantity(600),
    ]);
    assert_eq!(&lhs + &rhs, res);
}

#[test]
fn accounts_add_missing_assets() {
    let lhs = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(500),
        Asset::Something(Stuff::Prediction) => Quantity(800),
        Asset::Something(Stuff::Policy) => Quantity(100),
    ]);
    let rhs = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(250),
        Asset::Something(Stuff::Token) => Quantity(10000),
        Asset::Something(Stuff::Prediction) => Quantity(700),
    ]);
    let res = Account(hashmap![
        Asset::Something(Stuff::Coupon) => Quantity(750),
        Asset::Something(Stuff::Token) => Quantity(10000),
        Asset::Something(Stuff::Prediction) => Quantity(1500),
        Asset::Something(Stuff::Policy) => Quantity(100),
    ]);
    assert_eq!(&lhs + &rhs, res);
}
