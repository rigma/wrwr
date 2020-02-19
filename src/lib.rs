//! # rtp
//!
//! `rtp` crate provides an implementation of RTP protocol in Rust.

extern crate chrono;

pub mod codecs;
pub mod packet;
mod packetizer;
mod payload_generator;
mod sequencer;

pub use packet::Packet;
pub use packetizer::{
    G711Packetizer, G722Packetizer, H264Packetizer, OpusPacketizer, Packetizer, VP8Packetizer,
    VP9Packetizer,
};
pub use payload_generator::PayloadGenerator;
pub use sequencer::Sequencer;
