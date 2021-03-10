extern crate exgine;
extern crate uuid;

use exgine::{account::*, asset, hashmap};
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub enum Fungible {
    Upvote,
    Token,
    Coin,
    Point,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub struct Unique {
    id: Uuid,
    created_at: Instant,
}

impl Default for Unique {
    fn default() -> Self {
        Unique {
            id: Uuid::default(),
            created_at: Instant::now(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub enum NonFungible {
    Coupon(Unique),
    Sticker(Unique),
    Card(Unique),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub enum TestAsset {
    Fungible(Fungible),
    NonFungible(NonFungible),
}

impl asset::Asset for TestAsset {}

type Asset = TestAsset;

#[test]
fn accounts_equal_exisiting_assets() {
    let nft_coupon = NonFungible::Coupon(Unique::default());
    let lhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(500),
        Asset::Fungible(Fungible::Token) => fixed_amount(10000),
        Asset::Fungible(Fungible::Coin) => fixed_amount(800),
        Asset::Fungible(Fungible::Point) => fixed_amount(100),
        Asset::NonFungible(nft_coupon) => fixed_amount(500),
    ]);
    let rhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(500),
        Asset::Fungible(Fungible::Coin) => fixed_amount(800),
        Asset::Fungible(Fungible::Point) => fixed_amount(100),
        Asset::Fungible(Fungible::Token) => fixed_amount(10000),
        Asset::NonFungible(nft_coupon) => fixed_amount(500),
    ]);
    assert_eq!(lhs, rhs);
}

#[test]
fn accounts_equal_missing_assets() {
    let nft_coupon = NonFungible::Coupon(Unique::default());
    let lhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(0),
        Asset::Fungible(Fungible::Token) => fixed_amount(10000),
        Asset::Fungible(Fungible::Point) => fixed_amount(100),
    ]);
    let rhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Coin) => fixed_amount(0),
        Asset::Fungible(Fungible::Point) => fixed_amount(100),
        Asset::Fungible(Fungible::Token) => fixed_amount(10000),
        Asset::NonFungible(nft_coupon) => fixed_amount(0),
    ]);
    assert_eq!(lhs, rhs);
}

#[test]
fn accounts_not_equal_existing() {
    let lhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(500),
        Asset::Fungible(Fungible::Token) => fixed_amount(10000),
        Asset::Fungible(Fungible::Coin) => fixed_amount(800),
        Asset::Fungible(Fungible::Point) => fixed_amount(100),
    ]);
    let rhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(500),
        Asset::Fungible(Fungible::Token) => fixed_amount(10000),
        Asset::Fungible(Fungible::Coin) => fixed_amount(800),
        Asset::Fungible(Fungible::Point) => fixed_amount(10),
    ]);
    assert!(lhs != rhs);

    let nft_coupon_a = NonFungible::Coupon(Unique::default());
    let nft_coupon_b = NonFungible::Coupon(Unique::default());
    let lhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(500),
        Asset::NonFungible(nft_coupon_a) => fixed_amount(1),
    ]);
    let rhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(500),
        Asset::NonFungible(nft_coupon_b) => fixed_amount(1),
    ]);
    assert!(lhs != rhs);
}

#[test]
fn accounts_sub_existing_assets() {
    let lhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(500),
        Asset::Fungible(Fungible::Token) => fixed_amount(10000),
        Asset::Fungible(Fungible::Coin) => fixed_amount(800),
        Asset::Fungible(Fungible::Point) => fixed_amount(100),
    ]);
    let rhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Token) => fixed_amount(10000),
        Asset::Fungible(Fungible::Upvote) => fixed_amount(250),
        Asset::Fungible(Fungible::Point) => fixed_amount(200),
        Asset::Fungible(Fungible::Coin) => fixed_amount(700),
    ]);
    let res = Account::from(hashmap![
        Asset::Fungible(Fungible::Coin) => fixed_amount(100),
        Asset::Fungible(Fungible::Upvote) => fixed_amount(250),
        Asset::Fungible(Fungible::Token) => fixed_amount(0),
        Asset::Fungible(Fungible::Point) => fixed_amount(-100),
    ]);
    assert_eq!(&lhs - &rhs, res);
}

#[test]
fn accounts_sub_missing_assets() {
    let lhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(500),
        Asset::Fungible(Fungible::Coin) => fixed_amount(800),
        Asset::Fungible(Fungible::Point) => fixed_amount(100),
    ]);
    let rhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(250),
        Asset::Fungible(Fungible::Token) => fixed_amount(10000),
        Asset::Fungible(Fungible::Coin) => fixed_amount(700),
    ]);
    let res = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(250),
        Asset::Fungible(Fungible::Token) => fixed_amount(-10000),
        Asset::Fungible(Fungible::Coin) => fixed_amount(100),
        Asset::Fungible(Fungible::Point) => fixed_amount(100),
    ]);
    assert_eq!(&lhs - &rhs, res);
}

#[test]
fn accounts_add_existing_assets() {
    let lhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(250),
        Asset::Fungible(Fungible::Token) => fixed_amount(100),
        Asset::Fungible(Fungible::Coin) => fixed_amount(800),
        Asset::Fungible(Fungible::Point) => fixed_amount(400),
    ]);
    let rhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(250),
        Asset::Fungible(Fungible::Token) => fixed_amount(200),
        Asset::Fungible(Fungible::Point) => fixed_amount(200),
        Asset::Fungible(Fungible::Coin) => fixed_amount(700),
    ]);
    let res = Account::from(hashmap![
        Asset::Fungible(Fungible::Coin) => fixed_amount(1500),
        Asset::Fungible(Fungible::Upvote) => fixed_amount(500),
        Asset::Fungible(Fungible::Token) => fixed_amount(300),
        Asset::Fungible(Fungible::Point) => fixed_amount(600),
    ]);
    assert_eq!(&lhs + &rhs, res);
}

#[test]
fn accounts_add_missing_assets() {
    let nft_coupon_a = NonFungible::Coupon(Unique::default());
    let nft_coupon_b = NonFungible::Coupon(Unique::default());
    let lhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(500),
        Asset::Fungible(Fungible::Coin) => fixed_amount(800),
        Asset::Fungible(Fungible::Point) => fixed_amount(100),
        Asset::NonFungible(nft_coupon_a) => fixed_amount(1),
    ]);
    let rhs = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(250),
        Asset::Fungible(Fungible::Token) => fixed_amount(10000),
        Asset::Fungible(Fungible::Coin) => fixed_amount(700),
        Asset::NonFungible(nft_coupon_b) => fixed_amount(1),
    ]);
    let res = Account::from(hashmap![
        Asset::Fungible(Fungible::Upvote) => fixed_amount(750),
        Asset::Fungible(Fungible::Token) => fixed_amount(10000),
        Asset::Fungible(Fungible::Coin) => fixed_amount(1500),
        Asset::Fungible(Fungible::Point) => fixed_amount(100),
        Asset::NonFungible(nft_coupon_a) => fixed_amount(1),
        Asset::NonFungible(nft_coupon_b) => fixed_amount(1),
    ]);
    assert_eq!(&lhs + &rhs, res);
}
