//! Differential cases for the layout frame's pointer-list release.
//!
//! The lifted release and its verified rewrite run on the same generated
//! frames and entry lists. The list is memory the harness owns: an array of
//! entry words the frame's base word addresses, with the count in the frame.
//! The rewrite calls each entry's teardown, its free routine, and the owner's
//! report through the checker runtime's callee table; the harness points those
//! at recording stubs. The lifted release makes the same calls through a
//! recording host that holds the same entries. Both sides must produce the
//! same answer, the same calls in the same order with the same arguments, and
//! the same frame bytes. A deliberately wrong lift of the release must differ.
//!
//! The owner's report receives the list field's address. The rewrite's stub
//! reports it as an offset from the frame's buffer, and the lifted host
//! reports the field's offset, so the two logs compare the same thing.
//!
//! The checker runtime's callee table is a static the checker worker sets
//! before a trial; the harness sets it once, in the one test here.

// Writes the checker runtime's callee table static, which the checker worker sets before each trial in the real checker. Test-only.
#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicU32, Ordering};

    use lf_checker_rt::CHECKER_CTABLE;
    use lf_input_frontend::layout_frame::list::ListHost;
    use lf_input_frontend::layout_frame::{SPAN, UILayoutFrame, field};
    use lf_layoutframe_diff::rewrites as r;

    /// Bytes of the buffer the rewrite runs on: the frame's span and a tail
    /// that must come back unchanged.
    const BUF: usize = 0x100;

    /// Generated cases.
    const TRIALS: u32 = 3000;

    /// Largest entry count drawn. The counts 255, 256 and 257 are among them.
    const MAX_ENTRIES: usize = 300;

    /// Words in the checker runtime's callee table.
    const CALLEE_TABLE_WORDS: usize = 256;

    /// Callee ids the rewrite calls: the entry teardown, the free routine,
    /// and the owner's report.
    const CALLEE_RELEASE: usize = 1;
    const CALLEE_FREE: usize = 2;
    const CALLEE_OWNER: usize = 3;

    /// The frame base the rewrite's stubs measure the list field from.
    static FRAME_BASE: AtomicU32 = AtomicU32::new(0);

    /// One call, as the log records it.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Ev {
        Release(u32),
        Free(u32),
        Owner { field_offset: u32, base: u32, end: u32 },
    }

    /// The log the rewrite's stubs write to. Only the rewrite runs use it.
    static LOG: Mutex<Vec<Ev>> = Mutex::new(Vec::new());

    extern "thiscall" fn release_stub(entry: u32) -> u32 {
        LOG.lock().unwrap().push(Ev::Release(entry));
        0
    }

    extern "cdecl" fn free_stub(entry: u32) -> u32 {
        LOG.lock().unwrap().push(Ev::Free(entry));
        0
    }

    extern "thiscall" fn owner_stub(field: u32, base: u32, end: u32) -> u32 {
        let field_offset = field.wrapping_sub(FRAME_BASE.load(Ordering::SeqCst));
        LOG.lock().unwrap().push(Ev::Owner { field_offset, base, end });
        0
    }

    /// The host the lifted release is given: the entries the harness laid out
    /// and the log of calls.
    struct Host {
        base: u32,
        entries: Vec<u32>,
        log: Vec<Ev>,
    }

    impl ListHost for Host {
        fn list_entry(&mut self, list: u32, index: usize) -> u32 {
            assert_eq!(list, self.base, "the release walked another list");
            self.entries[index]
        }

        fn release_entry(&mut self, entry: u32) {
            self.log.push(Ev::Release(entry));
        }

        fn free_entry(&mut self, entry: u32) {
            self.log.push(Ev::Free(entry));
        }

        fn list_owner_released(&mut self, _frame: &mut UILayoutFrame, base: u32, end: u32) {
            self.log.push(Ev::Owner { field_offset: field::LIST_BASE as u32, base, end });
        }
    }

    /// Points the callee table at the three stubs, once, for this binary.
    fn install() {
        let callee_table: *mut u32 = Box::leak(Box::new([0u32; CALLEE_TABLE_WORDS])).as_mut_ptr();
        // SAFETY: the table holds CALLEE_TABLE_WORDS words and the three ids are below that.
        unsafe {
            callee_table
                .add(CALLEE_RELEASE)
                .write((release_stub as extern "thiscall" fn(u32) -> u32) as usize as u32);
            callee_table
                .add(CALLEE_FREE)
                .write((free_stub as extern "cdecl" fn(u32) -> u32) as usize as u32);
            callee_table
                .add(CALLEE_OWNER)
                .write((owner_stub as extern "thiscall" fn(u32, u32, u32) -> u32) as usize as u32);
            core::ptr::addr_of_mut!(CHECKER_CTABLE).write(callee_table);
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

        fn u32(&mut self) -> u32 {
            (self.next() >> 32) as u32
        }

        /// A count: the edges 0, 1, 2, 255, 256, 257 and 300 about one time in
        /// two, otherwise any count up to the largest drawn.
        fn count(&mut self) -> u16 {
            const EDGE: [u16; 7] = [0, 1, 2, 255, 256, 257, 300];
            if self.next() % 2 == 0 {
                EDGE[(self.next() % EDGE.len() as u64) as usize]
            } else {
                (self.next() % (MAX_ENTRIES as u64 + 1)) as u16
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

    /// The deliberate faults of the wrong lift.
    #[derive(Clone, Copy)]
    enum Fault {
        /// Frees a null entry too.
        FreesNullEntries,
        /// Frees each entry before running its teardown.
        FreesBeforeRelease,
        /// Reports one word past the end of the walked range.
        OwnerEndPastRange,
    }

    /// The release with one fault, over the same host.
    fn wrong_release(fault: Fault, host: &mut Host, base: u32, count: u16) -> u32 {
        let end = base.wrapping_add(u32::from(count) * 4);
        if base != end {
            for index in 0..usize::from(count) {
                let entry = host.entries[index];
                match fault {
                    Fault::FreesNullEntries => {
                        host.log.push(Ev::Release(entry));
                        host.log.push(Ev::Free(entry));
                    }
                    Fault::FreesBeforeRelease => {
                        if entry != 0 {
                            host.log.push(Ev::Free(entry));
                            host.log.push(Ev::Release(entry));
                        }
                    }
                    Fault::OwnerEndPastRange => {
                        if entry != 0 {
                            host.log.push(Ev::Release(entry));
                            host.log.push(Ev::Free(entry));
                        }
                    }
                }
            }
        }
        let owner_end = match fault {
            Fault::OwnerEndPastRange => end.wrapping_add(4),
            _ => end,
        };
        host.log.push(Ev::Owner { field_offset: field::LIST_BASE as u32, base, end: owner_end });
        0
    }

    #[derive(Default)]
    struct Outcome {
        lift_mismatches: u32,
        wrong_mismatches: u32,
        trials_with_calls: u32,
        first_mismatch: Option<String>,
    }

    fn run(name: &str, fault: Fault) -> Outcome {
        let mut rng = Rng(seed_of(name));
        let mut outcome = Outcome::default();
        for trial_index in 0..TRIALS {
            let count = rng.count();
            // The entry words the frame's base addresses. A third are null.
            let mut entries = vec![0u32; MAX_ENTRIES.max(usize::from(count))];
            for slot in entries.iter_mut().take(usize::from(count)) {
                *slot = if rng.next() % 3 == 0 { 0 } else { rng.u32() | 1 };
            }
            let base = entries.as_ptr() as usize as u32;

            let mut frame = [0u8; SPAN];
            for byte in &mut frame {
                *byte = rng.next() as u8;
            }
            frame[field::LIST_BASE..field::LIST_BASE + 4].copy_from_slice(&base.to_le_bytes());
            frame[field::LIST_COUNT..field::LIST_COUNT + 2].copy_from_slice(&count.to_le_bytes());

            let mut rewritten = Buffer([0u8; BUF]);
            rewritten.0[..SPAN].copy_from_slice(&frame);
            rewritten.0[SPAN..].fill(0x5A);
            FRAME_BASE.store(rewritten.0.as_ptr() as usize as u32, Ordering::SeqCst);
            LOG.lock().unwrap().clear();
            let rw_answer = r::fn_00DB18C0::rw_00db18c0(rewritten.0.as_mut_ptr() as usize as u32);
            let rw_log = std::mem::take(&mut *LOG.lock().unwrap());
            if rw_log.len() > 1 {
                outcome.trials_with_calls += 1;
            }

            let mut lifted = UILayoutFrame::from_bytes(frame);
            let mut host = Host {
                base,
                entries: entries[..usize::from(count)].to_vec(),
                log: Vec::new(),
            };
            let lift_answer = lifted.release_pointer_list_entries(&mut host);

            let frame_matches = rewritten.0[..SPAN] == lifted.bytes()[..];
            let tail_matches = rewritten.0[SPAN..] == [0x5A; BUF - SPAN];
            if rw_answer != lift_answer || rw_log != host.log || !frame_matches || !tail_matches {
                outcome.lift_mismatches += 1;
                if outcome.first_mismatch.is_none() {
                    outcome.first_mismatch = Some(format!(
                        "trial {trial_index}: count {count}: answers {rw_answer:#x} / {lift_answer:#x}, calls {} / {}, frame equal {frame_matches}, tail equal {tail_matches}",
                        rw_log.len(),
                        host.log.len()
                    ));
                }
            }

            let mut wrong_host = Host {
                base,
                entries: entries[..usize::from(count)].to_vec(),
                log: Vec::new(),
            };
            let wrong_answer = wrong_release(fault, &mut wrong_host, base, count);
            if wrong_answer != rw_answer || wrong_host.log != rw_log {
                outcome.wrong_mismatches += 1;
            }
        }
        outcome
    }

    #[test]
    fn list_release_matches_its_rewrite_and_wrong_lifts_are_caught() {
        install();
        let outcome = run("release_pointer_list_entries", Fault::FreesNullEntries);
        assert_eq!(
            outcome.lift_mismatches, 0,
            "release_pointer_list_entries: {}",
            outcome.first_mismatch.unwrap_or_default()
        );
        assert!(
            outcome.trials_with_calls > TRIALS / 2,
            "only {} of {} trials made calls",
            outcome.trials_with_calls,
            TRIALS
        );
        for (label, fault) in [
            ("frees null entries", Fault::FreesNullEntries),
            ("frees before release", Fault::FreesBeforeRelease),
            ("owner end past range", Fault::OwnerEndPastRange),
        ] {
            let wrong = run(label, fault);
            assert!(
                wrong.wrong_mismatches > 0,
                "{label}: the deliberately wrong lift matched the rewrite on every trial"
            );
        }
    }
}
