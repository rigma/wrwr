use crate::PayloadGenerator;

const VP8_HEADER_SIZE: usize = 1;

/// This payload generator is responsible to generate RTP packet's payloads
/// from VP8 data in order to send them into a RTP stream.
#[derive(Clone, Copy, Debug, Default)]
pub struct VP8PayloadGenerator;

impl PayloadGenerator for VP8PayloadGenerator {
    fn generate(&mut self, mtu: usize, payload: &[u8]) -> Option<Vec<Vec<u8>>> {
        if mtu <= VP8_HEADER_SIZE {
            return None;
        }

        let mut output = Vec::new();

        let max_fragment_size = mtu - VP8_HEADER_SIZE;
        let mut data_remaining = payload.len();
        let mut data_index = 0;

        while data_remaining > 0 {
            let payload_size = max_fragment_size.min(data_remaining);
            let mut generated = Vec::with_capacity(VP8_HEADER_SIZE + payload_size);

            // Initializing the generated payload
            for _ in 0..VP8_HEADER_SIZE + payload_size {
                generated.push(0x00u8);
            }

            generated[VP8_HEADER_SIZE..]
                .copy_from_slice(&payload[data_index..data_index + payload_size]);
            output.push(generated);

            data_remaining -= payload_size;
            data_index += payload_size;
        }

        if output.len() > 0 {
            output[0][0] = 0x10;

            Some(output)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_rtp_payload() {
        let mut generator = VP8PayloadGenerator::default();
        let payload = [0x90u8; 3];

        let payloads = generator.generate(VP8_HEADER_SIZE + 1, &payload);
        assert!(payloads.is_some());

        let payloads = payloads.unwrap();
        assert_eq!(payload.len(), payloads.len());
        assert_eq!(0x10, payloads[0][0]);
    }

    #[test]
    fn it_returns_none_when_payload_is_empty() {
        let mut generator = VP8PayloadGenerator::default();

        let payloads = generator.generate(2, &[]);
        assert!(payloads.is_none());
    }

    #[test]
    fn it_returns_none_when_mtu_is_too_small() {
        let mut generator = VP8PayloadGenerator::default();
        let payload = [0x90u8; 3];

        let payloads = generator.generate(1, &payload);
        assert!(payloads.is_none());
    }
}
