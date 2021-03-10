extern crate exgine;
mod market;

use exgine::{
    account::{self, Quantity, fixed_amount, Tranx},
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
                fixed_amount(1),
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
        Asset::MissionTime => fixed_amount(1000000),
    ])
}

fn agent_default() -> Account {
    Account::from(hashmap![
        Asset::MissionTime => fixed_amount(1),
        Asset::Trust => fixed_amount(10000),
        Asset::EnlistCertificate(Instant::now()) => fixed_amount(1),
        Asset::Resource(Resource::Battery) => fixed_amount(10000),
        Asset::Resource(Resource::RgbSensor) => fixed_amount(10000),
        Asset::Resource(Resource::ThermalSensor) => fixed_amount(10000),
        Asset::Resource(Resource::PoseEstimation) => fixed_amount(10000),
    ])
}

fn rates_default() -> HashMap<Market, Rate> {
    hashmap![
        Market::MissionTimeWithResource =>
        Rate {
            credit: hashmap![Asset::MissionTime => fixed_amount(1)],
            debit: hashmap![
                Asset::Resource(Resource::Battery) => fixed_amount(20),
                Asset::Resource(Resource::ThermalSensor) => fixed_amount(9),
                Asset::Resource(Resource::RgbSensor) => fixed_amount(3),
                Asset::Resource(Resource::PoseEstimation) => fixed_amount(1),
            ],
        },
        Market::MissionTimeWithTrust =>
        Rate {
            credit: hashmap![Asset::MissionTime => fixed_amount(1)],
            debit: hashmap![Asset::Trust => fixed_amount(1)],
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
