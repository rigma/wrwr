use std::fmt;

/// The version of the RTCP protocol implemented.
pub const RTCP_VERSION: u8 = 2;

const HEADER_LENGTH: usize = 4;
const VERSION_MASK: u8 = 0x3;
const VERSION_SHIFT: usize = 6;
const PADDING_MASK: u8 = 0x1;
const PADDING_SHIFT: usize = 5;
const COUNT_MASK: u8 = 0x1f;

/// This enumeration lists the different types of RTCP packages
/// as defined in the [RFC 3550] and [RFC 4585].
///
/// You can also find RTCP packet in [IANA register].
///
/// [RFC 3550]: https://tools.ietf.org/html/rfc3550
/// [RFC 4585]: https://tools.ietf.org/html/rfc4585
/// [IANA register]: https://www.iana.org/assignments/rtp-parameters/rtp-parameters.xhtml#rtp-parameters-4
#[derive(Clone, Copy, Debug)]
pub enum PacketType {
    /// The [Sender Report] (SR) RTCP packet type.
    ///
    /// [Sender Report]: https://tools.ietf.org/html/rfc3550#section-6.4.1
    SenderReport = 200,

    /// The [Receiver Report] (RR) RTCP packet type.
    ///
    /// [Receiver Report]: https://tools.ietf.org/html/rfc3550#section-6.4.2
    ReceiverReport = 201,

    /// The [Source Description] (SDES) RTCP packet type.
    ///
    /// [Source Description]: https://tools.ietf.org/html/rfc3550#section-6.5
    SourceDescription = 202,

    /// The [Goodbye] (BYE) RTCP packet type.
    ///
    /// [Source Description]: https://tools.ietf.org/html/rfc3550#section-6.6
    Goodbye = 203,

    /// The [Application Defined] (APP) RTCP packet type.
    ///
    /// [Application Defined]: https://tools.ietf.org/html/rfc3550#section-6.7
    ApplicationDefined = 204,

    /// The [Transport Specific Feedback] RTCP packet type.
    ///
    /// [Transport Specific Feedback]: https://tools.ietf.org/html/rfc4585#section-6.2
    TransportSpecificFeedback = 205,

    /// The [Payload Specific Feedback] RTCP packet type.
    ///
    /// [Payload Specific Feedback]: https://tools.ietf.org/html/rfc4585#section-6.3
    PayloadSpecificFeedback = 206,

    /// This value is used when the RTCP packet type is not known.
    Unknown = 0,
}

impl Eq for PacketType {}

impl PartialEq for PacketType {
    fn eq(&self, other: &Self) -> bool {
        *self as u8 == *other as u8
    }
}

impl From<u8> for PacketType {
    fn from(byte: u8) -> Self {
        match byte {
            200 => Self::SenderReport,
            201 => Self::ReceiverReport,
            202 => Self::SourceDescription,
            203 => Self::Goodbye,
            204 => Self::ApplicationDefined,
            205 => Self::TransportSpecificFeedback,
            206 => Self::PayloadSpecificFeedback,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for PacketType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::ApplicationDefined => "APP",
                Self::Goodbye => "BYE",
                Self::PayloadSpecificFeedback => "PSFB",
                Self::ReceiverReport => "RR",
                Self::SourceDescription => "SD",
                Self::SenderReport => "SR",
                Self::TransportSpecificFeedback => "TSFB",
                Self::Unknown => "UNKNOWN",
            }
        )
    }
}

/// Represents the header of a RTCP packet which are the 4 first bytes
/// of a marshalled RTCP packet.
///
/// As defined in [RFC 3550], this structure follows this data wire:
///
/// ```text
///  0               1               2               3
///  0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |V=2|P|    RC   |      PT       |             length            |
/// +===+===========+===============+===============================+
/// |                       report data blocks                      |
/// |                               ...                             |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
///
/// ## Legends
///
/// - `V` the `version` encoded on 2-bits word
/// - `P` the `padding` bit
/// - `RC` the `report_count` encoded on 5-bits word
/// - `PT` the `packet_type` field
#[derive(Clone, Copy, Debug)]
pub struct Header {
    /// The `padding` field indicates if the current RTCP packet contains
    /// some additional padding bytes at the end which are not part of the
    /// control information but are included in the `length` field.
    pub padding: bool,

    /// The `report_count` represents the number of report blocks included
    /// contained in the RTCP packet. Zero is a valid value for this field.
    pub report_count: u8,

    /// The `paacket_type` is the constant defining the RTCP packet type.
    /// You may see at `PacketType` enum for further information.
    pub packet_type: PacketType,

    /// The `length` field is the length of RTCP packet, padding bytes included
    /// if `padding` value is `true`.
    pub length: u16,
}

impl Header {
    /// Extracts and parses a RTCP packet header from a marshalled RTCP packet.
    pub fn from_raw(raw_packet: &[u8]) -> Result<Self, ()> {
        if raw_packet.len() < HEADER_LENGTH {
            return Err(());
        }

        // Checking RTCP version used
        let version = (raw_packet[0] >> VERSION_SHIFT) & VERSION_MASK;
        if version != RTCP_VERSION {
            return Err(());
        }

        // Decoding the first byte of the packet's header
        let padding = ((raw_packet[0] >> PADDING_SHIFT) & PADDING_MASK) > 0;
        let report_count = raw_packet[0] & COUNT_MASK;

        // Decoding the second byte of the packet's header
        let packet_type = PacketType::from(raw_packet[1]);

        // Decoding the length from 2 last bytes of the packet's header
        let length = [raw_packet[2], raw_packet[3]];
        let length = u16::from_be_bytes(length);

        Ok(Self {
            padding,
            report_count,
            packet_type,
            length,
        })
    }

    /// Exports the current header into a marshalled RTCP packet header.
    pub fn to_raw(&self) -> Result<Vec<u8>, ()> {
        if self.report_count > 31 {
            return Err(());
        }

        let mut output = Vec::with_capacity(4);
        for _ in 0..4 {
            output.push(0u8);
        }

        output[0] |= RTCP_VERSION << VERSION_SHIFT;

        if self.padding {
            output[0] |= 1 << PADDING_SHIFT;
        }

        output[0] |= self.report_count;
        output[1] |= self.packet_type as u8;
        output[2..].copy_from_slice(&self.length.to_be_bytes());

        Ok(output)
    }
}

impl Eq for Header {}

impl PartialEq for Header {
    fn eq(&self, other: &Self) -> bool {
        self.padding == other.padding
            && self.report_count == other.report_count
            && self.packet_type == other.packet_type
            && self.length == other.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_instanciates_a_rtcp_packet_header_from_marshalled_packet() {
        let raw = [0x81u8, 0xc9u8, 0x00u8, 0x07u8];

        let header = Header::from_raw(&raw);
        assert!(header.is_ok());

        let header = header.unwrap();
        assert_eq!(
            Header {
                padding: false,
                report_count: 1,
                packet_type: PacketType::ReceiverReport,
                length: 7,
            },
            header
        );
    }

    #[test]
    fn it_returns_an_error_when_version_is_not_correct() {
        let raw = [0x00u8, 0xc9u8, 0x00u8, 0x07u8];

        let header = Header::from_raw(&raw);
        assert!(header.is_err());
    }

    #[test]
    fn it_exports_a_rtcp_packet_header_into_a_marshalled_packet() {
        let header = Header {
            padding: false,
            report_count: 1,
            packet_type: PacketType::ReceiverReport,
            length: 7,
        };

        let raw = header.to_raw();
        assert!(raw.is_ok());

        let raw = raw.unwrap();
        assert_eq!(vec![0x81u8, 0xc9, 0x00u8, 0x07u8], raw);
    }
}
