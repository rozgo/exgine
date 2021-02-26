extern crate exgine;

use account::*;
use asset;
use exgine::*;
use rate::*;
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

fn mission_default() -> Account<Asset> {
    Account(hashmap![
        Asset::MissionTime => Quantity(1000000),
    ])
}

fn agent_default() -> Account<Asset> {
    Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(10000),
        Asset::Reward(Reward::Token) => Quantity(10000),
        Asset::Reward(Reward::Prediction) => Quantity(10000),
        Asset::Reward(Reward::Policy) => Quantity(10000),
    ])
}

fn rates_default() -> HashMap<Market, Rate<Asset>> {
    hashmap![
        Market::MissionTimeWithResource =>
        Rate {
            credit: hashmap![Asset::MissionTime => Quantity(1)],
            debit: hashmap![
                Asset::Reward(Reward::Prediction) => Quantity(9),
                Asset::Reward(Reward::Token) => Quantity(3),
                Asset::Reward(Reward::Policy) => Quantity(1),
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

    let res_seller = Account(hashmap![
        Asset::MissionTime => Quantity(999999),
        Asset::Reward(Reward::Token) => Quantity(3),
        Asset::Reward(Reward::Prediction) => Quantity(9),
        Asset::Reward(Reward::Policy) => Quantity(1),
    ]);

    let res_buyer = Account(hashmap![
        Asset::MissionTime => Quantity(1),
        Asset::Reward(Reward::Score) => Quantity(10000),
        Asset::Reward(Reward::Token) => Quantity(9997),
        Asset::Reward(Reward::Prediction) => Quantity(9991),
        Asset::Reward(Reward::Policy) => Quantity(9999),
    ]);

    match Account::exchange(rate, Quantity(1), &agent, &mission) {
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

    let res_seller = Account(hashmap![
        Asset::MissionTime => Quantity(999998),
        Asset::Reward(Reward::Token) => Quantity(6),
        Asset::Reward(Reward::Prediction) => Quantity(18),
        Asset::Reward(Reward::Policy) => Quantity(2),
    ]);

    let res_buyer = Account(hashmap![
        Asset::MissionTime => Quantity(2),
        Asset::Reward(Reward::Score) => Quantity(10000),
        Asset::Reward(Reward::Token) => Quantity(9994),
        Asset::Reward(Reward::Prediction) => Quantity(9982),
        Asset::Reward(Reward::Policy) => Quantity(9998),
    ]);

    match Account::exchange(rate, Quantity(2), &agent, &mission) {
        Tranx::Approved(buyer, seller) => {
            assert_eq!(res_seller, seller);
            assert_eq!(res_buyer, buyer);
        }
        _ => assert!(false),
    }
}
