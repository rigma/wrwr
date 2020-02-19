use rand::Rng;

/// This structure is responsible to generate sequence numbers used to
/// with RTP packets during a transmission.
///
/// The initial sequence number is randomly generated and then it's
/// incremented each time a new sequence number is requested. Therefore,
/// this structure needs to be mutable in order to update its internal
/// state.
#[derive(Copy, Clone, Debug)]
pub struct Sequencer {
    sequence_number: u16,
    roll_over_count: u64,
}

impl Sequencer {
    /// Instanciates a new RTP packet sequencer which is responsible to
    /// generate sequence numbers for packets.
    pub fn new() -> Self {
        Self {
            sequence_number: rand::thread_rng().gen(),
            roll_over_count: 0,
        }
    }

    /// Generates the next sequence number to use with the RTP stream
    pub fn next_sequence_number(&mut self) -> u16 {
        self.sequence_number += 1;
        if self.sequence_number == 0 {
            self.roll_over_count += 1;
        }

        self.sequence_number
    }

    /// Retrieves the number of times where the sequence number has
    /// overflowed.
    pub fn roll_over_count(&self) -> u64 {
        self.roll_over_count
    }
}

impl Default for Sequencer {
    fn default() -> Self {
        Self::new()
    }
}
