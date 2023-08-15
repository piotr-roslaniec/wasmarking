use instant::Instant;
use web_sys::console;
use wasm_bindgen::prelude::*;

use crate::{ArkRelation, JfRelation};

static REPEAT: usize = 3;

pub fn jf_bench(operation: &str, job: impl Fn()) {
    let start = Instant::now();
    for _ in 0..REPEAT {
        job()
    }
    let elapsed = Instant::now().duration_since(start);

    console::log_1(
        &format!(
            "{:?} key generation. Relation performance: {:?}",
            operation,
            (elapsed / REPEAT as u32)
        )
        .into(),
    );
}

pub mod key_generation {
    use super::*;

    #[wasm_bindgen]
    pub fn ark_bench(relation_name: &str) {
        let relation = ArkRelation::from(relation_name);

        let start = Instant::now();
        for _ in 0..REPEAT {
            relation.generate_keys();
        }
        let elapsed = Instant::now().duration_since(start);

        console::log_1(
            &format!(
                "{:?} key generation. Relation performance: {:?}",
                relation_name,
                (elapsed / REPEAT as u32)
            )
            .into(),
        );
    }

    #[wasm_bindgen]
    pub fn jf_bench_withdraw() {
        let relation = JfRelation::Withdraw;

        let job = || relation.generate_circuit();
        jf_bench("circuit generation", job);

        let job = || {
            let _ = relation.generate_srs();
        };
        jf_bench("srs generation", job);

        let srs = relation.generate_srs();

        let job = || {
            let _ = relation.generate_keys(&srs);
        };
        jf_bench("keys generation", job);
    }
}

pub mod prover {
    use super::*;

    #[wasm_bindgen]
    pub fn bench_ark(relation_name: &str) {
        let relation = ArkRelation::from(relation_name);
        let (pk, _) = relation.generate_keys();

        let start = Instant::now();
        for _ in 0..REPEAT {
            relation.generate_proof(pk.clone());
        }
        let elapsed = Instant::now().duration_since(start);

        console::log_1(
            &format!(
                "{:?} prover. Relation performance: {:?}",
                relation_name,
                (elapsed / REPEAT as u32)
            )
            .into(),
        );
    }

    #[wasm_bindgen]
    pub fn bench_jf_withdraw() {
        let relation = JfRelation::Withdraw;
        let srs = relation.generate_srs();
        let (pk, _) = relation.generate_keys(&srs);

        let start = Instant::now();
        for _ in 0..REPEAT {
            relation.generate_proof(pk.clone());
        }
        let elapsed = Instant::now().duration_since(start);

        console::log_1(
            &format!(
                "{:?} prover. Relation performance: {:?}",
                "jf_withdraw",
                (elapsed / REPEAT as u32)
            )
            .into(),
        );
    }
}
