use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn bench_ark_xor() {
    wasmarking::prover::bench_ark("xor");
}

#[wasm_bindgen_test]
fn bench_ark_withdraw() {
    wasmarking::prover::bench_ark("withdraw");
}

#[wasm_bindgen_test]
fn bench_jf_withdraw() {
    wasmarking::prover::bench_jf_withdraw();
}
