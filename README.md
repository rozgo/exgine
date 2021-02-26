# Market Exchange Simulator
A category theory inspired smart contract, where objects are assets and morphisms are rates, creating a mathematical formalization for an exchange (assets, accounts, rates and transactions).

Core component for [OwnedEconomies](https://github.com/rozgo/OwnedEconomies)


## Requirements:
- Install Rust [https://rustup.rs](https://rustup.rs)
```
$ curl https://sh.rustup.rs -sSf | sh
```

## Run tests:
```
cargo test -- --nocapture
```
- In every tick agent should be able to purchase 1 MissionTime.
- First it tries to purchase MissionTime with its Resource through Exchange::MissionTimeWithResource.
- If this fails, it will try to purchase through Exchange::MissionTimeWithTrust.
- If agent cannot purchase any more MissionTime it dies.

If everything worked you should see test results like this:
```
running 1 test
RIP! Agent was alive for 2 hours, 55 minutes and 1 seconds.
test agent_lifetime_until_death ... ok
```

## Run examples:
```
cargo run --example robot_mission
```