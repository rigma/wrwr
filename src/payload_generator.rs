/// This trait defines the mandatory methods that a payload
/// generator structure shpuld implement in order to slice
/// data into suitable RTP packet's payloads.
pub trait PayloadGenerator {
    /// Try to generates a sequence of suitable RTP packet's payload
    /// from MTU (Maximum Transmission Unit) and an arbitrary payload.
    ///
    /// If no payloads can be generated, this method should return `None`.
    fn generate(&self, mtu: usize, payload: &[u8]) -> Option<Vec<Vec<u8>>>;
}
