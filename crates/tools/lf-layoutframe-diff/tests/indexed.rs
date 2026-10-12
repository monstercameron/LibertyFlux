//! Differential cases for the layout frame's indexed-list check.
//!
//! The lifted check and its verified rewrite run on the same generated
//! frames, registry answers and lists. The rewrite asks the frame, through its
//! vtable, for two queries; it looks the second answer up in the shared
//! registry through a callee; it reads the record's list and the list's
//! entries from memory; and it asks the shared resolver about each entry's
//! key through a second callee. The harness points the vtable slots and the
//! callees at recording stubs that answer from a script, and lays out the
//! record, list, entries and keys as memory the rewrite reads. The lifted
//! check makes the same calls through a recording host that gives the same
//! answers. Both sides must produce the same answer, the same calls in the
//! same order with the same arguments, and the same frame bytes. A
//! deliberately wrong lift of the check must differ.
//!
//! The rewrite's answer for a fallback is the frame's own list field, which
//! the harness maps from the buffer. The registry object is passed to the
//! registry callee as an address; the lifted host has no such address, so the
//! callee's log records the lookup key only.
//!
//! The callee table is a static the checker worker sets before a trial; the
//! harness sets it once, in the one test here.

// Writes the checker runtime's callee table static, which the checker worker sets before each trial in the real checker. Test-only.
#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::Mutex;

    use lf_checker_rt::CHECKER_CTABLE;
    use lf_input_frontend::layout_frame::{
        IndexedListHost, ListAnswer, SPAN, UILayoutFrame, field,
    };
    use lf_layoutframe_diff::rewrites as r;

    /// Bytes of the buffer the rewrite runs on: the frame's span and a tail
    /// that must come back unchanged.
    const BUF: usize = 0x100;

    /// Generated cases.
    const TRIALS: u32 = 2500;

    /// Largest entry count drawn. The counts 255, 256 and 257 are among them.
    const MAX_ENTRIES: usize = 300;

    /// Words in a fake vtable: enough for the highest query slot.
    const VTABLE_WORDS: usize = 0x100 / 4;

    /// Byte offsets of the frame's two queries.
    const FIRST_QUERY_AT: usize = 0xF0;
    const SECOND_QUERY_AT: usize = 0x4C;

    /// Words in the checker runtime's callee table.
    const CALLEE_TABLE_WORDS: usize = 256;

    /// Callee ids the rewrite calls: the registry lookup and the resolver.
    const CALLEE_REGISTRY: usize = 1;
    const CALLEE_RESOLVER: usize = 4;

    /// Offset of the fallback field from the frame's start.
    const FALLBACK_FIELD_AT: u32 = field::LIST_BASE as u32;

    /// One call, as the log records it.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Ev {
        FirstQuery,
        SecondQuery,
        Registry(u32),
        Resolve(u32),
    }

    /// The answers the stubs give, consumed in call order. The same script
    /// drives the lifted host.
    #[derive(Clone)]
    struct Script {
        first: u32,
        key: u32,
        record: u32,
        resolves: Vec<u32>,
        cursor: usize,
    }

    /// The log and script the rewrite's stubs use. Only the rewrite runs use it.
    static LOG: Mutex<Vec<Ev>> = Mutex::new(Vec::new());
    static SCRIPT: Mutex<Option<Script>> = Mutex::new(None);

    fn answer_resolve() -> u32 {
        let mut guard = SCRIPT.lock().unwrap();
        let script = guard.as_mut().expect("script set");
        let value = script.resolves[script.cursor % script.resolves.len()];
        script.cursor += 1;
        value
    }

    extern "thiscall" fn first_query_stub(_this: u32) -> u32 {
        LOG.lock().unwrap().push(Ev::FirstQuery);
        SCRIPT.lock().unwrap().as_ref().expect("script set").first
    }

    extern "thiscall" fn second_query_stub(_this: u32) -> u32 {
        LOG.lock().unwrap().push(Ev::SecondQuery);
        SCRIPT.lock().unwrap().as_ref().expect("script set").key
    }

    extern "thiscall" fn registry_stub(_registry: u32, key: u32) -> u32 {
        LOG.lock().unwrap().push(Ev::Registry(key));
        SCRIPT.lock().unwrap().as_ref().expect("script set").record
    }

    extern "cdecl" fn resolver_stub(key: u32) -> u32 {
        LOG.lock().unwrap().push(Ev::Resolve(key));
        answer_resolve()
    }

    /// The host the lifted check is given: the same script, and the list
    /// records and entries laid out for the rewrite.
    struct Host<'a> {
        script: Script,
        list: u32,
        count: u16,
        entries: &'a [(u32, u32)],
        log: Vec<Ev>,
    }

    impl IndexedListHost for Host<'_> {
        fn first_query(&mut self, _frame: &mut UILayoutFrame) -> u32 {
            self.log.push(Ev::FirstQuery);
            self.script.first
        }

        fn second_query(&mut self, _frame: &mut UILayoutFrame) -> u32 {
            self.log.push(Ev::SecondQuery);
            self.script.key
        }

        fn registry_record(&mut self, key: u32) -> u32 {
            self.log.push(Ev::Registry(key));
            self.script.record
        }

        fn registry_list(&mut self, record: u32) -> u32 {
            assert_eq!(record, self.script.record);
            self.list
        }

        fn list_count(&mut self, list: u32) -> u16 {
            assert_eq!(list, self.list);
            self.count
        }

        fn list_entry(&mut self, list: u32, index: usize) -> u32 {
            assert_eq!(list, self.list);
            self.entries[index].0
        }

        fn entry_key(&mut self, entry: u32) -> u32 {
            self.entries
                .iter()
                .find(|(address, _)| *address == entry)
                .map(|(_, key)| *key)
                .expect("entry is one of the list's")
        }

        fn key_resolves(&mut self, key: u32) -> u32 {
            self.log.push(Ev::Resolve(key));
            let value = self.script.resolves[self.script.cursor % self.script.resolves.len()];
            self.script.cursor += 1;
            value
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

        /// A word whose low byte is zero a quarter of the time.
        fn answer(&mut self) -> u32 {
            if self.next() % 4 == 0 {
                self.u32() & !0xFF
            } else {
                self.u32() | 1
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
        /// Goes on past a failed first query.
        IgnoresFirstQuery,
        /// Goes on past an entry the resolver refuses.
        IgnoresRefusal,
        /// Answers the fallback for an empty list.
        EmptyListFails,
    }

    /// The check with one fault, over the same host.
    fn wrong_check(fault: Fault, host: &mut Host<'_>, frame: &mut UILayoutFrame) -> ListAnswer {
        if host.first_query(frame) as u8 == 0 && !matches!(fault, Fault::IgnoresFirstQuery) {
            return ListAnswer::FieldFallback;
        }
        let key = host.second_query(frame);
        let record = host.registry_record(key);
        if record == 0 {
            return ListAnswer::FieldFallback;
        }
        let list = host.registry_list(record);
        if list == 0 {
            return ListAnswer::FieldFallback;
        }
        let count = host.list_count(list);
        if matches!(fault, Fault::EmptyListFails) && count == 0 {
            return ListAnswer::FieldFallback;
        }
        for index in 0..usize::from(count) {
            let entry = host.list_entry(list, index);
            let entry_key = host.entry_key(entry);
            if host.key_resolves(entry_key) as u8 == 0 && !matches!(fault, Fault::IgnoresRefusal) {
                return ListAnswer::FieldFallback;
            }
        }
        ListAnswer::List(list)
    }

    #[derive(Default)]
    struct Outcome {
        lift_mismatches: u32,
        wrong_mismatches: u32,
        trials_reaching_entries: u32,
        first_mismatch: Option<String>,
    }

    fn run(name: &str, frame_vtable: u32, fault: Fault) -> Outcome {
        let mut rng = Rng(seed_of(name));
        let mut outcome = Outcome::default();
        for trial_index in 0..TRIALS {
            let count = rng.count();
            // The entries: an item word and a key word per entry, the array of
            // item addresses, and the list record that names the array.
            let mut items: Vec<[u32; 2]> =
                (0..usize::from(count)).map(|_| [0, rng.u32()]).collect();
            let mut array: Vec<u32> = items
                .iter_mut()
                .map(|item| item.as_mut_ptr() as usize as u32)
                .collect();
            array.resize(MAX_ENTRIES.max(array.len()), 0);
            let mut list_block = vec![0u8; 8];
            list_block[0..4].copy_from_slice(&(array.as_ptr() as usize as u32).to_le_bytes());
            list_block[4..6].copy_from_slice(&count.to_le_bytes());
            let list = list_block.as_ptr() as usize as u32;
            let mut record_block = vec![0u32; 4];
            record_block[1] = list;
            let record = record_block.as_ptr() as usize as u32;

            let entries: Vec<(u32, u32)> = items
                .iter()
                .map(|item| (item.as_ptr() as usize as u32, item[1]))
                .collect();

            let resolves: Vec<u32> = (0..MAX_ENTRIES + 1).map(|_| rng.answer()).collect();
            let first = rng.answer();
            let key = rng.u32();
            let record_answer = if rng.next() % 4 == 0 { 0 } else { record };
            let script = Script {
                first,
                key,
                record: record_answer,
                resolves,
                cursor: 0,
            };

            let mut frame = [0u8; SPAN];
            for byte in &mut frame {
                *byte = rng.next() as u8;
            }
            frame[0..4].copy_from_slice(&frame_vtable.to_le_bytes());

            let mut rewritten = Buffer([0u8; BUF]);
            rewritten.0[..SPAN].copy_from_slice(&frame);
            rewritten.0[SPAN..].fill(0x5A);
            let buffer_base = rewritten.0.as_ptr() as usize as u32;
            LOG.lock().unwrap().clear();
            *SCRIPT.lock().unwrap() = Some(script.clone());
            let rw_raw = r::fn_00DB1490::rw_00DB1490(rewritten.0.as_mut_ptr());
            let rw_log = std::mem::take(&mut *LOG.lock().unwrap());
            let rw_answer = if rw_raw == buffer_base + FALLBACK_FIELD_AT {
                u32::MAX
            } else {
                rw_raw
            };
            if rw_log.iter().any(|ev| matches!(ev, Ev::Resolve(_))) {
                outcome.trials_reaching_entries += 1;
            }

            let mut lifted = UILayoutFrame::from_bytes(frame);
            let mut host = Host {
                script: script.clone(),
                list,
                count,
                entries: &entries,
                log: Vec::new(),
            };
            let lift_raw = lifted.every_list_entry_resolves(&mut host);
            let lift_answer = match lift_raw {
                ListAnswer::FieldFallback => u32::MAX,
                ListAnswer::List(handle) => handle,
            };

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
                script,
                list,
                count,
                entries: &entries,
                log: Vec::new(),
            };
            let mut wrong_frame = UILayoutFrame::from_bytes(frame);
            let wrong_raw = wrong_check(fault, &mut wrong_host, &mut wrong_frame);
            let wrong_answer = match wrong_raw {
                ListAnswer::FieldFallback => u32::MAX,
                ListAnswer::List(handle) => handle,
            };
            if wrong_answer != rw_answer || wrong_host.log != rw_log {
                outcome.wrong_mismatches += 1;
            }
            drop(items);
        }
        outcome
    }

    #[test]
    fn indexed_check_matches_its_rewrite_and_wrong_lifts_are_caught() {
        // The frame's vtable: the two queries, as stubs.
        let frame_vtable: &'static mut [u32; VTABLE_WORDS] =
            Box::leak(Box::new([0u32; VTABLE_WORDS]));
        frame_vtable[FIRST_QUERY_AT / 4] =
            (first_query_stub as extern "thiscall" fn(u32) -> u32) as usize as u32;
        frame_vtable[SECOND_QUERY_AT / 4] =
            (second_query_stub as extern "thiscall" fn(u32) -> u32) as usize as u32;
        let frame_vtable_addr = frame_vtable.as_ptr() as usize as u32;

        // The callee table: the registry lookup and the resolver.
        let callee_table: *mut u32 = Box::leak(Box::new([0u32; CALLEE_TABLE_WORDS])).as_mut_ptr();
        // SAFETY: the table holds CALLEE_TABLE_WORDS words and both ids are below that.
        unsafe {
            callee_table
                .add(CALLEE_REGISTRY)
                .write((registry_stub as extern "thiscall" fn(u32, u32) -> u32) as usize as u32);
            callee_table
                .add(CALLEE_RESOLVER)
                .write((resolver_stub as extern "cdecl" fn(u32) -> u32) as usize as u32);
            core::ptr::addr_of_mut!(CHECKER_CTABLE).write(callee_table);
        }

        let outcome = run(
            "every_list_entry_resolves",
            frame_vtable_addr,
            Fault::EmptyListFails,
        );
        assert_eq!(
            outcome.lift_mismatches,
            0,
            "every_list_entry_resolves: {}",
            outcome.first_mismatch.unwrap_or_default()
        );
        assert!(
            outcome.trials_reaching_entries > TRIALS / 4,
            "only {} of {} trials reached the entries",
            outcome.trials_reaching_entries,
            TRIALS
        );
        for (label, fault) in [
            ("ignores a failed first query", Fault::IgnoresFirstQuery),
            ("ignores a refused entry", Fault::IgnoresRefusal),
            ("fails an empty list", Fault::EmptyListFails),
        ] {
            let wrong = run(label, frame_vtable_addr, fault);
            assert!(
                wrong.wrong_mismatches > 0,
                "{label}: the deliberately wrong lift matched the rewrite on every trial"
            );
        }
    }
}
