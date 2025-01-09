// File: lib.rs
//! ZeroML - Verifiable Agent Inference System
//! Implementation of verifiable agent outputs using zero-knowledge proofs
//! Copyright (c) 2025 ZeroML

use ark_ff::fields::{Field, PrimeField};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem, SynthesisError};
use ark_snark::SNARK;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

pub mod types {
    use super::*;
    
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct AgentInference {
        pub input: Vec<u8>,
        pub output: Vec<u8>,
        pub timestamp: u64,
        pub agent_id: String,
    }

    #[derive(Clone, Debug)]
    pub struct VerificationCircuit<F: Field> {
        pub input: Option<F>,
        pub output: Option<F>,
        pub salt: Option<F>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Proof {
        pub pi_a: Vec<String>,
        pub pi_b: Vec<Vec<String>>,
        pub pi_c: Vec<String>,
    }
}