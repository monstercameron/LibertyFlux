//! Differential cases for the layout frame's parameter-block refresh.
//!
//! The lifted refresh and its verified rewrite run on the same generated
//! frames, blocks and stamps. The rewrite asks the frame, through its vtable,
//! for a stamp; calls the allocator through the checker runtime's callee table
//! when the frame has no block; and reads and writes the block's words in
//! memory. The harness points the vtable slot and the allocator at recording
//! stubs, and gives the block as memory the harness owns. The lifted refresh
//! makes the same stamp and allocator calls through a recording host that
//! keeps a copy of the block's words. Both sides must produce the same answer,
//! the same calls in the same order, the same final block words and the same
//! frame bytes. A deliberately wrong lift of the refresh must differ.
//!
//! Not run: the allocator answering zero. The original's allocation-failure
//! path writes through the null block it was given, and so does the rewrite
//! (its comment says which store faults first); a trial there proves nothing,
//! so the harness never has the allocator answer zero.
//!
//! The callee table is a static the checker worker sets before a trial; the
//! harness sets it once, in the one test here.

// Writes the checker runtime's callee table static, which the checker worker sets before each trial in the real checker. Test-only.
#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicU32, Ordering};

    use lf_checker_rt::CHECKER_CTABLE;
    use lf_input_frontend::layout_frame::parameter_block::{BLOCK_BYTES, ParameterBlockHost};
    use lf_input_frontend::layout_frame::{SPAN, UILayoutFrame, field};
    use lf_layoutframe_diff::rewrites as r;

    /// Bytes of the buffer the rewrite runs on: the frame's span and a tail
    /// that must come back unchanged.
    const BUF: usize = 0x100;

    /// Generated cases.
    const TRIALS: u32 = 3000;

    /// Words in a fake vtable: enough for the stamp slot.
    const VTABLE_WORDS: usize = 0x100 / 4;

    /// Byte offset of the frame's stamp query in its vtable.
    const STAMP_QUERY_AT: usize = 0x4C;

    /// Words in the checker runtime's callee table.
    const CALLEE_TABLE_WORDS: usize = 256;

    /// Callee id the rewrite calls for the allocator.
    const CALLEE_ALLOCATE: usize = 1;

    /// Words in the block the refresh reads and writes.
    const BLOCK_WORDS: usize = (BLOCK_BYTES / 4) as usize;

    /// The block the allocator answers with, for the trial.
    static BLOCK: AtomicU32 = AtomicU32::new(0);

    /// The stamps the frame's stamp query answers, in call order.
    static STAMPS: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static STAMP_CURSOR: AtomicU32 = AtomicU32::new(0);

    /// One call, as the log records it. Block words are not logged: the final
    /// words are compared instead, since the rewrite writes them in memory.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Ev {
        Allocate(u32),
        Stamp,
    }

    /// The log the rewrite's stubs write to. Only the rewrite runs use it.
    static LOG: Mutex<Vec<Ev>> = Mutex::new(Vec::new());

    fn next_stamp() -> u32 {
        let cursor = STAMP_CURSOR.fetch_add(1, Ordering::SeqCst) as usize;
        let stamps = STAMPS.lock().unwrap();
        stamps[cursor % stamps.len()]
    }

    extern "thiscall" fn stamp_stub(_this: u32) -> u32 {
        LOG.lock().unwrap().push(Ev::Stamp);
        next_stamp()
    }

    extern "cdecl" fn allocate_stub(size: u32) -> u32 {
        LOG.lock().unwrap().push(Ev::Allocate(size));
        BLOCK.load(Ordering::SeqCst)
    }

    /// The host the lifted refresh is given: the stamps, the block's words as
    /// a copy, and the log of calls.
    struct Host {
        stamps: Vec<u32>,
        cursor: usize,
        block: u32,
        words: [u32; BLOCK_WORDS],
        log: Vec<Ev>,
    }

    impl ParameterBlockHost for Host {
        fn frame_stamp(&mut self, _frame: &mut UILayoutFrame) -> u32 {
            self.log.push(Ev::Stamp);
            let value = self.stamps[self.cursor % self.stamps.len()];
            self.cursor += 1;
            value
        }

        fn allocate_block(&mut self, size: u32) -> u32 {
            self.log.push(Ev::Allocate(size));
            self.block
        }

        fn write_block_word(&mut self, block: u32, index: usize, value: u32) {
            assert_eq!(block, self.block, "the refresh wrote another block");
            self.words[index] = value;
        }

        fn read_block_word(&mut self, block: u32, index: usize) -> u32 {
            assert_eq!(block, self.block, "the refresh read another block");
            self.words[index]
        }
    }

    /// Points the callee table at the allocator stub, once, for this binary.
    fn install() -> u32 {
        let vtable: &'static mut [u32; VTABLE_WORDS] = Box::leak(Box::new([0u32; VTABLE_WORDS]));
        vtable[STAMP_QUERY_AT / 4] =
            (stamp_stub as extern "thiscall" fn(u32) -> u32) as usize as u32;
        let callee_table: *mut u32 = Box::leak(Box::new([0u32; CALLEE_TABLE_WORDS])).as_mut_ptr();
        // SAFETY: the table holds CALLEE_TABLE_WORDS words and the id is below that.
        unsafe {
            callee_table
                .add(CALLEE_ALLOCATE)
                .write((allocate_stub as extern "cdecl" fn(u32) -> u32) as usize as u32);
            core::ptr::addr_of_mut!(CHECKER_CTABLE).write(callee_table);
        }
        vtable.as_ptr() as usize as u32
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

        fn u32(&mut self) -> u32 {
            (self.next() >> 32) as u32
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

    /// The deliberate faults of the wrong lift.
    #[derive(Clone, Copy)]
    enum Fault {
        /// Writes the count and fraction in the wrong order.
        SwapsCountAndFraction,
        /// Compares the block's stamp word with the fresh stamp, not the current one.
        ComparesFreshStamp,
        /// Skips the stamp query on the fresh-block path.
        SkipsFreshStamp,
    }

    /// The refresh with one fault, over the same host.
    fn wrong_refresh(
        fault: Fault,
        host: &mut Host,
        frame: &mut UILayoutFrame,
        fraction: u32,
        count: u32,
    ) -> u32 {
        let block_at_slot = frame.bytes()[field::CACHE_EC..field::CACHE_EC + 4]
            .iter()
            .any(|b| *b != 0);
        let write_all = |host: &mut Host, stamp: u32| {
            host.write_block_word(host.block, 4, 0);
            host.write_block_word(host.block, 3, 0);
            host.write_block_word(host.block, 2, stamp);
            match fault {
                Fault::SwapsCountAndFraction => {
                    host.write_block_word(host.block, 1, fraction);
                    host.write_block_word(host.block, 0, count);
                }
                _ => {
                    host.write_block_word(host.block, 1, count);
                    host.write_block_word(host.block, 0, fraction);
                }
            }
        };
        if !block_at_slot {
            let block = host.allocate_block(BLOCK_BYTES);
            for (index, value) in [0u32, 1, 0, 0, 0].into_iter().enumerate() {
                host.write_block_word(block, index, value);
            }
            let fresh = host.frame_stamp(frame);
            write_all(host, fresh);
            fresh
        } else {
            let current = host.frame_stamp(frame);
            let stored = host.read_block_word(host.block, 2);
            let compared = match fault {
                Fault::ComparesFreshStamp => host.frame_stamp(frame),
                _ => current,
            };
            if stored == compared {
                let fresh = match fault {
                    Fault::SkipsFreshStamp => current,
                    _ => host.frame_stamp(frame),
                };
                write_all(host, fresh);
                fresh
            } else {
                current
            }
        }
    }

    #[derive(Default)]
    struct Outcome {
        lift_mismatches: u32,
        wrong_mismatches: u32,
        refreshed_trials: u32,
        first_mismatch: Option<String>,
    }

    fn run(name: &str, frame_vtable: u32, fault: Fault) -> Outcome {
        let mut rng = Rng(seed_of(name));
        let mut outcome = Outcome::default();
        for trial_index in 0..TRIALS {
            let fraction = rng.u32();
            let count = rng.u32();
            let stamps = vec![rng.u32(), rng.u32(), rng.u32(), rng.u32()];
            let mut block_words: Vec<u32> = (0..BLOCK_WORDS).map(|_| rng.u32()).collect();
            // Half the trials store the first stamp in the block, so the
            // refresh's equal-stamp branch runs.
            if rng.u32() % 2 == 0 {
                block_words[2] = stamps[0];
            }
            let block_address = block_words.as_mut_ptr() as usize as u32;
            BLOCK.store(block_address, Ordering::SeqCst);

            let mut frame = [0u8; SPAN];
            for byte in &mut frame {
                *byte = rng.next() as u8;
            }
            frame[0..4].copy_from_slice(&frame_vtable.to_le_bytes());
            // The block pointer lives in the cache word: absent on half the trials.
            let cache = if rng.u32() % 2 == 0 { 0 } else { block_address };
            frame[field::CACHE_EC..field::CACHE_EC + 4].copy_from_slice(&cache.to_le_bytes());

            // The block's words before either side runs. The rewrite writes into
            // the block's memory, so the host and the wrong lift start from this copy.
            let mut initial_words = [0u32; BLOCK_WORDS];
            initial_words.copy_from_slice(&block_words);

            let mut rewritten = Buffer([0u8; BUF]);
            rewritten.0[..SPAN].copy_from_slice(&frame);
            rewritten.0[SPAN..].fill(0x5A);
            *STAMPS.lock().unwrap() = stamps.clone();
            STAMP_CURSOR.store(0, Ordering::SeqCst);
            LOG.lock().unwrap().clear();
            let rw_answer = r::fn_00DB1A00::rw_00db1a00(
                rewritten.0.as_mut_ptr() as usize as u32,
                fraction,
                count,
            );
            let rw_log = std::mem::take(&mut *LOG.lock().unwrap());
            // The rewrite's final block words are the words it left in the block's memory.
            let rw_words: Vec<u32> = (0..BLOCK_WORDS)
                .map(|index| {
                    // SAFETY: the block is the live Vec the trial owns; index is below BLOCK_WORDS.
                    unsafe { ((block_address as usize + index * 4) as *const u32).read() }
                })
                .collect();
            if rw_words != initial_words.to_vec() {
                outcome.refreshed_trials += 1;
            }

            let mut lifted = UILayoutFrame::from_bytes(frame);
            let mut host = Host {
                stamps: stamps.clone(),
                cursor: 0,
                block: block_address,
                words: initial_words,
                log: Vec::new(),
            };
            let lift_answer = lifted.refresh_parameter_block(&mut host, fraction, count);
            let lift_words = host.words.to_vec();

            let frame_matches = rewritten.0[..SPAN] == lifted.bytes()[..];
            let tail_matches = rewritten.0[SPAN..] == [0x5A; BUF - SPAN];
            let words_match = rw_words == lift_words;
            if rw_answer != lift_answer
                || rw_log != host.log
                || !frame_matches
                || !tail_matches
                || !words_match
            {
                outcome.lift_mismatches += 1;
                if outcome.first_mismatch.is_none() {
                    outcome.first_mismatch = Some(format!(
                        "trial {trial_index}: answers {rw_answer:#x} / {lift_answer:#x}, calls {rw_log:?} / {:?}, frame equal {frame_matches}, tail equal {tail_matches}, block equal {words_match}",
                        host.log
                    ));
                }
            }

            let mut wrong_frame = UILayoutFrame::from_bytes(frame);
            let mut wrong_host = Host {
                stamps: stamps.clone(),
                cursor: 0,
                block: block_address,
                words: initial_words,
                log: Vec::new(),
            };
            let wrong_answer =
                wrong_refresh(fault, &mut wrong_host, &mut wrong_frame, fraction, count);
            if wrong_answer != rw_answer
                || wrong_host.log != rw_log
                || wrong_host.words.to_vec() != rw_words
            {
                outcome.wrong_mismatches += 1;
            }
        }
        outcome
    }

    #[test]
    fn parameter_block_refresh_matches_its_rewrite_and_wrong_lifts_are_caught() {
        let frame_vtable = install();
        let outcome = run(
            "refresh_parameter_block",
            frame_vtable,
            Fault::SwapsCountAndFraction,
        );
        assert_eq!(
            outcome.lift_mismatches,
            0,
            "refresh_parameter_block: {}",
            outcome.first_mismatch.unwrap_or_default()
        );
        assert!(
            outcome.refreshed_trials > TRIALS / 2,
            "only {} of {} trials wrote the block",
            outcome.refreshed_trials,
            TRIALS
        );
        for (label, fault) in [
            ("swaps count and fraction", Fault::SwapsCountAndFraction),
            ("compares the fresh stamp", Fault::ComparesFreshStamp),
            ("skips the fresh stamp", Fault::SkipsFreshStamp),
        ] {
            let wrong = run(label, frame_vtable, fault);
            assert!(
                wrong.wrong_mismatches > 0,
                "{label}: the deliberately wrong lift matched the rewrite on every trial"
            );
        }
    }
}
