pub mod extend_trace;
pub mod gpu_common;
pub mod qm31;
pub mod utils;

use serde::{Deserialize, Serialize};

use super::{Backend, BackendForChannel};
use crate::core::vcs::blake2_merkle::Blake2sMerkleChannel;

// pub mod accumulation;
// pub mod bit_reverse;
// pub mod blake2s;
// pub mod circle;
// pub mod cm31;
// pub mod column;
// pub mod conversion;
// pub mod domain;
// pub mod fft;
// pub mod fri;
// mod grind;
// pub mod lookups;
// pub mod m31;
// #[cfg(not(target_arch = "wasm32"))]
// pub mod poseidon252;
// pub mod prefix_sum;
// pub mod qm31;
// pub mod quotients;
// mod utils;
// pub mod very_packed_m31;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct WgpuBackend {
    inner: SimdBackend,
}

impl Backend for WgpuBackend {}
impl BackendForChannel<Blake2sMerkleChannel> for WgpuBackend {}
