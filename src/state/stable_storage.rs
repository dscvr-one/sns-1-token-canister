use super::State;
use crate::prelude::*;
use crate::service_controller::ServiceControllers;
use crate::soul_bound_nft::SoulBoundNFT;
use ic_cdk::api::stable;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, CandidType)]
pub struct StableStorage {
    pub(crate) nft_img: Vec<u8>,
    pub(crate) nft_index: u64,
    pub(crate) soul_bound_nfts: HashMap<Principal, SoulBoundNFT>,
    pub(crate) controllers: ServiceControllers,
}

impl From<&mut State> for StableStorage {
    fn from(state: &mut State) -> Self {
        Self {
            nft_img: std::mem::take(&mut state.nft_img),
            nft_index: std::mem::take(&mut state.nft_index),
            soul_bound_nfts: std::mem::take(&mut state.soul_bound_nfts),
            controllers: std::mem::take(&mut state.controllers),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, CandidType)]
pub struct StableStorageChunk<T> {
    pub chunk: Vec<T>,
    pub start: Option<usize>,
}

pub(crate) fn stable_save<T>(t: T) -> Result<(), rmp_serde::encode::Error>
where
    T: serde::Serialize,
{
    let mut storage = stable::StableWriter::default();
    rmp_serde::encode::write(&mut storage, &t)?;
    rmp_serde::encode::write(&mut storage, &ic_cdk::api::instruction_counter())
}

pub(crate) fn stable_restore<T1, T2>() -> (
    Result<T1, rmp_serde::decode::Error>,
    Result<T2, rmp_serde::decode::Error>,
)
where
    T1: for<'de> serde::Deserialize<'de>,
    T2: for<'de> serde::Deserialize<'de>,
{
    let mut reader = stable::StableReader::default();
    let t1 = rmp_serde::decode::from_read(&mut reader);
    let t2 = rmp_serde::decode::from_read(&mut reader);
    (t1, t2)
}
