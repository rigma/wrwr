use chrono::Local;

/// This structure is responsible to packetize payloads that need
/// to be transmited through an RTP channel.
#[derive(Clone, Copy, Debug, Default)]
pub struct Packetizer;

impl Packetizer {
    /// Retrieves the current Unix timestamp of the local system
    /// into its NTP representation.
    pub fn ntp_time(&self) -> u64 {
        to_ntp_time(Local::now().timestamp_nanos() as u64)
    }
}

/// Converts an Unix epoch, in nanoseconds, into a NTP time.
fn to_ntp_time(unix_epoch: u64) -> u64 {
    let s = unix_epoch / 1_000_000_000;
    let s = s + 0x83aa7e80;
    let s = s << 32;

    let f = unix_epoch % 1_000_000_000;
    let f = f << 32;
    let f = f / 1_000_000_000;

    s | f
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, TimeZone};

    #[test]
    fn it_converts_an_epoch_into_a_ntp_time() {
        let time = FixedOffset::west(5 * 3600)
            .ymd(1985, 6, 23)
            .and_hms(4, 0, 0);
        assert_eq!(
            to_ntp_time(time.timestamp_nanos() as u64),
            0xa0c65b1000000000
        );

        let time = Local.ymd(2020, 1, 28).and_hms(11, 34, 23);
        assert_eq!(
            to_ntp_time(time.timestamp_nanos() as u64),
            0xe1da8caf00000000
        );
    }
}
