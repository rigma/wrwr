mod goodbye;
pub mod header;
mod sender_report;

pub use goodbye::Goodbye;
pub use header::Header;
pub use sender_report::SenderReport;

pub(self) const SSRC_LENGTH: usize = 4;
pub(self) const SSRC_MAX_COUNT: usize = 0x1f;

/// This trait is shared by all RTCP packets implemented in this
/// library.
///
/// It defines the common behavior that all RTCP packets shall
/// implement.
pub trait Packet {
    /// Transformed a marshalled RTCP packet into a rusty
    /// representation.
    fn from_raw(raw_packet: &[u8]) -> Result<Self, ()>
    where
        Self: Sized;

    /// Exports the RTCP packet into a marshalled representation.
    fn to_raw(&self) -> Result<Vec<u8>, ()>;

    /// Gets the length of the marshalled RTCP packet.
    fn length(&self) -> usize;

    /// Retrieves the list of synchronization sources (SSRC) that
    /// this RTCP packet refers to.
    fn synchronization_sources(&self) -> Vec<u32>;
}
