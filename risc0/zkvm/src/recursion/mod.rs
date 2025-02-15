// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module implements the verifier for the recursion circuit.
//!
//! This module implements receipts that are generated from the recursion
//! circuit as well as verification functions for each type of receipt.
#[cfg(not(target_os = "zkvm"))]
use risc0_zkp::adapter::{CircuitCoreDef, TapsProvider};
#[cfg(not(target_os = "zkvm"))]
mod circuit_impl;
mod control_id;
mod info;
mod poly_ext;
mod receipt;
mod taps;

pub use poly_ext::DEF;
pub use receipt::{valid_control_ids, RollupReceipt};
pub use taps::TAPSET;

/// This struct implements traits that are defined by code generated by the
/// circuit definition.
pub struct CircuitImpl;
