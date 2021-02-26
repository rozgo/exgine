use account::*;
use asset::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Rate<TAsset: Asset> {
    pub credit: HashMap<TAsset, Quantity>,
    pub debit: HashMap<TAsset, Quantity>,
}
