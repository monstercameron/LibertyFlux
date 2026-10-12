//! Differential cases for the layout frame's record query.
//!
//! The lifted query and its verified rewrite run on the same generated frames
//! and record containers. The rewrite asks the frame, through its vtable, for
//! the container; the harness points that slot at a stub that answers the
//! container's address. The container and its records are memory the harness
//! owns: a count, an array of record addresses, and one flag byte per record.
//! The lifted query asks the same container through its host. Both sides must
//! produce the same answer, which includes the container's high bits, and the
//! same frame bytes. A deliberately wrong lift must differ from the rewrite.
//!
//! The container sits at an address whose low byte is not zero, so a wrong
//! answer that keeps the container's low byte is visible.

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::atomic::{AtomicU32, Ordering};

    use lf_input_frontend::layout_frame::records::RecordHost;
    use lf_input_frontend::layout_frame::{SPAN, UILayoutFrame};
    use lf_layoutframe_diff::rewrites as r;

    /// Bytes of the buffer the rewrite runs on: the frame's span and a tail
    /// that must come back unchanged.
    const BUF: usize = 0x100;

    /// Generated cases.
    const TRIALS: u32 = 3000;

    /// Words in the fake vtable: enough for the query's slot.
    const VTABLE_WORDS: usize = 0x100 / 4;

    /// Byte offset of the frame's record query in its vtable.
    const RECORD_QUERY_AT: usize = 0xF8;

    /// Byte offset of the container inside its block, chosen so that its
    /// address has a non-zero low byte.
    const CONTAINER_AT: usize = 0x31;

    /// Largest record count the cases draw. The counts 255, 256 and 257 are
    /// among the counts drawn, so the 8- and 16-bit edges are reached.
    const MAX_RECORDS: usize = 300;

    /// The container's address, answered by the query stub.
    static CONTAINER: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn query_stub(_this: u32) -> u32 {
        CONTAINER.load(Ordering::SeqCst)
    }

    /// The container and its records, owned by the harness for one trial.
    struct Records {
        _block: Vec<u8>,
        _array: Vec<u32>,
        flags: Vec<u8>,
        count: u16,
        container: u32,
    }

    impl Records {
        /// Lays out a container of `count` records with the given flags.
        fn new(count: u16, flags: Vec<u8>) -> Self {
            let mut block = vec![0u8; 0x80];
            let mut array = vec![0u32; MAX_RECORDS.max(usize::from(count))];
            let mut flags_block = flags;
            flags_block.resize(array.len(), 0);
            let container = block.as_ptr() as usize as u32 + CONTAINER_AT as u32;
            let array_base = array.as_mut_ptr() as usize as u32;
            for (index, slot) in array.iter_mut().enumerate() {
                *slot = flags_block.as_ptr() as usize as u32 + index as u32;
            }
            block[CONTAINER_AT..CONTAINER_AT + 4].copy_from_slice(&array_base.to_le_bytes());
            block[CONTAINER_AT + 4..CONTAINER_AT + 6].copy_from_slice(&count.to_le_bytes());
            Self { _block: block, _array: array, flags: flags_block, count, container }
        }
    }

    /// The host the lifted query is given: the container the harness laid
    /// out, and its record flags.
    struct Host<'a> {
        records: &'a Records,
    }

    impl RecordHost for Host<'_> {
        fn query_records(&mut self, _frame: &mut UILayoutFrame) -> u32 {
            self.records.container
        }

        fn record_count(&mut self, container: u32) -> u16 {
            assert_eq!(container, self.records.container, "the lift asked for another container");
            self.records.count
        }

        fn record_flags(&mut self, container: u32, index: usize) -> u8 {
            assert_eq!(container, self.records.container, "the lift asked for another container");
            self.records.flags[index]
        }
    }

    /// Small deterministic generator (splitmix64), seeded per case.
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }

        /// A record count: the edges 0, 1, 255, 256 and 257 about one time in
        /// two, otherwise any count up to the largest drawn.
        fn count(&mut self) -> u16 {
            const EDGE: [u16; 8] = [0, 1, 2, 3, 255, 256, 257, 300];
            if self.next() % 2 == 0 {
                EDGE[(self.next() % EDGE.len() as u64) as usize]
            } else {
                (self.next() % (MAX_RECORDS as u64 + 1)) as u16
            }
        }
    }

    fn seed_of(name: &str) -> u64 {
        let mut hash: u64 = 0xCBF2_9CE4_8422_2325;
        for byte in name.bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
        hash
    }

    /// The buffer the rewrite runs on, 4-byte aligned like the object.
    #[repr(C, align(4))]
    struct Buffer([u8; BUF]);

    /// The deliberate faults of the wrong lift: the flags tested in the
    /// reverse order, and the container's low byte kept in the answer.
    #[derive(Clone, Copy)]
    enum Fault {
        ReversedFlagOrder,
        KeepsContainerLowByte,
    }

    /// The query with one fault, over the same container.
    fn wrong_lift(fault: Fault, records: &Records) -> u32 {
        let mut observed = 0u8;
        if records.count > 1 {
            for index in 0..usize::from(records.count) {
                let flags = records.flags[index];
                match fault {
                    Fault::ReversedFlagOrder => {
                        if flags & 0x08 != 0 {
                            observed |= 0x08;
                        } else if flags & 0x02 != 0 {
                            observed |= 0x02;
                        }
                    }
                    Fault::KeepsContainerLowByte => {
                        if flags & 0x02 != 0 {
                            observed |= 0x02;
                        } else if flags & 0x08 != 0 {
                            observed |= 0x08;
                        }
                    }
                }
            }
        }
        match fault {
            Fault::ReversedFlagOrder => (records.container & 0xFFFF_FF00) | u32::from(observed == 0x0A),
            Fault::KeepsContainerLowByte => records.container | u32::from(observed == 0x0A),
        }
    }

    #[derive(Default)]
    struct Outcome {
        lift_mismatches: u32,
        wrong_mismatches: u32,
        first_mismatch: Option<String>,
    }

    fn run(name: &str, vtable_addr: u32, fault: Fault) -> Outcome {
        let mut rng = Rng(seed_of(name));
        let mut outcome = Outcome::default();
        for trial_index in 0..TRIALS {
            let count = rng.count();
            let mut flags = vec![0u8; usize::from(count)];
            for byte in &mut flags {
                *byte = rng.next() as u8;
            }
            let records = Records::new(count, flags);
            CONTAINER.store(records.container, Ordering::SeqCst);

            let mut frame = [0u8; SPAN];
            for (index, byte) in frame.iter_mut().enumerate() {
                *byte = (rng.next() as u8).wrapping_add(index as u8);
            }
            frame[0..4].copy_from_slice(&vtable_addr.to_le_bytes());

            let mut rewritten = Buffer([0u8; BUF]);
            rewritten.0[..SPAN].copy_from_slice(&frame);
            rewritten.0[SPAN..].fill(0x5A);
            let rw_answer = r::fn_00DB1430::rw_00DB1430(rewritten.0.as_mut_ptr());

            let mut lifted = UILayoutFrame::from_bytes(frame);
            let lift_answer = lifted.two_flags_set_among_records(&mut Host { records: &records });

            let frame_matches = rewritten.0[..SPAN] == lifted.bytes()[..];
            let tail_matches = rewritten.0[SPAN..] == [0x5A; BUF - SPAN];
            if rw_answer != lift_answer || !frame_matches || !tail_matches {
                outcome.lift_mismatches += 1;
                if outcome.first_mismatch.is_none() {
                    outcome.first_mismatch = Some(format!(
                        "trial {trial_index}: count {count}: answers {rw_answer:#010x} / {lift_answer:#010x}, frame equal {frame_matches}, tail equal {tail_matches}"
                    ));
                }
            }

            let wrong_answer = wrong_lift(fault, &records);
            if wrong_answer != rw_answer {
                outcome.wrong_mismatches += 1;
            }
        }
        outcome
    }

    #[test]
    fn two_flags_query_matches_its_rewrite_and_wrong_lifts_are_caught() {
        // A fake vtable whose record-query slot answers the container.
        let vtable: &'static mut [u32; VTABLE_WORDS] = Box::leak(Box::new([0u32; VTABLE_WORDS]));
        vtable[RECORD_QUERY_AT / 4] =
            (query_stub as extern "thiscall" fn(u32) -> u32) as usize as u32;
        let vtable_addr = vtable.as_ptr() as usize as u32;

        let outcome = run("two_flags_set_among_records", vtable_addr, Fault::ReversedFlagOrder);
        assert_eq!(
            outcome.lift_mismatches, 0,
            "two_flags_set_among_records: {}",
            outcome.first_mismatch.unwrap_or_default()
        );
        assert!(outcome.wrong_mismatches > 0, "the reversed-order lift matched on every trial");
        let kept = run("two_flags_set_among_records_low_byte", vtable_addr, Fault::KeepsContainerLowByte);
        assert!(kept.wrong_mismatches > 0, "the low-byte lift matched on every trial");
    }
}
