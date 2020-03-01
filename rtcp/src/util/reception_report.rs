pub const RECEPTION_REPORT_LENGTH: usize = 24;

const FRACTION_LOST_OFFSET: usize = 4;
const TOTAL_LOST_OFFSET: usize = 5;
const LAST_SEQUENCE_OFFSET: usize = 8;
const JITTER_OFFSET: usize = 12;
const LAST_SENDER_REPORT_OFFSET: usize = 16;
const DELAY_OFFSET: usize = 20;

/// Represents a reception report which his a component of a RTCP report.
#[derive(Debug, Default)]
pub struct ReceptionReport {
    /// This field contains the synchronization source identifier
    /// of the source which the information in this reception report
    /// block partains.
    pub synchronization_source: u32,

    /// Indicates the fraction of RTP data packets from the source
    /// lost since the previous sender report or receiver report was
    /// sent, expressed as a fixed point number with the binary point
    /// at the left edge of the field.
    pub fraction_lost: u8,

    /// Indicates the total number of RTP data packets from the source
    /// that have been lost since the beginning of the communication.
    pub total_lost: u32,

    /// The latest sequence number received during the communication.
    ///
    /// The low 16 bits contains the highest sequence number received in
    /// a RTP data packet from source, and the most significant 16 bits
    /// extend that sequence number with the corresponding count of
    /// sequence number cycles.
    pub last_sequence_number: u32,

    /// An estimate of the stastical variance of RTP data packet interarrival
    /// time, measured in timestamp units and expressed as an unsigned integer.
    pub jitter: u32,

    /// The 32-bit word in the middle of a 64 bits NTP timestamp received
    /// as part of the most recent RTCP sender report packet from source. If no
    /// sender report has been received yet, this field is set to zero.
    pub last_sender_report: u32,

    /// The delay, expressed in units of 1/65536 seconds, between receiving the
    /// last sender report packet from source which is sending this reception
    /// report block. If no sender report has been received yet, this field is
    /// set to zero.
    pub delay: u32,
}

impl ReceptionReport {
    /// Parses a marshalled RTCP data packet to extract and instanciates an instance
    /// of `ReceptionReport`.
    pub fn from_raw(raw_packet: &[u8]) -> Result<Self, ()> {
        if raw_packet.len() < RECEPTION_REPORT_LENGTH {
            return Err(());
        }

        let synchronization_source = [raw_packet[0], raw_packet[1], raw_packet[2], raw_packet[3]];
        let synchronization_source = u32::from_be_bytes(synchronization_source);

        let fraction_lost = raw_packet[FRACTION_LOST_OFFSET];

        let total_lost = raw_packet[TOTAL_LOST_OFFSET + 2] as u32
            | (raw_packet[TOTAL_LOST_OFFSET + 1] as u32) << 8
            | (raw_packet[TOTAL_LOST_OFFSET] as u32) << 16;

        let last_sequence_number = [
            raw_packet[LAST_SEQUENCE_OFFSET],
            raw_packet[LAST_SEQUENCE_OFFSET + 1],
            raw_packet[LAST_SEQUENCE_OFFSET + 2],
            raw_packet[LAST_SEQUENCE_OFFSET + 3],
        ];
        let last_sequence_number = u32::from_be_bytes(last_sequence_number);

        let jitter = [
            raw_packet[JITTER_OFFSET],
            raw_packet[JITTER_OFFSET + 1],
            raw_packet[JITTER_OFFSET + 2],
            raw_packet[JITTER_OFFSET + 3],
        ];
        let jitter = u32::from_be_bytes(jitter);

        let last_sender_report = [
            raw_packet[LAST_SENDER_REPORT_OFFSET],
            raw_packet[LAST_SENDER_REPORT_OFFSET + 1],
            raw_packet[LAST_SENDER_REPORT_OFFSET + 2],
            raw_packet[LAST_SENDER_REPORT_OFFSET + 3],
        ];
        let last_sender_report = u32::from_be_bytes(last_sender_report);

        let delay = [
            raw_packet[DELAY_OFFSET],
            raw_packet[DELAY_OFFSET + 1],
            raw_packet[DELAY_OFFSET + 2],
            raw_packet[DELAY_OFFSET + 3],
        ];
        let delay = u32::from_be_bytes(delay);

        Ok(Self {
            synchronization_source,
            fraction_lost,
            total_lost,
            last_sequence_number,
            jitter,
            last_sender_report,
            delay,
        })
    }

    /// Exports the current reception report instance into a marshalled
    /// representation.bool
    pub fn to_raw(&self) -> Result<Vec<u8>, ()> {
        if self.total_lost >= 1 << 25 {
            return Err(());
        }

        let mut output = Vec::with_capacity(RECEPTION_REPORT_LENGTH);
        for _ in 0..RECEPTION_REPORT_LENGTH {
            output.push(0u8);
        }

        output[0..FRACTION_LOST_OFFSET].copy_from_slice(&self.synchronization_source.to_be_bytes());
        output[FRACTION_LOST_OFFSET] = self.fraction_lost;

        for i in 0..3 {
            output[TOTAL_LOST_OFFSET + i] = (self.total_lost >> 8 * i) as u8;
        }


        output[LAST_SEQUENCE_OFFSET..JITTER_OFFSET].copy_from_slice(&self.last_sequence_number.to_be_bytes());
        output[JITTER_OFFSET..LAST_SENDER_REPORT_OFFSET].copy_from_slice(&self.jitter.to_be_bytes());
        output[LAST_SENDER_REPORT_OFFSET..DELAY_OFFSET].copy_from_slice(&self.last_sender_report.to_be_bytes());
        output[DELAY_OFFSET..].copy_from_slice(&self.delay.to_be_bytes());

        Ok(output)
    }
}

impl Eq for ReceptionReport {}

impl PartialEq for ReceptionReport {
    fn eq(&self, other: &Self) -> bool {
        self.synchronization_source == other.synchronization_source
            && self.fraction_lost == other.fraction_lost
            && self.total_lost == other.total_lost
            && self.last_sequence_number == other.last_sequence_number
            && self.jitter == other.jitter
            && self.last_sender_report == other.last_sender_report
            && self.delay == other.delay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_marshalled_reception_report() {
        let raw: [u8; RECEPTION_REPORT_LENGTH] = [
            0x00, 0x00, 0x12, 0x34,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xab, 0xcd,
            0x00, 0x00, 0x00, 0x12,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        let expected = ReceptionReport {
            synchronization_source: 0x1234,
            fraction_lost: 0,
            total_lost: 0,
            last_sequence_number: 0xabcd,
            jitter: 0x12,
            last_sender_report: 0x0,
            delay: 0x0,
        };

        let report = ReceptionReport::from_raw(&raw);
        assert!(report.is_ok());

        let report = report.unwrap();
        assert_eq!(expected, report);
    }

    #[test]
    fn it_exports_a_reception_report_into_a_marshalled_packet() {
        let packet = ReceptionReport {
            synchronization_source: 0x1234,
            fraction_lost: 0,
            total_lost: 0,
            last_sequence_number: 0xabcd,
            jitter: 0x12,
            last_sender_report: 0x0,
            delay: 0x0,
        };
        let expected: Vec<u8> = vec![
            0x00, 0x00, 0x12, 0x34,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xab, 0xcd,
            0x00, 0x00, 0x00, 0x12,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];

        let raw = packet.to_raw();
        assert!(raw.is_ok());

        let raw = raw.unwrap();
        assert_eq!(expected, raw);
    }
}
