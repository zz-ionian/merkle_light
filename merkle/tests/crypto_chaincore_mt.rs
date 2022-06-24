#![cfg(test)]
#![cfg(feature = "chaincore")]

extern crate crypto;
extern crate merkle_light;

use crypto::digest::Digest;
use crypto::sha3::{Sha3, Sha3Mode};
use merkle_light::hash::Algorithm;
use merkle_light::merkle::MerkleTree;
use std::fmt;
use std::hash::Hasher;
use std::iter::FromIterator;

#[derive(Clone)]
struct CryptoChainCoreAlgorithm(Sha3);

impl CryptoChainCoreAlgorithm {
    fn new() -> CryptoChainCoreAlgorithm {
        CryptoChainCoreAlgorithm(Sha3::new(Sha3Mode::Sha3_256))
    }
}

impl Default for CryptoChainCoreAlgorithm {
    fn default() -> CryptoChainCoreAlgorithm {
        CryptoChainCoreAlgorithm::new()
    }
}

impl Hasher for CryptoChainCoreAlgorithm {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.input(msg)
    }

    #[inline]
    fn finish(&self) -> u64 {
        unimplemented!()
    }
}

type CryptoSHA256Hash = [u8; 32];

impl Algorithm<CryptoSHA256Hash> for CryptoChainCoreAlgorithm {
    #[inline]
    fn hash(&mut self) -> CryptoSHA256Hash {
        let mut h = [0u8; 32];
        self.0.result(&mut h);
        h
    }

    #[inline]
    fn reset(&mut self) {
        self.0.reset();
    }
}

struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    fn new<T>(data: &'a T) -> HexSlice<'a>
    where
        T: ?Sized + AsRef<[u8]> + 'a,
    {
        HexSlice(data.as_ref())
    }
}

/// reverse order
impl<'a> fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.0.len();
        for i in 0..len {
            let byte = self.0[len - 1 - i];
            write!(f, "{:x}{:x}", byte >> 4, byte & 0xf)?;
        }
        Ok(())
    }
}

#[test]
fn test_crypto_chaincore_node() {
    let mut h1 = [0u8; 32];
    let mut h2 = [0u8; 32];
    let mut h3 = [0u8; 32];
    h1[0] = 0x00;
    h2[0] = 0x11;
    h3[0] = 0x22;

    let t: MerkleTree<CryptoSHA256Hash, CryptoChainCoreAlgorithm> =
        MerkleTree::from_iter(vec![h1, h2, h3]);
    assert_eq!(
        format!("{}", HexSlice::new(t.root().as_ref())),
        "23704c527ffb21d1b1816938114c2fb0f6e50475d4ab5d07ebff855e7fd20335"
    );
}
