use super::{
    header::{self, Header, PacketType},
    Packet, SSRC_LENGTH, SSRC_MAX_COUNT,
};

const REASON_MAX_LENGTH: usize = 255;

/// This structure represents the RTCP Goodbye packet. It is used
/// to indicates that some sources are closing their RTP connection.
///
/// This structure is following this data wire:
/// ```text
///        0               1               2               3
///        0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
///       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///       |V=2|P|    RC   |   PT=BYE=203  |             length            |
///       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///       |                           SSRC/CSRC                           |
///       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///       :                              ...                              :
///       +=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
/// (opt) |     length    |               reason for leaving            ...
///       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
///
/// A BYE packet is always giving a list of synchronization sources
/// which are closing their connection. Optional a BYE packet can have
/// a reason for leaving provided with a string which can not exceed a
/// length of 255 characters.
///
/// In a BYE packet, the RC field of the RTCP header indicates the number
/// of sources which are closing their connection.
#[derive(Clone, Debug)]
pub struct Goodbye {
    /// The parsed RTCP header of this packet
    pub header: Header,

    /// The list of sources which are closing their connection
    pub sources: Vec<u32>,

    /// An optional goodbye reason
    pub reason: Option<String>,
}

impl Packet for Goodbye {
    fn from_raw(raw_packet: &[u8]) -> Result<Self, ()> {
        let header = Header::from_raw(raw_packet)?;

        if header.packet_type != PacketType::Goodbye {
            return Err(());
        } else if raw_packet.len() % 4 > 0 {
            return Err(());
        }

        let reason_offset = header::HEADER_LENGTH + SSRC_LENGTH * header.report_count as usize;
        if reason_offset > raw_packet.len() {
            return Err(());
        }

        let reason = if reason_offset < raw_packet.len() {
            let length = raw_packet[reason_offset] as usize;
            if reason_offset + length + 1 > raw_packet.len() {
                return Err(());
            }

            if let Ok(reason) = String::from_utf8(Vec::from(
                &raw_packet[reason_offset + 1..reason_offset + length + 1],
            )) {
                Some(reason)
            } else {
                return Err(());
            }
        } else {
            None
        };

        let mut sources = Vec::with_capacity(header.report_count as usize);
        for i in 0..header.report_count as usize {
            let offset = header::HEADER_LENGTH + SSRC_LENGTH * i;

            let source = [
                raw_packet[offset],
                raw_packet[offset + 1],
                raw_packet[offset + 2],
                raw_packet[offset + 3],
            ];
            let source = u32::from_be_bytes(source);

            sources.push(source);
        }

        Ok(Goodbye {
            header,
            reason,
            sources,
        })
    }

    fn to_raw(&self) -> Result<Vec<u8>, ()> {
        if self.sources.len() > SSRC_MAX_COUNT {
            return Err(());
        }

        let marshalled_header = self.header.to_raw()?;

        let mut output = Vec::with_capacity(self.length());
        for _ in 0..output.len() {
            output.push(0);
        }

        output[..header::HEADER_LENGTH].copy_from_slice(&marshalled_header);
        self.sources.iter().enumerate().for_each(|(i, ssrc)| {
            let offset = header::HEADER_LENGTH + SSRC_LENGTH * i;

            output[offset..offset + 3].copy_from_slice(&ssrc.to_be_bytes());
        });

        if let Some(reason) = &self.reason {
            let reason = reason.as_bytes();
            if reason.len() > REASON_MAX_LENGTH {
                return Err(());
            }

            let offset = header::HEADER_LENGTH + SSRC_LENGTH * self.sources.len();
            output[offset] = reason.len() as u8;
            output[offset + 1..offset + reason.len() + 1].copy_from_slice(reason);
        }

        Ok(output)
    }

    fn length(&self) -> usize {
        let mut length = header::HEADER_LENGTH + SSRC_LENGTH * self.sources.len();
        if let Some(reason) = &self.reason {
            length += reason.len() + 1;
        }

        // We'll compute the number of missing bytes to have a length divisible by 4
        let padding = if length % 4 > 0 { 4 - length % 4 } else { 0 };

        length + padding
    }

    fn synchronization_sources(&self) -> std::vec::Vec<u32> {
        self.sources.clone()
    }
}

impl Eq for Goodbye {}

impl PartialEq for Goodbye {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.sources == other.sources && self.reason == other.reason
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_unmarshalls_a_goodbye_packet() {
        let raw = [
            0x81u8, 0xcbu8, 0x00u8, 0x0cu8, // V=2, P=0, RC=1, PT=BYE, length=12
            0x90u8, 0x2fu8, 0x9eu8, 0x2eu8, // SSRC=[0x902f9e2e]
            0x03u8, 0x46u8, 0x4fu8, 0x4fu8, // length=3, reason="FOO"
        ];
        let expected = Goodbye {
            header: Header {
                padding: false,
                report_count: 1,
                packet_type: PacketType::Goodbye,
                length: raw.len() as u16,
            },
            sources: vec![0x902f9e2e],
            reason: Some(String::from("FOO")),
        };

        let packet = Goodbye::from_raw(&raw);
        assert!(packet.is_ok());

        let packet = packet.unwrap();
        assert_eq!(expected, packet);
    }

    #[test]
    fn it_returns_an_error_when_reason_length_is_invalid() {
        let raw = [
            0x81u8, 0xcbu8, 0x00u8, 0x0cu8, // V=2, P=0, RC=1, PT=BYE, length=12
            0x90u8, 0x2fu8, 0x9eu8, 0x2eu8, // SSRC=[0x902f9e2e]
            0x04u8, 0x46u8, 0x4fu8, 0x4fu8, // length=4, reason="FOO"
        ];

        let packet = Goodbye::from_raw(&raw);
        assert!(packet.is_err());
    }

    #[test]
    fn it_returns_an_error_when_packet_type_is_wrong() {
        let raw = [
            0x81u8, 0xcau8, 0x00u8, 0x0cu8, // V=2, P=0, RC=1, PT=SDES, length=12
            0x90u8, 0x2fu8, 0x9eu8, 0x2eu8, // SSRC=[0x902f9e2e]
            0x03u8, 0x46u8, 0x4fu8, 0x4fu8, // length=3, reason="FOO"
        ];

        let packet = Goodbye::from_raw(&raw);
        assert!(packet.is_err());
    }

    #[test]
    fn it_returns_an_error_when_bytes_are_not_aligned() {
        let raw = [
            0x81u8, 0xcbu8, 0x00u8, 0x0cu8, // V=2, P=0, RC=1, PT=BYE, length=12
            0x90u8, 0x2fu8, 0x9eu8, 0x2eu8, // SSRC=[0x902f9e2e]
            0x01u8, 0x46u8, // length=1, reason="F"
        ];

        let packet = Goodbye::from_raw(&raw);
        assert!(packet.is_err());
    }

    #[test]
    fn it_returns_an_error_when_report_count_is_wrong() {
        let raw = [
            0x82u8, 0xcbu8, 0x00u8, 0x0cu8, // V=2, P=0, RC=1, PT=BYE, length=8
            0x90u8, 0x2fu8, 0x9eu8, 0x2eu8, // SSRC=[0x902f9e2e]
        ];

        let packet = Goodbye::from_raw(&raw);
        assert!(packet.is_err());
    }
}
