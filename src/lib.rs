//! # rtp
//!
//! `rtp` crate provides an implementation of RTP protocol in Rust.

extern crate chrono;

mod packet;
mod packetizer;
mod payload_generator;

pub use packet::Packet;
pub use packetizer::Packetizer;
pub use payload_generator::PayloadGenerator;
