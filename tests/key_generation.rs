use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn ark_bench_xor() {
    wasmarking::key_generation::ark_bench("xor");
}

#[wasm_bindgen_test]
fn ark_bench_withdraw() {
    wasmarking::key_generation::ark_bench("withdraw");
}

#[wasm_bindgen_test]
fn jf_bench_withdraw() {
    wasmarking::key_generation::jf_bench_withdraw();
}
