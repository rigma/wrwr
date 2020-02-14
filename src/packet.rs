const HEADER_LENGHT: usize = 4;
const VERSION_SHIFT: usize = 6;
const VERSION_MASK: u8 = 0x03;
const PADDING_SHIFT: usize = 5;
const PADDING_MASK: u8 = 0x1;
const EXTENSION_SHIFT: usize = 4;
const EXTENSION_MASK: u8 = 0x01;
const CC_MASK: u8 = 0x0f;
const MARKER_SHIFT: usize = 7;
const MARKER_MASK: u8 = 0x01;
const PT_MASK: u8 = 0x7f;
const SEQ_NUM_OFFSET: usize = 2;
const TIMESTAMP_OFFSET: usize = 4;
const SSRC_OFFSET: usize = 8;
const CSRC_OFFSET: usize = 12;
const CSRC_LENGTH: usize = 4;

#[derive(Clone, Debug, Default)]
pub struct Packet {
    pub version: u8,
    pub padding: bool,
    pub extension: bool,
    pub marker: bool,
    pub payload_offset: usize,
    pub payload_type: u8,
    pub sequence_number: u16,
    pub timestamp: u32,
    pub ssrc: u32,
    pub csrc: Vec<u32>,
    pub extension_profile: Option<u16>,
    pub extension_payload: Option<Vec<u8>>,
    pub payload: Vec<u8>,
    pub raw: Vec<u8>,
}

impl Packet {
    pub fn unmarshal(raw_packet: &[u8]) -> Result<Packet, ()> {
        if raw_packet.len() < HEADER_LENGHT {
            return Err(());
        }

        let version = (raw_packet[0] >> VERSION_SHIFT) & VERSION_MASK;
        let padding = (raw_packet[0] >> PADDING_SHIFT) & PADDING_MASK > 0;
        let extension = (raw_packet[0] >> EXTENSION_SHIFT) & EXTENSION_MASK > 0;
        let mut csrc = Vec::<u32>::with_capacity((raw_packet[0] & CC_MASK) as usize);

        let marker = (raw_packet[1] >> MARKER_SHIFT) & MARKER_MASK > 0;
        let payload_type = raw_packet[1] & PT_MASK;

        let sequence_number: u16 = (raw_packet[SEQ_NUM_OFFSET] as u16) << 8 | raw_packet[SEQ_NUM_OFFSET + 1] as u16;
        let sequence_number = sequence_number.to_be();

        let timestamp: u32 = (raw_packet[TIMESTAMP_OFFSET] as u32) << 24
            | (raw_packet[TIMESTAMP_OFFSET + 1] as u32) << 16
            | (raw_packet[TIMESTAMP_OFFSET + 2] as u32) << 8
            | (raw_packet[SSRC_OFFSET + 3] as u32);
        let timestamp = timestamp.to_be();

        let ssrc: u32 = (raw_packet[SSRC_OFFSET] as u32) << 24
            | (raw_packet[SSRC_OFFSET + 1] as u32) << 16
            | (raw_packet[SSRC_OFFSET + 2] as u32) << 8
            | (raw_packet[SSRC_OFFSET + 3] as u32);
        let ssrc = ssrc.to_be();

        let mut payload_offset = CSRC_OFFSET + CSRC_LENGTH * csrc.len();
        if raw_packet.len() < payload_offset {
            return Err(());
        }

        for i in 0..csrc.len() {
            let offset = CSRC_OFFSET + CSRC_LENGTH * i;

            csrc[i] = (raw_packet[offset] as u32) << 24
                | (raw_packet[offset + 1] as u32) << 16
                | (raw_packet[offset + 2] as u32) << 8
                | (raw_packet[offset + 3] as u32);
        }

        let mut extension_profile: Option<u16> = None;
        let mut extension_payload: Option<Vec<u8>> = None;

        if extension {
            if raw_packet.len() < payload_offset + 4 {
                return Err(());
            }

            extension_profile =
                Some((raw_packet[payload_offset] as u16) << 8 | (raw_packet[payload_offset + 1] as u16));
            payload_offset += 2;

            let extension_length: usize =
                4 * ((raw_packet[payload_offset] as usize) << 8 | (raw_packet[payload_offset + 1] as usize));
            payload_offset += 2;

            if raw_packet.len() < payload_offset + extension_length {
                return Err(());
            }

            extension_payload = Some(Vec::from(
                &raw_packet[payload_offset..payload_offset + extension_length],
            ));
            payload_offset += extension_length;
        }

        let payload = Vec::from(&raw_packet[payload_offset..]);
        let raw = Vec::from(raw_packet);

        Ok(Self {
            version,
            padding,
            extension,
            marker,
            payload_offset,
            payload_type,
            sequence_number,
            timestamp,
            ssrc,
            csrc,
            extension_profile,
            extension_payload,
            payload,
            raw,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_unmarshall_on_empty_packet_produce_an_error() {
        assert!(Packet::unmarshal(&vec![]).is_err());
    }

    #[test]
    fn it_unmarshall_a_packet_without_extension() {
        let raw_packet: [u8; 25] = [
            0x90, 0xe0, 0x69, 0x8f, 0xd9, 0xc2, 0x93, 0xda, 0x1c, 0x64, 0x27, 0x82, 0x00, 0x01,
            0x00, 0x01, 0xff, 0xff, 0xff, 0xff, 0x98, 0x36, 0xbe, 0x88, 0x9e,
        ];

        assert!(Packet::unmarshal(&raw_packet).is_ok());
    }
}
