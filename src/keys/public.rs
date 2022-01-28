// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(feature = "std")]
use crate::{h0, h1, Signature};
use crate::{Error, SecretKey};
use bls12_381::G2Affine;

/// A BLS public key, holding a BLS12-381 G2 element inside.
/// The G2 element is constructed by multiplying a [`SecretKey`]
/// by `g2` (the base point of the G2 group).
/// Can be used for signature verification.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
pub struct PublicKey(pub(crate) G2Affine);

impl From<&SecretKey> for PublicKey {
    /// Generates a new [`PublicKey`] from a [`SecretKey`].
    /// pk = g_2 * sk
    fn from(sk: &SecretKey) -> Self {
        let g_2 = G2Affine::generator();
        let gx = g_2 * sk.0;

        Self(gx.into())
    }
}

impl PublicKey {
    /// Verify a [`Signature`] by comparing the results of the two pairing
    /// operations: e(sig, g_2) == e(Hₒ(m), pk).
    #[cfg(feature = "std")]
    pub fn verify(&self, sig: &Signature, msg: &[u8]) -> Result<(), Error> {
        let h0m = h0(msg);
        let p1 = bls12_381::pairing(&sig.0, &G2Affine::generator());
        let p2 = bls12_381::pairing(&h0m, &self.0);

        if p1.eq(&p2) {
            Ok(())
        } else {
            Err(Error::InvalidSignature)
        }
    }

    /// Return pk * t, where t is H_(pk).
    #[cfg(feature = "std")]
    pub fn pk_t(&self) -> G2Affine {
        let t = h1(self);
        let gx = self.0 * t;
        gx.into()
    }

    /// Return the compressed byte representation of the [`PublicKey`].
    pub fn to_bytes(&self) -> [u8; PublicKey::serialized_size()] {
        self.0.to_compressed()
    }

    /// Attempt to create a [`PublicKey`] from a G2Affine byte representation.
    pub fn from_bytes(
        bytes: &[u8; PublicKey::serialized_size()],
    ) -> Result<Self, Error> {
        let choice = G2Affine::from_compressed(&bytes);
        if choice.is_some().unwrap_u8() == 1 {
            Ok(Self(choice.unwrap()))
        } else {
            Err(Error::InvalidBytes)
        }
    }

    /// Return the amount of bytes needed to serialize a [`PublicKey`].
    pub const fn serialized_size() -> usize {
        96
    }
}
