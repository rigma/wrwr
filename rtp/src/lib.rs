//! # wrwr-rtp
//!
//! `wrwr_rtp` crate provides an implementation of RTP protocol in Rust.

extern crate chrono;
#[macro_use]
extern crate failure;

pub mod codecs;
pub mod errors;
pub mod packet;
pub mod packetizer;
mod payload_generator;
mod sequencer;

pub use payload_generator::PayloadGenerator;
pub use sequencer::Sequencer;

/// A conveniance module appropriate for glob imports (`use wrwr_rtp::prelude::*;`).
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::errors::RtpPacketError;
    #[doc(no_inline)]
    pub use crate::packet::{Packet, HEADER_SIZE, RTP_VERSION};
    #[doc(no_inline)]
    pub use crate::packetizer::{
        ExtensionNumber, G711Packetizer, G722Packetizer, H264Packetizer, OpusPacketizer,
        Packetizer, VP8Packetizer, VP9Packetizer,
    };
}
