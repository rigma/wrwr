//! # rtp
//!
//! `rtp` crate provides an implementation of RTP protocol in Rust.

extern crate chrono;

pub mod codecs;
mod packet;
mod packetizer;
mod payload_generator;
mod sequencer;

pub use packet::Packet;
pub use packetizer::Packetizer;
pub use payload_generator::PayloadGenerator;
pub use sequencer::Sequencer;
