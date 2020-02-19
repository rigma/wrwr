use crate::PayloadGenerator;
use rand::Rng;

const VP9_HEADER_SIZE: usize = 3;

/// This payload generator is responsible to generate RTP packet's payloads
/// from VP9 data in order to send them into a RTP stream.
#[derive(Clone, Copy, Debug)]
pub struct VP9PayloadGenerator {
    pub picture_id: u16,
    pub initialized: bool,
}

impl VP9PayloadGenerator {
    /// Initialize internal picture ID to a random value.
    pub fn initialize(&mut self) {
        self.picture_id = rand::thread_rng().gen();
        self.initialized = true;
    }
}

impl Default for VP9PayloadGenerator {
    fn default() -> Self {
        let picture_id = rand::thread_rng().gen();

        Self {
            picture_id,
            initialized: true,
        }
    }
}

impl PayloadGenerator for VP9PayloadGenerator {
    fn generate(&mut self, mtu: usize, payload: &[u8]) -> Option<Vec<Vec<u8>>> {
        if payload.len() == 0 || mtu <= VP9_HEADER_SIZE {
            return None;
        }

        // If the generator is not yet initialized, we'll do it before instanciating
        // payloads.
        if !self.initialized {
            self.initialize();
        }

        let mut payloads = Vec::new();

        // Working variables
        let max_size = mtu - VP9_HEADER_SIZE;
        let mut data_remaining = payload.len();
        let mut data_index = 0;

        while data_remaining > 0 {
            let current_size = max_size.min(data_remaining);

            // Initiaziling the RTP payload
            let mut generated = Vec::with_capacity(VP9_HEADER_SIZE + current_size);
            for _ in 0..VP9_HEADER_SIZE + current_size {
                generated.push(0x00);
            }

            // Defining header for the current fragment according to https://www.ietf.org/id/draft-ietf-payload-vp9-09.txt
            generated[0] = 0x90;

            if data_index == 0 {
                generated[0] |= 0x08;
            }
            if data_remaining == current_size {
                generated[0] |= 0x04;
            }

            generated[1] = (self.picture_id >> 8) as u8 | 0x80;
            generated[2] = self.picture_id as u8;

            generated[VP9_HEADER_SIZE..]
                .copy_from_slice(&payload[data_index..data_index + current_size]);

            payloads.push(generated);

            data_remaining -= current_size;
            data_index += current_size;
        }

        // Incrementing picture ID and checking if it has overflowed
        self.picture_id += 1;
        if self.picture_id >= 0x8000 {
            self.picture_id = 0;
        }

        if payloads.len() > 0 {
            Some(payloads)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_produces_rtp_payload_for_one_packet() {
        let mut generator = VP9PayloadGenerator {
            picture_id: 0,
            initialized: true,
        };
        let expected = vec![vec![0x9c, 0x80, 0x00, 0x01, 0x02]];

        let payloads = generator.generate(10, &[0x01, 0x02]);
        assert!(payloads.is_some());

        let payloads = payloads.unwrap();
        assert_eq!(1, payloads.len());
        assert_eq!(expected, payloads);
    }

    #[test]
    fn it_produces_rtp_payload_for_two_packets() {
        let mut generator = VP9PayloadGenerator {
            picture_id: 0,
            initialized: true,
        };
        let expected = vec![vec![0x98, 0x80, 0x00, 0x01], vec![0x94, 0x80, 0x00, 0x02]];

        let payloads = generator.generate(4, &[0x01, 0x02]);
        assert!(payloads.is_some());

        let payloads = payloads.unwrap();
        assert_eq!(2, payloads.len());
        assert_eq!(expected, payloads);
    }

    #[test]
    fn it_produces_rtp_payload_for_three_packets() {
        let mut generator = VP9PayloadGenerator {
            picture_id: 0,
            initialized: true,
        };
        let expected = vec![
            vec![0x98, 0x80, 0x00, 0x01],
            vec![0x90, 0x80, 0x00, 0x02],
            vec![0x94, 0x80, 0x00, 0x03],
        ];

        let payloads = generator.generate(4, &[0x01, 0x02, 0x03]);
        assert!(payloads.is_some());

        let payloads = payloads.unwrap();
        assert_eq!(3, payloads.len());
        assert_eq!(expected, payloads);
    }

    #[test]
    fn it_produces_rtp_payload_for_two_packet_frames() {
        let mut generator = VP9PayloadGenerator {
            picture_id: 0,
            initialized: true,
        };
        let expected = vec![
            vec![0x98, 0x80, 0x00, 0x01, 0x02],
            vec![0x94, 0x80, 0x00, 0x03],
        ];

        let payloads = generator.generate(5, &[0x01, 0x02, 0x03]);
        assert!(payloads.is_some());
        assert_eq!(expected, payloads.unwrap());

        let expected = vec![vec![0x9c, 0x80, 0x01, 0x04]];

        let payloads = generator.generate(5, &[0x04]);
        assert!(payloads.is_some());
        assert_eq!(expected, payloads.unwrap());
    }

    #[test]
    fn it_returns_none_when_payload_is_empty() {
        let mut generator = VP9PayloadGenerator::default();

        assert!(generator.generate(5, &[]).is_none());
    }

    #[test]
    fn it_returns_none_when_mtu_is_too_small() {
        let mut generator = VP9PayloadGenerator::default();

        assert!(generator.generate(1, &[0x01; 3]).is_none());
    }
}
