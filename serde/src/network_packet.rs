/// An enumeration of possible scalar data type in a packet data wire.
#[derive(Clone, Copy, Debug)]
pub enum WireType {
    Integer(usize),
    UnsignedInteger(usize),
    Float(usize),
    Char,
    String,
    Unknown,
}

impl Default for WireType {
    fn default() -> Self {
        Self::Unknown
    }
}

/// A wire is a component of a packet data wire.
///
/// It contains its associated name, the wire type, the offset inside the network packet
/// and the length of the data in bytes.
#[derive(Clone, Debug, Default)]
pub struct Wire {
    /// The name of the wire
    pub name: String,

    /// The type of the wire
    pub wire_type: WireType,

    /// The offset of the wire inside the network packet.
    pub offset: usize,

    /// The length of the wire inside the network packet.
    pub len: usize,
}

/// A trait to help the WRWR to serialize and deserializes marshalled RTP packets.
pub trait NetworkPacket {
    fn data_wire() -> Vec<Wire>;
}
