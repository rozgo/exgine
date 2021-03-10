extern crate exgine;

use exgine::{
    account::{self, fixed_amount    , Tranx},
    asset, hashmap, rate,
};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub enum Resource {
    Battery,
    RgbSensor,
    ThermalSensor,
    PoseEstimation,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub enum Reward {
    Score,
    Token,
    Prediction,
    Currency,
    Policy,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub enum RobotMissionAsset {
    Resource(Resource),
    Reward(Reward),
    MissionTime,
    Trust,
    EnlistCertificate(Instant),
}

impl asset::Asset for RobotMissionAsset {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RobotMissionMarket {
    MissionTimeWithResource,
    MissionTimeWithTrust,
}

type Asset = RobotMissionAsset;
type Market = RobotMissionMarket;
type Rate = rate::Rate<RobotMissionAsset>;
type Account = account::Account<RobotMissionAsset>;

fn mission_default() -> Account {
    Account::from(hashmap![
        Asset::MissionTime => fixed_amount(1000000),
    ])
}

fn agent_default() -> Account {
    Account::from(hashmap![
        Asset::Reward(Reward::Score) => fixed_amount(10000),
        Asset::Reward(Reward::Token) => fixed_amount(10000),
        Asset::Reward(Reward::Prediction) => fixed_amount(10000),
        Asset::Reward(Reward::Policy) => fixed_amount(10000),
    ])
}

fn rates_default() -> HashMap<Market, Rate> {
    hashmap![
        Market::MissionTimeWithResource =>
        Rate {
            credit: hashmap![Asset::MissionTime => fixed_amount(1)],
            debit: hashmap![
                Asset::Reward(Reward::Prediction) => fixed_amount(9),
                Asset::Reward(Reward::Token) => fixed_amount(3),
                Asset::Reward(Reward::Policy) => fixed_amount(1),
            ],
        },
    ]
}

#[test]
fn rate_buy_lifetime() {
    let mission = mission_default();
    let agent = agent_default();
    let rates = rates_default();
    let rate = rates.get(&Market::MissionTimeWithResource).unwrap();

    let res_seller = Account::from(hashmap![
        Asset::MissionTime => fixed_amount(999999),
        Asset::Reward(Reward::Token) => fixed_amount(3),
        Asset::Reward(Reward::Prediction) => fixed_amount(9),
        Asset::Reward(Reward::Policy) => fixed_amount(1),
    ]);

    let res_buyer = Account::from(hashmap![
        Asset::MissionTime => fixed_amount(1),
        Asset::Reward(Reward::Score) => fixed_amount(10000),
        Asset::Reward(Reward::Token) => fixed_amount(9997),
        Asset::Reward(Reward::Prediction) => fixed_amount(9991),
        Asset::Reward(Reward::Policy) => fixed_amount(9999),
    ]);

    match Account::exchange(rate, fixed_amount(1), &agent, &mission) {
        Tranx::Approved(buyer, seller) => {
            assert_eq!(res_seller, seller);
            assert_eq!(res_buyer, buyer);
        }
        _ => assert!(false),
    }
}

#[test]
fn rate_buy_lifetime_quantity() {
    let mission = mission_default();
    let agent = agent_default();
    let rates = rates_default();
    let rate = rates.get(&Market::MissionTimeWithResource).unwrap();

    let res_seller = Account::from(hashmap![
        Asset::MissionTime => fixed_amount(999998),
        Asset::Reward(Reward::Token) => fixed_amount(6),
        Asset::Reward(Reward::Prediction) => fixed_amount(18),
        Asset::Reward(Reward::Policy) => fixed_amount(2),
    ]);

    let res_buyer = Account::from(hashmap![
        Asset::MissionTime => fixed_amount(2),
        Asset::Reward(Reward::Score) => fixed_amount(10000),
        Asset::Reward(Reward::Token) => fixed_amount(9994),
        Asset::Reward(Reward::Prediction) => fixed_amount(9982),
        Asset::Reward(Reward::Policy) => fixed_amount(9998),
    ]);

    match Account::exchange(rate, fixed_amount(2), &agent, &mission) {
        Tranx::Approved(buyer, seller) => {
            assert_eq!(res_seller, seller);
            assert_eq!(res_buyer, buyer);
        }
        _ => assert!(false),
    }
}
