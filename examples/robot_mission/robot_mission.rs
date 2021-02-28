extern crate exgine;
mod market;

use exgine::{
    account::{self, Quantity, Tranx},
    hashmap, rate,
};
use market::*;
use std::collections::HashMap;
use std::time::Instant;

type Asset = RobotMissionAsset;
type Market = RobotMissionMarket;
type Rate = rate::Rate<RobotMissionAsset>;
type Account = account::Account<RobotMissionAsset>;

pub struct Agent {
    pub account: Account,
    pub is_alive: bool,
}

impl Agent {
    pub fn simulate(&mut self, rates: &HashMap<Market, Rate>, mission: &Account) {
        // Every tick agent should be able to purchase 1 MissionTime.
        // First it tries to purchase MissionTime with its Resource through Market::MissionTimeWithResource.
        // If this fails, it will try to purchase through Market::MissionTimeWithTrust.
        // If agent cannot purchase any more MissionTime it dies.
        let Quantity::Amount(lifetime_before) = self.account.quantity(&Asset::MissionTime);
        let exs = [
            Market::MissionTimeWithResource,
            Market::MissionTimeWithTrust,
        ];
        if let Some(Tranx::Approved(buyer, _)) = exs.iter().find_map(|ex| {
            match Account::exchange(
                rates.get(ex).unwrap(),
                Quantity::Amount(1),
                &self.account,
                mission,
            ) {
                Tranx::Denied(_) => None,
                tranx => Some(tranx),
            }
        }) {
            self.account = buyer;
        }
        let Quantity::Amount(lifetime_after) = self.account.quantity(&Asset::MissionTime);
        if lifetime_after <= lifetime_before {
            self.is_alive = false;
        }
    }
}

fn mission_default() -> Account {
    Account::from(hashmap![
        Asset::MissionTime => Quantity::Amount(1000000),
    ])
}

fn agent_default() -> Account {
    Account::from(hashmap![
        Asset::MissionTime => Quantity::Amount(1),
        Asset::Trust => Quantity::Amount(10000),
        Asset::EnlistCertificate(Instant::now()) => Quantity::Amount(1),
        Asset::Resource(Resource::Battery) => Quantity::Amount(10000),
        Asset::Resource(Resource::RgbSensor) => Quantity::Amount(10000),
        Asset::Resource(Resource::ThermalSensor) => Quantity::Amount(10000),
        Asset::Resource(Resource::PoseEstimation) => Quantity::Amount(10000),
    ])
}

fn rates_default() -> HashMap<Market, Rate> {
    hashmap![
        Market::MissionTimeWithResource =>
        Rate {
            credit: hashmap![Asset::MissionTime => Quantity::Amount(1)],
            debit: hashmap![
                Asset::Resource(Resource::Battery) => Quantity::Amount(20),
                Asset::Resource(Resource::ThermalSensor) => Quantity::Amount(9),
                Asset::Resource(Resource::RgbSensor) => Quantity::Amount(3),
                Asset::Resource(Resource::PoseEstimation) => Quantity::Amount(1),
            ],
        },
        Market::MissionTimeWithTrust =>
        Rate {
            credit: hashmap![Asset::MissionTime => Quantity::Amount(1)],
            debit: hashmap![Asset::Trust => Quantity::Amount(1)],
        },
    ]
}

fn main() {
    let mission = mission_default();
    let rates = rates_default();

    let mut agent = Agent {
        account: agent_default(),
        is_alive: true,
    };

    while agent.is_alive {
        agent.simulate(&rates, &mission);
    }

    let Quantity::Amount(total_secs) = agent.account.quantity(&Asset::MissionTime);

    let hours = (total_secs / 60) / 60;
    let mins = (total_secs - hours * 60 * 60) / 60;
    let secs = total_secs - (hours * 60 * 60 + mins * 60);

    println!("{:#?}", agent.account);
    println!(
        "RIP! Agent was alive for {} hours, {} minutes and {} seconds.",
        hours, mins, secs
    );
}
