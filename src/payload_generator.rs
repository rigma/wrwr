/// This trait defines the mandatory methods that a payload
/// generator structure shpuld implement in order to slice
/// data into suitable RTP packet's payloads.
pub trait PayloadGenerator {
    fn generate(&self, maximum_transmission_unit: usize, payload: &[u8]) -> Vec<Vec<u8>>;
}
