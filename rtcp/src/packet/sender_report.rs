use crate::{
    packet::{
        header::{self, Header, PacketType},
        Packet,
    },
    util::{ReceptionReport, RECEPTION_REPORT_LENGTH},
};

const SR_HEADER_LENGTH: usize = 24;
const SSRC_OFFSET: usize = 4;
const NTP_OFFSET: usize = 8;
const RTP_OFFSET: usize = 16;
const PACKET_COUNT_OFFSET: usize = 20;
const BYTE_COUNT_OFFSET: usize = 24;
const REPORTS_OFFSET: usize = 28;

#[derive(Debug)]
pub struct SenderReport {
    pub header: Header,
    pub synchronization_source: u32,
    pub ntp_time: u64,
    pub rtp_time: u32,
    pub packet_count: u32,
    pub byte_count: u32,
    pub reports: Vec<ReceptionReport>,
    pub profile_extensions: Option<Vec<u8>>,
}

impl Packet for SenderReport {
    fn from_raw(raw_packet: &[u8]) -> Result<Self, ()> {
        use std::mem::size_of;

        if raw_packet.len() < header::HEADER_LENGTH + SR_HEADER_LENGTH {
            return Err(());
        }

        let header = Header::from_raw(raw_packet)?;
        if header.packet_type != PacketType::SenderReport {
            return Err(());
        }

        let mut synchronization_source = [0u8; size_of::<u32>()];
        synchronization_source.copy_from_slice(&raw_packet[SSRC_OFFSET..NTP_OFFSET]);
        let synchronization_source = u32::from_be_bytes(synchronization_source);

        let mut ntp_time = [0u8; size_of::<u64>()];
        ntp_time.copy_from_slice(&raw_packet[NTP_OFFSET..RTP_OFFSET]);
        let ntp_time = u64::from_be_bytes(ntp_time);

        let mut rtp_time = [0u8; size_of::<u32>()];
        rtp_time.copy_from_slice(&raw_packet[RTP_OFFSET..PACKET_COUNT_OFFSET]);
        let rtp_time = u32::from_be_bytes(rtp_time);

        let mut packet_count = [0u8; size_of::<u32>()];
        packet_count.copy_from_slice(&raw_packet[PACKET_COUNT_OFFSET..BYTE_COUNT_OFFSET]);
        let packet_count = u32::from_be_bytes(packet_count);

        let mut byte_count = [0u8; size_of::<u32>()];
        byte_count.copy_from_slice(&raw_packet[BYTE_COUNT_OFFSET..REPORTS_OFFSET]);
        let byte_count = u32::from_be_bytes(byte_count);

        let mut reports = Vec::with_capacity(header.report_count as usize);
        if header.report_count > 0 {
            for i in 0..header.report_count as usize {
                let offset = REPORTS_OFFSET + RECEPTION_REPORT_LENGTH * i;
                if offset > raw_packet.len() {
                    return Err(());
                }

                if let Ok(report) =
                    ReceptionReport::from_raw(&raw_packet[offset..offset + RECEPTION_REPORT_LENGTH])
                {
                    reports.push(report);
                } else {
                    return Err(());
                }
            }
        }

        if reports.len() != header.report_count as usize {
            return Err(());
        }

        let profile_extensions = if (REPORTS_OFFSET
            + RECEPTION_REPORT_LENGTH * header.report_count as usize)
            < raw_packet.len()
        {
            Some(Vec::from(
                &raw_packet
                    [REPORTS_OFFSET + RECEPTION_REPORT_LENGTH * header.report_count as usize..],
            ))
        } else {
            None
        };

        Ok(Self {
            header,
            synchronization_source,
            ntp_time,
            rtp_time,
            packet_count,
            byte_count,
            reports,
            profile_extensions,
        })
    }

    fn to_raw(&self) -> Result<Vec<u8>, ()> {
        let mut output = Vec::with_capacity(self.length());
        for _ in 0..output.len() {
            output.push(0u8);
        }

        output[..SSRC_OFFSET].copy_from_slice(&self.header.to_raw()?);
        output[SSRC_OFFSET..NTP_OFFSET].copy_from_slice(&self.synchronization_source.to_be_bytes());
        output[NTP_OFFSET..RTP_OFFSET].copy_from_slice(&self.ntp_time.to_be_bytes());
        output[RTP_OFFSET..PACKET_COUNT_OFFSET].copy_from_slice(&self.packet_count.to_be_bytes());
        output[BYTE_COUNT_OFFSET..REPORTS_OFFSET].copy_from_slice(&self.byte_count.to_be_bytes());

        if self.reports.len() > 0x1f {
            return Err(());
        }

        for (i, report) in self.reports.iter().enumerate() {
            let offset = REPORTS_OFFSET + RECEPTION_REPORT_LENGTH * i;
            let export = report.to_raw()?;

            output[offset..offset + export.len()].copy_from_slice(&export);
        }

        if let Some(profile_extensions) = &self.profile_extensions {
            let offset = REPORTS_OFFSET + RECEPTION_REPORT_LENGTH * self.reports.len();
            output[offset..].copy_from_slice(profile_extensions);
        }

        Ok(output)
    }

    fn length(&self) -> usize {
        let mut length =
            header::HEADER_LENGTH + SR_HEADER_LENGTH + self.reports.len() * RECEPTION_REPORT_LENGTH;
        if let Some(profile_extensions) = &self.profile_extensions {
            length += profile_extensions.len();
        }

        length
    }

    fn synchronization_sources(&self) -> Vec<u32> {
        self.reports
            .iter()
            .map(|report| report.synchronization_source)
            .collect()
    }
}

impl Eq for SenderReport {}

impl PartialEq for SenderReport {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.synchronization_source == other.synchronization_source
            && self.ntp_time == other.ntp_time
            && self.rtp_time == other.rtp_time
            && self.packet_count == other.packet_count
            && self.byte_count == other.byte_count
            && self.reports == other.reports
            && self.profile_extensions == other.profile_extensions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_unmarshalls_a_sender_report_packet() {
        let raw = [
            // v=2, p=0, count=1, SR, len=7
            0x81, 0xc8, 0x0, 0x7, // ssrc=0x902f9e2e
            0x90, 0x2f, 0x9e, 0x2e, // ntp=0xda8bd1fcdddda05a
            0xda, 0x8b, 0xd1, 0xfc, 0xdd, 0xdd, 0xa0, 0x5a, // rtp=0xaaf4edd5
            0xaa, 0xf4, 0xed, 0xd5, // packetCount=1
            0x00, 0x00, 0x00, 0x01, // octetCount=2
            0x00, 0x00, 0x00, 0x02, // ssrc=0xbc5e9a40
            0xbc, 0x5e, 0x9a, 0x40, // fracLost=0, totalLost=0
            0x0, 0x0, 0x0, 0x0, // lastSeq=0x46e1
            0x0, 0x0, 0x46, 0xe1, // jitter=273
            0x0, 0x0, 0x1, 0x11, // lsr=0x9f36432
            0x9, 0xf3, 0x64, 0x32, // delay=150137
            0x0, 0x2, 0x4a, 0x79,
        ];
        let expected = SenderReport {
            header: Header {
                padding: false,
                report_count: 1,
                packet_type: PacketType::SenderReport,
                length: 7,
            },
            synchronization_source: 0x902f9e2e,
            ntp_time: 0xda8bd1fcdddda05a,
            rtp_time: 0xaaf4edd5,
            byte_count: 2,
            packet_count: 1,
            reports: vec![ReceptionReport {
                synchronization_source: 0xbc5e9a40,
                fraction_lost: 0,
                total_lost: 0,
                last_sequence_number: 0x46e1,
                jitter: 273,
                last_sender_report: 0x9f36432,
                delay: 150137,
            }],
            profile_extensions: None,
        };
        let packet = SenderReport::from_raw(&raw);
        assert!(packet.is_ok());

        let packet = packet.unwrap();
        assert_eq!(packet, expected);
    }
}
