use crate::PayloadGenerator;

/// This payload generator is responsible to generate RTP packet's payloads
/// from Opus data in order to send them into a RTP stream.
///
/// > This payload generator is not using the MTU parameter for payload
/// > generation.
#[derive(Clone, Copy, Debug, Default)]
pub struct OpusPayloadGenerator;

impl PayloadGenerator for OpusPayloadGenerator {
    // TODO: investigate why the MTU is ignored here
    fn generate(&mut self, _mtu: usize, payload: &[u8]) -> Option<Vec<Vec<u8>>> {
        if payload.len() == 0 {
            return None;
        }

        let mut output = Vec::new();
        output.push(Vec::from(payload));

        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_a_rtp_payload() {
        let mut generator = OpusPayloadGenerator::default();
        let payload = [0x90u8; 3];

        let payloads = generator.generate(42, &payload);
        assert!(payloads.is_some());
    }

    #[test]
    fn it_returns_none_when_payload_is_empty() {
        let mut generator = OpusPayloadGenerator::default();

        let payloads = generator.generate(24, &[]);
        assert!(payloads.is_none());
    }
}
