//! Test vectors are from NESSIE:
//! https://www.cosic.esat.kuleuven.be/nessie/testvectors/
#![no_std]
extern crate des;
#[macro_use]
extern crate block_cipher_trait;

new_test!(des_test, "des", des::Des);
new_test!(tdes_ede3_test, "tdes", des::TdesEde3);
new_test!(tdes_ede2_test, "tdes2", des::TdesEde2);
