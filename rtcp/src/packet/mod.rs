/// This trait is shared by all RTCP packets implemented in this
/// library.
///
/// It defines the common behavior that all RTCP packets shall
/// implement.
pub trait Packet {
    /// Transformed a marshalled RTCP packet into a rusty
    /// representation.
    fn from_raw(raw_packet: &[u8]) -> Self;

    /// Exports the RTCP packet into a marshalled representation.
    fn to_raw(&self) -> Result<Vec<u8>, ()>;

    /// Retrieves the list of synchronization sources (SSRC) that
    /// this RTCP packet refers to.
    fn synchronization_sources(&self) -> Vec<u32>;
}