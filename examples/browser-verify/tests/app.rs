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

use risc0_zkvm::SessionReceipt;
use risc0_zkvm_receipts::receipts::{FIB_ID, FIB_RECEIPT};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

// This runs a unit test in the browser, so it can use browser APIs.
#[wasm_bindgen_test]
fn test_verify() {
    let receipt: SessionReceipt = bincode::deserialize(FIB_RECEIPT).unwrap();
    receipt.verify(FIB_ID).unwrap();
}
