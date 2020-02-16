//! # rtp
//!
//! `rtp` crate provides an implementation of RTP protocol in Rust.

extern crate chrono;

mod packet;
mod packetizer;

pub use packet::Packet;
pub use packetizer::Packetizer;
