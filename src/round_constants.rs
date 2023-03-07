// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! This module is designed to load from `ark.bin` the 960
//! constants used as `round_constants`.
//!
//! The constants were originally computed using:
//! https://extgit.iaik.tugraz.at/krypto/hadesmimc/blob/master/code/calc_round_numbers.py
//! and then mapped onto `Scalar` in the Bls12_381 scalar field.
#![allow(non_snake_case)]

use crate::u64_from_buffer;
use bls12_381::Scalar;

const CONSTANTS: usize = 960;

/// `ROUND_CONSTANTS` constists on a static reference
/// that points to the pre-loaded 960 Fq constants.
///
/// This 960 `Scalar` constants are loaded from `ark.bin`
/// where all of the `Scalar`s are represented in buf.
///
/// This round constants have been taken from:
/// https://extgit.iaik.tugraz.at/krypto/hadesmimc/blob/master/code/calc_round_numbers.py
/// and then mapped onto `Fq` in the Ristretto scalar field.
pub const ROUND_CONSTANTS: [Scalar; CONSTANTS] = {
    let bytes = include_bytes!("../assets/ark.bin");
    let mut cnst = [Scalar::zero(); CONSTANTS];

    let mut i = 0;
    let mut j = 0;
    while i < bytes.len() {
        let a = u64_from_buffer(&bytes, i);
        let b = u64_from_buffer(&bytes, i + 8);
        let c = u64_from_buffer(&bytes, i + 16);
        let d = u64_from_buffer(&bytes, i + 24);

        cnst[j] = Scalar::from_raw([a, b, c, d]);
        j += 1;

        i += 32;
    }

    cnst
};

#[cfg(test)]
mod test {
    use super::ROUND_CONSTANTS;
    use bls12_381::Scalar;
    #[test]
    fn test_round_constants() {
        // Check each element is non-zero
        let zero = Scalar::zero();
        let has_zero = ROUND_CONSTANTS.iter().any(|&x| x == zero);
        for ctant in ROUND_CONSTANTS.iter() {
            let bytes = ctant.to_bytes();
            assert!(&Scalar::from_bytes(&bytes).unwrap() == ctant);
        }
        assert!(!has_zero);
    }
}
