/// This enumeration is exposing the errors that can occurs during RTP packet
/// marshalling or unmarshalling process.
#[derive(Debug, Fail)]
pub enum RtpPacketError {
    /// Emitted when the RTP version used to encode the packet is not supported.
    #[fail(display = "Invalid RTP version used: {}", version)]
    InvalidRtpVersion { version: u8 },

    /// Emitted when the RTP header extension has not a payload made of 4-byte words.
    #[fail(display = "Invalid RTP header extension provided: {}", length)]
    InvalidRtpHeaderExtension { length: usize },

    /// Emitted when the marshalled RTP packet is not a valid one. Either it's too
    /// small to contain a header, either it's too small to contain a payload.
    #[fail(display = "Provided marshalled RTP packet is not valid")]
    InvalidRtpPacket,
}
