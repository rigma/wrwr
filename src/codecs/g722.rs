use crate::PayloadGenerator;

/// This payload generator is used when you want to send G722 packets
/// into an RTP data stream.
#[derive(Clone, Copy, Debug, Default)]
pub struct G722PayloadGenerator;

impl PayloadGenerator for G722PayloadGenerator {
    fn generate(&mut self, mtu: usize, payload: &[u8]) -> Option<Vec<Vec<u8>>> {
        if mtu == 0 || payload.len() == 0 {
            return None;
        }

        let mut output = Vec::new();
        let mut offset = 0;

        while payload.len() - offset > mtu {
            let payload = Vec::from(&payload[offset..offset + mtu]);

            output.push(payload);
            offset += mtu;
        }

        let payload = Vec::from(&payload[offset..]);
        output.push(payload);

        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngCore;

    const LENGTH: usize = 10000;
    const MTU: usize = 1500;

    #[test]
    fn it_generates_rtp_payloads() {
        let mut samples = [0u8; LENGTH];
        let mut generator = G722PayloadGenerator::default();

        rand::thread_rng().fill_bytes(&mut samples);

        let payloads = generator.generate(MTU, &samples);
        assert!(payloads.is_some());

        let payloads = payloads.unwrap();

        assert_eq!(payloads.len(), (LENGTH as f64 / MTU as f64).ceil() as usize);
        assert_eq!(
            samples[..],
            payloads.into_iter().flatten().collect::<Vec<_>>()[..]
        );
    }

    #[test]
    fn it_returns_no_payload_when_mtu_is_null() {
        let mut generator = G722PayloadGenerator::default();
        let samples = [0x90u8; 3];

        let payloads = generator.generate(0, &samples);
        assert!(payloads.is_none());
    }

    #[test]
    fn it_generates_packets_according_to_mtu() {
        let mut generator = G722PayloadGenerator::default();
        let samples = [0x90u8; 3];

        let payloads = generator.generate(1, &samples).unwrap();
        assert_eq!(3, payloads.len());

        let payloads = generator.generate(2, &samples).unwrap();
        assert_eq!(2, payloads.len());

        let payloads = generator.generate(10, &samples).unwrap();
        assert_eq!(1, payloads.len());
    }
}
