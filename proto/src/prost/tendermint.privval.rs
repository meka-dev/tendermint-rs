#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteSignerError {
    #[prost(int32, tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
}
/// PubKeyRequest requests the consensus public key from the remote signer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKeyRequest {
    #[prost(string, tag="1")]
    pub chain_id: ::prost::alloc::string::String,
}
/// PubKeyResponse is a response message containing the public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKeyResponse {
    #[prost(message, optional, tag="1")]
    pub pub_key: ::core::option::Option<super::crypto::PublicKey>,
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
/// SignVoteRequest is a request to sign a vote
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignVoteRequest {
    #[prost(message, optional, tag="1")]
    pub vote: ::core::option::Option<super::types::Vote>,
    #[prost(string, tag="2")]
    pub chain_id: ::prost::alloc::string::String,
}
/// SignedVoteResponse is a response containing a signed vote or an error
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedVoteResponse {
    #[prost(message, optional, tag="1")]
    pub vote: ::core::option::Option<super::types::Vote>,
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
/// SignProposalRequest is a request to sign a proposal
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignProposalRequest {
    #[prost(message, optional, tag="1")]
    pub proposal: ::core::option::Option<super::types::Proposal>,
    #[prost(string, tag="2")]
    pub chain_id: ::prost::alloc::string::String,
}
/// SignedProposalResponse is response containing a signed proposal or an error
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedProposalResponse {
    #[prost(message, optional, tag="1")]
    pub proposal: ::core::option::Option<super::types::Proposal>,
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
/// PingRequest is a request to confirm that the connection is alive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
}
/// PingResponse is a response to confirm that the connection is alive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
}
/// MekatekBuild represents a request for Mekatek to build a block.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MekatekBuild {
    #[prost(string, tag="1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub height: i64,
    #[prost(string, tag="3")]
    pub validator_addr: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub max_bytes: i64,
    #[prost(int64, tag="5")]
    pub max_gas: i64,
    #[prost(bytes="vec", tag="6")]
    pub txs_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// SignMekatekBuildRequest is a request to sign a MektatekBuildBlockRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMekatekBuildRequest {
    #[prost(message, optional, tag="1")]
    pub build: ::core::option::Option<MekatekBuild>,
}
/// SignedMekatekBuildResponse is response containing a signed MekatekBuild or an error
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedMekatekBuildResponse {
    #[prost(message, optional, tag="1")]
    pub build: ::core::option::Option<MekatekBuild>,
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
/// MekatekChallenge is a registration challenge that validators must sign in order to authenticate with
/// Mekatek.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MekatekChallenge {
    #[prost(string, tag="1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub challenge: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// SignMekatekChallengeRequest is a request to sign a MektatekChallenge
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMekatekChallengeRequest {
    #[prost(message, optional, tag="1")]
    pub challenge: ::core::option::Option<MekatekChallenge>,
}
/// SignedMekatekChallengeResponse is response containing a signed MekatekChallenge or an error
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedMekatekChallengeResponse {
    #[prost(message, optional, tag="1")]
    pub challenge: ::core::option::Option<MekatekChallenge>,
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(oneof="message::Sum", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12")]
    pub sum: ::core::option::Option<message::Sum>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sum {
        #[prost(message, tag="1")]
        PubKeyRequest(super::PubKeyRequest),
        #[prost(message, tag="2")]
        PubKeyResponse(super::PubKeyResponse),
        #[prost(message, tag="3")]
        SignVoteRequest(super::SignVoteRequest),
        #[prost(message, tag="4")]
        SignedVoteResponse(super::SignedVoteResponse),
        #[prost(message, tag="5")]
        SignProposalRequest(super::SignProposalRequest),
        #[prost(message, tag="6")]
        SignedProposalResponse(super::SignedProposalResponse),
        #[prost(message, tag="7")]
        PingRequest(super::PingRequest),
        #[prost(message, tag="8")]
        PingResponse(super::PingResponse),
        #[prost(message, tag="9")]
        SignMekatekBuildRequest(super::SignMekatekBuildRequest),
        #[prost(message, tag="10")]
        SignedMekatekBuildResponse(super::SignedMekatekBuildResponse),
        #[prost(message, tag="11")]
        SignMekatekChallengeRequest(super::SignMekatekChallengeRequest),
        #[prost(message, tag="12")]
        SignedMekatekChallengeResponse(super::SignedMekatekChallengeResponse),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Errors {
    Unknown = 0,
    UnexpectedResponse = 1,
    NoConnection = 2,
    ConnectionTimeout = 3,
    ReadTimeout = 4,
    WriteTimeout = 5,
}
impl Errors {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Errors::Unknown => "ERRORS_UNKNOWN",
            Errors::UnexpectedResponse => "ERRORS_UNEXPECTED_RESPONSE",
            Errors::NoConnection => "ERRORS_NO_CONNECTION",
            Errors::ConnectionTimeout => "ERRORS_CONNECTION_TIMEOUT",
            Errors::ReadTimeout => "ERRORS_READ_TIMEOUT",
            Errors::WriteTimeout => "ERRORS_WRITE_TIMEOUT",
        }
    }
}
