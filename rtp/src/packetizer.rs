use crate::{packet, Packet, PayloadGenerator, Sequencer};
use chrono::Local;
use rand::Rng;

pub type G711Packetizer = Packetizer<crate::codecs::g711::G711PayloadGenerator>;
pub type G722Packetizer = Packetizer<crate::codecs::g722::G722PayloadGenerator>;
pub type H264Packetizer = Packetizer<crate::codecs::h264::H264PayloadGenerator>;
pub type OpusPacketizer = Packetizer<crate::codecs::opus::OpusPayloadGenerator>;
pub type VP8Packetizer = Packetizer<crate::codecs::vp8::VP8PayloadGenerator>;
pub type VP9Packetizer = Packetizer<crate::codecs::vp9::VP9PayloadGenerator>;

/// List of extension numbers to add to the extension profile of a RTP packet.
#[derive(Clone, Copy, Debug)]
pub enum ExtensionNumber {
    /// AbsSendTime extension, see http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
    AbsSendTime(u32),

    /// Unknown extension
    Unknown,
}

/// This structure is responsible to packetize payloads that need
/// to be transmited through an RTP channel.
#[derive(Clone, Debug)]
pub struct Packetizer<G: PayloadGenerator + Default> {
    /// The Maximum Transmission Unit (MTU) used by the packetizer to
    /// generates packets.
    pub mtu: usize,

    /// The RTP payload type of the packets generated by the packetizer.
    pub payload_type: u8,

    /// The synchronization source (SSRC) identifier which emitting the
    /// packets.
    pub synchronization_source: u32,

    timestamp: u32,
    extensions: Vec<ExtensionNumber>,
    generator: G,
    sequencer: Sequencer,
}

impl<G> Packetizer<G>
where
    G: PayloadGenerator + Default,
{
    /// Instanciates a new instance of a packetizer with its parameters.
    pub fn new(mtu: usize, payload_type: u8, ssrc: u32) -> Self {
        Self {
            mtu,
            payload_type,
            synchronization_source: ssrc,
            timestamp: rand::thread_rng().gen(),
            extensions: Vec::new(),
            generator: G::default(),
            sequencer: Sequencer::new(),
        }
    }

    /// Transforms the data in a codecs format into a list of RTP packets.
    ///
    /// The data must be in the codec supported by the generator you've
    /// specified when you've instanciated the packetizer.
    pub fn packetize(&mut self, data: &[u8], samples: u32) -> Option<Vec<Packet>> {
        if data.len() == 0 || self.mtu <= packet::HEADER_SIZE {
            return None;
        }

        // Trying to retrieve RTP packets' payloads
        let payloads = self
            .generator
            .generate(self.mtu - packet::HEADER_SIZE, data);
        if let None = payloads {
            return None;
        }

        // Transforming RTP payloads into RTP packets
        let payloads = payloads.unwrap();
        if payloads.len() == 0 {
            return None;
        }

        let abs_send_time = self.extensions.iter().fold(None, |current, extension| {
            if let ExtensionNumber::AbsSendTime(value) = &extension {
                Some(*value)
            } else {
                current
            }
        });

        let packets: Vec<Packet> = payloads
            .iter()
            .enumerate()
            .map(|(index, payload)| {
                let mut extension = false;
                let mut extension_profile = None;
                let mut extension_payload = None;
                let marker = payload.len() - 1 == index;

                if marker {
                    if let Some(abs_send_time) = &abs_send_time {
                        let time = get_ntp_time();

                        extension = true;
                        extension_profile = Some(0xbede);
                        extension_payload = Some(vec![
                            ((*abs_send_time << 4) | 2) as u8,
                            (time & 0xff0000 >> 16) as u8,
                            (time & 0xff00 >> 8) as u8,
                            (time & 0xff) as u8,
                        ]);
                    }
                }

                Packet {
                    version: packet::PACKET_VERSION,
                    padding: false,
                    extension,
                    marker,
                    payload_type: self.payload_type,
                    sequence_number: self.sequencer.next_sequence_number(),
                    timestamp: self.timestamp,
                    ssrc: self.synchronization_source,
                    csrc: Vec::new(),
                    extension_profile,
                    extension_payload,
                    payload_offset: packet::HEADER_SIZE,
                    payload: Vec::from(&payload[..]),
                    raw: None,
                }
            })
            .collect();

        // Refreshing internal timestamp
        self.timestamp += samples;

        Some(packets)
    }

    /// Adds an extension number to the packetizer instance.
    pub fn add_extension_number(&mut self, extension: ExtensionNumber) {
        self.extensions.push(extension);
    }
}

/// Converts an Unix epoch, in nanoseconds, into a NTP time.
fn get_ntp_time() -> u64 {
    let epoch = Local::now().timestamp_nanos() as u64;

    let s = epoch / 1_000_000_000;
    let s = s + 0x83aa7e80;
    let s = s << 32;

    let f = epoch % 1_000_000_000;
    let f = f << 32;
    let f = f / 1_000_000_000;

    s | f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_packetizes_arbitrary_data() {
        // We'll use the G722 codecs here because it's simple and accepts an array filled of zeros
        // as valid data.
        let mut packetizer = G722Packetizer::new(100, 98, 0x1234abcd);
        let data = [0u8; 128];

        let packets = packetizer.packetize(&data, 2000);
        assert!(packets.is_some());

        let packets = packets.unwrap();
        assert_eq!(2, packets.len());
    }

    #[test]
    fn it_returns_none_when_mtu_is_too_small() {
        let mut packetizer = G722Packetizer::new(5, 98, 0x1234abcd);
        let data = [0u8; 128];

        let packets = packetizer.packetize(&data, 2000);
        assert!(packets.is_none());
    }

    #[test]
    fn it_returns_none_when_there_is_no_data_to_packetize() {
        let mut packetizer = G722Packetizer::new(100, 98, 0x1234abcd);

        let packets = packetizer.packetize(&[], 2000);
        assert!(packets.is_none());
    }
}