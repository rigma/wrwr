/// Total length of the RTP packet's header in bytes
const HEADER_LENGHT: usize = 4;

/// The left shift to apply to the header's first byte to get packet's version
const VERSION_SHIFT: usize = 6;

/// The bitmask to use to retrieve the version
const VERSION_MASK: u8 = 0x03;

/// The left shift to apply to the header's first byte to get packet's padding
const PADDING_SHIFT: usize = 5;

/// The bitmask to use to retrieve the padding
const PADDING_MASK: u8 = 0x1;

/// The left shift to apply to the header's first byte to get packet's extension
const EXTENSION_SHIFT: usize = 4;

/// The bitmask to use to retrieve the extension
const EXTENSION_MASK: u8 = 0x01;

/// The bitmask to use to retrieve the CSRC field capacity
const CC_MASK: u8 = 0x0f;

/// The left shift to apply to the header's first byte to get packet's marker
const MARKER_SHIFT: usize = 7;

/// The bitmask to use to retrieve the marker
const MARKER_MASK: u8 = 0x01;

/// The bitmask to use to retrieve the payload type
const PAYLOAD_TYPE_MASK: u8 = 0x7f;

/// The offset in the raw RTP packet to use in order to find the packet's sequence number
const SEQ_NUM_OFFSET: usize = 2;

/// The offset in the raw RTP packet to use in order to find the packet's timestamp
const TIMESTAMP_OFFSET: usize = 4;

/// The offset in the raw RTP packet to use in order to find the packet's SSRC
const SSRC_OFFSET: usize = 8;

/// The offset in the raw RTP packet to use in order to find the packet's CSRC
const CSRC_OFFSET: usize = 12;

/// The length of the CSRC field in a raw RTP packet
const CSRC_LENGTH: usize = 4;

/// Represents a parsed RTP packet into a Rusty representation. This data structure
/// follows the [RFC 3550] specification.
///
/// A RTP packet follows the following wire format:
///
/// ```text
///  0               1               2               3
///  0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |V=2|P|X|  CC   |M|     PT      |       sequence number         |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                           timestamp                           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |           synchronization source (SSRC) identifier            |
/// +=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
/// |            contributing source (CSRC) identifiers             |
/// |                             ....                              |
/// +=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
/// |                    optional header extension                  |
/// +=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
/// |      defined by profile       |           length              |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                        header extension                       |
/// |                             ....                              |
/// +=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
/// |                         packet payload                        |
/// +=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
/// |                            payload                            |
/// |                              ....                             |
/// +=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
/// ```
///
/// ## Legends
///
/// - `V`: the Version of the packet (it should be `2`)
/// - `P`: the Padding
/// - `X`: the eXtension bit
/// - `CC`: the number of contributing source identifiers
/// - `M`: the Marker bit
/// - `PT`: the Payload Type
///
/// [RFC 3550]: https://tools.ietf.org/html/rfc3550
#[derive(Clone, Debug, Default)]
pub struct Packet {
    /// This `version` field indicates the version of the current RTP packet.
    ///
    /// Since [RFC 3550] the version to `2`. The value `1` is mainly used for
    /// the draft version of RTP protocol and value `0` is used in the _vat_
    /// audio tool.
    ///
    /// [RFC 3550]: https://tools.ietf.org/html/rfc3550
    pub version: u16,

    /// The `padding` field is responsible to indicates if the packet contains
    /// one or more additionnal padding bytes at the end which are not part of
    /// the payload.
    ///
    /// The last byte of the padding contains a count of how maany padding bytes
    /// should be ignored, including istself. `padding` may be needed by some
    /// encryption algorithms with fixed block sizes or to carry several RTP
    /// packets in lower-layer protocol data unit.
    pub padding: bool,

    /// The `extension` field indicates if the current RTP packet contains a
    /// header extension. If so, it has to follow the fixed RTP header as defined
    /// in the [Section 5.3.1] of the [RFC 3550].
    ///
    /// [Section 5.3.1]: https://tools.ietf.org/html/rfc3550#section-5.3.1
    /// [RFC 3550]: https://tools.ietf.org/html/rfc3550#section-5.3.1
    pub extension: bool,

    /// The `marker` field is used differently according to a profile.
    ///
    /// It is intended to allow to signifiant events such as frame boundaries to
    /// be marked in the packet stream. A profile **may** define additional marker
    /// bits or specify that there is no marker bit by changing the number of bits
    /// in the `payload_type` field.
    pub marker: bool,

    /// The `payload_type` indicates the format of the payload of the current RTP
    /// packet.
    ///
    /// Its values **may** be specified by a profile.
    pub payload_type: u8,

    /// The `sequence_number` field is a value which is incremented one by one for
    /// each RTP packet. The initial value of the field should be random to make
    /// known-plaintext attacks on encryption more difficult.
    ///
    /// `sequence_number` may be used by the receiver to detect packet loss and/or
    /// to restore packet sequence.
    pub sequence_number: u16,

    /// The `timestamp` field is the sampling instant of the first byte in the RTP
    /// data packet.
    ///
    /// This field **must** be derivated from a clock that increments monotonically
    /// and linearly in time to allow synchronization and jitter calculations.
    /// Furthermore, the resolution of the clock **must** be sufficient for the
    /// desired synchronization accuracy and for measuring packet arrival jitter.
    ///
    /// For further information, you can check the [specification] of this field in
    /// [RFC 3550]
    ///
    /// [specification]: https://tools.ietf.org/html/rfc3550#section-5.1
    /// [RFC 3550]: https://tools.ietf.org/html/rfc3550
    pub timestamp: u32,

    /// The `ssrc` field identifies the synchronization source of the current packet.
    ///
    /// It **should** be chosen randomly with the intent that no two synchronization
    /// sources within the same RTP session will have the same `ssrc` indentifier.
    pub ssrc: u32,

    /// The `csrc` vector is a list of 0 to 15 32 bits words which identifies the
    /// contributing sources for the packet's payload. The number of contributing
    /// sources is given by the CC field in the raw RTP packet.
    pub csrc: Vec<u32>,

    /// The `extension_profile` defines an optional extension header which follow
    /// the fixed size one in the raw packet.
    pub extension_profile: Option<u16>,

    /// the `extension_payload` vector is the content of the extension header if it
    /// is provided within the raw packet.
    pub extension_payload: Option<Vec<u8>>,

    /// Indicates the offset to use in the raw packet to access to the packet's
    /// payload and skip its header.
    pub payload_offset: usize,

    /// `payload` represents the content of the current RTP packet
    pub payload: Vec<u8>,

    /// The raw representation of the current RTP packet (headers + payload)
    pub raw: Vec<u8>,
}

impl Packet {
    /// Instanciates an RTP packet from a raw byte's array. If the method is
    /// not able to instanciates the packet, this function will return an error.
    pub fn from_raw(raw_packet: &[u8]) -> Result<Self, ()> {
        // If the packet lenght is lesser than the header's length, we return an error
        if raw_packet.len() < HEADER_LENGHT {
            return Err(());
        }

        // Decoding the first byte of the packet (version, padding, extension and CC count)
        let version: u16 = ((raw_packet[0] >> VERSION_SHIFT) & VERSION_MASK) as u16;
        let padding = (raw_packet[0] >> PADDING_SHIFT) & PADDING_MASK > 0;
        let extension = (raw_packet[0] >> EXTENSION_SHIFT) & EXTENSION_MASK > 0;
        let cc = (raw_packet[0] & CC_MASK) as usize;

        // Decoding the second byte of the packet (marker, payload type)
        let marker = (raw_packet[1] >> MARKER_SHIFT) & MARKER_MASK > 0;
        let payload_type = raw_packet[1] & PAYLOAD_TYPE_MASK;

        // Decoding the sequence number
        let sequence_number: u16 =
            (raw_packet[SEQ_NUM_OFFSET] as u16) << 8 | raw_packet[SEQ_NUM_OFFSET + 1] as u16;
        let sequence_number = sequence_number.to_be();

        // Decoding the timestamp
        let timestamp: u32 = (raw_packet[TIMESTAMP_OFFSET] as u32) << 24
            | (raw_packet[TIMESTAMP_OFFSET + 1] as u32) << 16
            | (raw_packet[TIMESTAMP_OFFSET + 2] as u32) << 8
            | (raw_packet[SSRC_OFFSET + 3] as u32);
        let timestamp = timestamp.to_be();

        // Decoding the synchronization source
        let ssrc: u32 = (raw_packet[SSRC_OFFSET] as u32) << 24
            | (raw_packet[SSRC_OFFSET + 1] as u32) << 16
            | (raw_packet[SSRC_OFFSET + 2] as u32) << 8
            | (raw_packet[SSRC_OFFSET + 3] as u32);
        let ssrc = ssrc.to_be();

        // Computing the current payload offset
        let mut payload_offset = CSRC_OFFSET + CSRC_LENGTH * cc;
        if raw_packet.len() < payload_offset {
            return Err(());
        }

        // Decoding the contributing source identifiers
        let csrc = (0..cc)
            .map(|i| {
                let offset = CSRC_OFFSET + CSRC_LENGTH * i;

                (raw_packet[offset] as u32) << 24
                    | (raw_packet[offset + 1] as u32) << 16
                    | (raw_packet[offset + 2] as u32) << 8
                    | (raw_packet[offset + 3] as u32)
            })
            .collect();

        // Checking if the header do have an extension
        let mut extension_profile: Option<u16> = None;
        let mut extension_payload: Option<Vec<u8>> = None;

        if extension {
            // If yes, we'll decode its profile and its payload
            if raw_packet.len() < payload_offset + 4 {
                return Err(());
            }

            extension_profile = Some(
                (raw_packet[payload_offset] as u16) << 8 | (raw_packet[payload_offset + 1] as u16),
            );
            payload_offset += 2;

            let extension_length: usize = 4
                * ((raw_packet[payload_offset] as usize) << 8
                    | (raw_packet[payload_offset + 1] as usize));
            payload_offset += 2;

            if raw_packet.len() < payload_offset + extension_length {
                return Err(());
            }

            extension_payload = Some(Vec::from(
                &raw_packet[payload_offset..payload_offset + extension_length],
            ));
            payload_offset += extension_length;
        }

        // Retrieving payload and saving raw packet into a new vector
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
        assert!(Packet::from_raw(&vec![]).is_err());
    }

    #[test]
    fn it_unmarshall_a_packet_without_extension() {
        let raw_packet: [u8; 25] = [
            0x90, 0xe0, 0x69, 0x8f, 0xd9, 0xc2, 0x93, 0xda, 0x1c, 0x64, 0x27, 0x82, 0x00, 0x01,
            0x00, 0x01, 0xff, 0xff, 0xff, 0xff, 0x98, 0x36, 0xbe, 0x88, 0x9e,
        ];

        assert!(Packet::from_raw(&raw_packet).is_ok());
    }
}
