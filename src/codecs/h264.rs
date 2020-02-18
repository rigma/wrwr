use crate::PayloadGenerator;

const FUA_HEADER_SIZE: usize = 2;
const NAL_UNIT_TYPE_MASK: u8 = 0x1f;
const NAL_UNIT_REF_IDC_MASK: u8 = 0x60;

/// This payload generator is responsible to generate RTP packet's payloads
/// from H.264 data in order to send them into a RTP stream.
#[derive(Clone, Copy, Debug, Default)]
pub struct H264PayloadGenerator;

impl H264PayloadGenerator {
    /// Determines the boundaries of a NAL unit of a H.264 payload. The
    /// boundaries are returned into a tuple defined this way: `(start, length)`.
    ///
    /// If no boundaries can be found, `None` is returned.
    fn get_nal_unit_boundaries(units: &[u8], start: usize) -> Option<(usize, usize)> {
        let mut zero_count = 0;

        for (index, byte) in units[start..].iter().enumerate() {
            match *byte {
                0u8 => zero_count += 1,
                1u8 => {
                    // If we've counted 2 or more zero byte, we've got our boundaries.
                    if zero_count >= 2 {
                        return Some((start + index - zero_count, zero_count + 1));
                    }
                }
                _ => zero_count = 0,
            };
        }

        None
    }

    /// Generates RTP payloads from a NAL unit thanks to the MTU.
    fn generate_payloads_from_nal_unit(mtu: usize, unit: &[u8]) -> Option<Vec<Vec<u8>>> {
        let unit_type = unit[0] & NAL_UNIT_TYPE_MASK;

        // If the unit type is invalid or if the MTU size is less than the FU-A header
        // size, we can not generate payloads.
        if unit_type == 9 || unit_type == 12 || mtu < FUA_HEADER_SIZE {
            return None;
        }

        let ref_idc = unit[0] & NAL_UNIT_REF_IDC_MASK;
        let mut payloads = Vec::new();

        // If the NAL unit's length is smaller than the MTU, then the unit
        // can be fitted into one RTP packet.
        if unit.len() < mtu {
            payloads.push(Vec::from(unit));

            return Some(payloads);
        }

        // We keep in memory the max fragment size and the unit's length
        let max_fragment_size = mtu - FUA_HEADER_SIZE;
        let unit_data_length = unit.len() - 1;

        // Work variables
        let mut unit_data_remaining = unit_data_length;
        let mut unit_data_index = 1;

        // Since we can not put the whole unit into one RTP packet, we'll fragment
        // the NAL unit into FU payloads which will be sequentially concatenated to
        // let the receiver reconstructs the unit.
        while unit_data_remaining > 0 {
            // Computing the payload size
            let payload_size = max_fragment_size.min(unit_data_remaining);
            let mut payload = Vec::with_capacity(FUA_HEADER_SIZE + payload_size);

            // Initializing payload
            for _ in 0..FUA_HEADER_SIZE + payload_size {
                payload.push(0x00);
            }

            // Defining the FUA header of payload following this wire:
            //
            // +---------------+
            // |0|1|2|3|4|5|6|7|
            // +-+-+-+-+-+-+-+-+
            // |F|NRI|   type  |
            // +-+-+-+---------+
            // |S|E|R|   type  |
            // +-+-+-+---------+
            // |      ...      |
            // +---------------+

            // Setting NAL Ref IDC (NRI) in the first byte of the payload
            payload[0] = 28;
            payload[0] |= ref_idc;

            // Adding the unit type and if the payload is the first or the last
            // for this unit
            payload[1] = unit_type;
            if unit_data_remaining == unit_data_length {
                payload[1] |= 1 << 7;
            } else if unit_data_remaining - payload_size == 0 {
                payload[1] |= 1 << 6;
            }

            // Adding the FU packet's payload
            payload[FUA_HEADER_SIZE..]
                .copy_from_slice(&unit[unit_data_index..unit_data_index + payload_size]);

            // Saving the produced packet
            payloads.push(payload);

            // Refreshing working variables
            unit_data_remaining -= payload_size;
            unit_data_index += payload_size;
        }

        Some(payloads)
    }
}

impl PayloadGenerator for H264PayloadGenerator {
    fn generate(&self, mtu: usize, payload: &[u8]) -> Option<Vec<Vec<u8>>> {
        if payload.len() == 0 {
            return None;
        }

        let mut output = Vec::new();

        // Retrieving NAL unit boundaries
        let mut boundaries = Self::get_nal_unit_boundaries(payload, 0);

        // If no boundaries are returned, then there is no NAL unit. In this case
        // we'll consider the payload as a big NAL unit and produce payloads from it
        if let None = boundaries {
            let mut payloads = Self::generate_payloads_from_nal_unit(mtu, payload);
            if let Some(payloads) = &mut payloads {
                output.append(payloads);

                return Some(output);
            }

            return None;
        }

        // We'll produce a RTP payload for each NAL unit found
        while let Some((start, length)) = boundaries {
            let previous_start = start + length;
            boundaries = Self::get_nal_unit_boundaries(payload, previous_start);

            let mut payloads = if let Some(boundaries) = &boundaries {
                Self::generate_payloads_from_nal_unit(mtu, &payload[previous_start..boundaries.0])
            } else {
                Self::generate_payloads_from_nal_unit(mtu, &payload[previous_start..])
            };

            if let Some(payloads) = &mut payloads {
                output.append(payloads);
            }
        }

        if output.len() > 0 {
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
    fn it_generates_rtp_payload_from_small_h264_data() {
        let generator = H264PayloadGenerator::default();
        let payload = [0x90u8; 3];

        let payloads = generator.generate(5, &payload);
        assert!(payloads.is_some());

        let payloads = payloads.unwrap();
        assert_eq!(1, payloads.len());
        assert_eq!(3, payloads[0].len());
    }

    #[test]
    fn it_generates_multiple_rtp_payloads_from_h264_data() {
        let generator = H264PayloadGenerator::default();
        let payload = [
            0x00u8, 0x00u8, 0x01u8, 0x90u8, 0x00u8, 0x00u8, 0x01u8, 0x90u8,
        ];

        let payloads = generator.generate(5, &payload);
        assert!(payloads.is_some());

        let payloads = payloads.unwrap();
        assert_eq!(2, payloads.len());

        for payload in payloads {
            assert_eq!(1, payload.len());
        }
    }

    #[test]
    fn it_returns_none_for_empty_payload() {
        let generator = H264PayloadGenerator::default();

        let payloads = generator.generate(5, &[]);
        assert!(payloads.is_none());
    }

    #[test]
    fn it_returns_none_for_null_mtu() {
        let generator = H264PayloadGenerator::default();
        let payload = [0x90u8; 3];

        let payloads = generator.generate(0, &payload);
        assert!(payloads.is_none());
    }

    #[test]
    fn it_returns_none_for_ignored_nal_types() {
        let generator = H264PayloadGenerator::default();
        let payload = [0x09u8, 0x00u8, 0x00u8];

        let payloads = generator.generate(5, &payload);
        assert!(payloads.is_none());
    }
}
