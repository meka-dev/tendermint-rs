//! Amino types used by Secret Connection

use core::convert::TryFrom;

use ed25519_dalek as ed25519;
use prost_derive::Message;
use tendermint_proto as proto;

use crate::error::Error;

/// Amino prefix for "tendermint/PubKeyEd25519" (4-bytes) + length prefix (1-byte)
const PUB_KEY_ED25519_AMINO_PREFIX: [u8; 5] = [0x16, 0x24, 0xde, 0x64, 0x20];

/// Authentication signature message
#[derive(Clone, PartialEq, Message)]
pub struct AuthSigMessage {
    /// Public key
    #[prost(bytes, tag = "1")]
    pub pub_key: Vec<u8>,

    /// Signature
    #[prost(bytes, tag = "2")]
    pub sig: Vec<u8>,
}

impl AuthSigMessage {
    pub fn new(pub_key: &ed25519::PublicKey, sig: &ed25519::Signature) -> Self {
        let mut pub_key_bytes = Vec::from(PUB_KEY_ED25519_AMINO_PREFIX);
        pub_key_bytes.extend_from_slice(pub_key.as_ref());

        Self {
            pub_key: pub_key_bytes,
            sig: sig.as_ref().to_vec(),
        }
    }
}

impl TryFrom<AuthSigMessage> for tendermint_proto::p2p::AuthSigMessage {
    type Error = Error;

    fn try_from(amino_msg: AuthSigMessage) -> Result<Self, Error> {
        // Strip Amino prefix
        if amino_msg.pub_key.len() < 5 {
            return Err(Error::protocol());
        }

        let (amino_prefix, pub_key) = amino_msg.pub_key.split_at(5);

        if amino_prefix != PUB_KEY_ED25519_AMINO_PREFIX {
            return Err(Error::protocol());
        }

        let pub_key = proto::crypto::PublicKey {
            sum: Some(proto::crypto::public_key::Sum::Ed25519(pub_key.to_vec())),
        };

        Ok(Self {
            pub_key: Some(pub_key),
            sig: amino_msg.sig,
        })
    }
}
