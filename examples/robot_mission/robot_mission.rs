extern crate exgine;

mod market;

use account::*;
use exgine::*;
use market::*;
use rate::*;
use std::collections::HashMap;
use std::time::Instant;

pub struct Agent {
    pub account: Account<RobotMissionAsset>,
    pub is_alive: bool,
}

type Asset = RobotMissionAsset;
type Market = RobotMissionMarket;

impl Agent {
    pub fn simulate(&mut self, rates: &HashMap<Market, Rate<Asset>>, mission: &Account<Asset>) {
        // Every tick agent should be able to purchase 1 MissionTime.
        // First it tries to purchase MissionTime with its Resource through Market::MissionTimeWithResource.
        // If this fails, it will try to purchase through Market::MissionTimeWithTrust.
        // If agent cannot purchase any more MissionTime it dies.
        let Quantity(lifetime_before) = self.account.quantity(&Asset::MissionTime);
        let exs = [
            Market::MissionTimeWithResource,
            Market::MissionTimeWithTrust,
        ];
        if let Some(Tranx::Approved(buyer, _)) = exs.iter().find_map(|ex| {
            match Account::exchange(rates.get(ex).unwrap(), Quantity(1), &self.account, mission) {
                Tranx::Denied(_) => None,
                tranx => Some(tranx),
            }
        }) {
            self.account = buyer;
        }
        let Quantity(lifetime_after) = self.account.quantity(&Asset::MissionTime);
        if lifetime_after <= lifetime_before {
            self.is_alive = false;
        }
    }
}

fn mission_default() -> Account<Asset> {
    Account(hashmap![
        Asset::MissionTime => Quantity(1000000),
    ])
}

fn agent_default() -> Account<Asset> {
    Account(hashmap![
        Asset::MissionTime => Quantity(1),
        Asset::Trust => Quantity(10000),
        Asset::EnlistCertificate(Instant::now()) => Quantity(1),
        Asset::Resource(Resource::Battery) => Quantity(10000),
        Asset::Resource(Resource::RgbSensor) => Quantity(10000),
        Asset::Resource(Resource::ThermalSensor) => Quantity(10000),
        Asset::Resource(Resource::PoseEstimation) => Quantity(10000),
    ])
}

fn rates_default() -> HashMap<Market, Rate<Asset>> {
    hashmap![
        Market::MissionTimeWithResource =>
        Rate {
            credit: hashmap![Asset::MissionTime => Quantity(1)],
            debit: hashmap![
                Asset::Resource(Resource::Battery) => Quantity(20),
                Asset::Resource(Resource::ThermalSensor) => Quantity(9),
                Asset::Resource(Resource::RgbSensor) => Quantity(3),
                Asset::Resource(Resource::PoseEstimation) => Quantity(1),
            ],
        },
        Market::MissionTimeWithTrust =>
        Rate {
            credit: hashmap![Asset::MissionTime => Quantity(1)],
            debit: hashmap![Asset::Trust => Quantity(1)],
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

    let Quantity(total_secs) = agent.account.quantity(&Asset::MissionTime);

    let hours = (total_secs / 60) / 60;
    let mins = (total_secs - hours * 60 * 60) / 60;
    let secs = total_secs - (hours * 60 * 60 + mins * 60);

    println!("{:#?}", agent.account);
    println!(
        "RIP! Agent was alive for {} hours, {} minutes and {} seconds.",
        hours, mins, secs
    );
}
