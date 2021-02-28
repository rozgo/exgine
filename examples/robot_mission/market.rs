use exgine::asset;
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
