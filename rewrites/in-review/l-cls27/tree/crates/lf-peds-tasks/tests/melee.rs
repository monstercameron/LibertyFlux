//! Host tests of the lifted complex melee task: the edge cases a reader
//! of the code would ask about. These run on every host; the differential
//! proof against the verified rewrites is in `lf-melee-diff`.

use lf_core::boundary::Handle32;
use lf_peds_tasks::melee::{
    CtorArgs, DISABLE_BIT, FLAG_BASE, FOLLOW_UP_ALT, KIND_MELEE, MeleeHost, MeleeTask,
    SLOT_OFFSETS, StartArgs, SubTask,
};

/// A host that records the calls and answers from fixed values.
#[derive(Default)]
struct FakeHost {
    base_calls: u32,
    resets: Vec<u32>,
    set_targets: Vec<(u32, u32)>,
    stamps: (u32, u32),
    destroys: u32,
    manager_word: u32,
    manager_reads: u32,
    frees: Vec<u32>,
    handle_answer: u32,
    handles: Vec<u32>,
    mode_answer: u32,
    modes: u32,
    flag_answer: u32,
    flag_modes: Vec<u32>,
    starts: Vec<StartArgs>,
    timings: (u32, u32),
    setups: Vec<u32>,
    registers: Vec<(u32, u32, u32)>,
    cleanups: u32,
    query_answer: u32,
    queries: Vec<Option<Handle32<SubTask>>>,
    sink_answer: u32,
    sinks: Vec<(u32, u32, u32)>,
    /// Words the flag helper writes into the task's mode word, one per
    /// call, as a callee may.
    flag_writes: Vec<u32>,
    /// Bytes the cleanup writes into the flag byte, one per call.
    cleanup_writes: Vec<u8>,
}

impl FakeHost {
    fn flag_write(&mut self) -> Option<u32> {
        if self.flag_writes.is_empty() {
            None
        } else {
            Some(self.flag_writes.remove(0))
        }
    }

    fn cleanup_write(&mut self) -> Option<u8> {
        if self.cleanup_writes.is_empty() {
            None
        } else {
            Some(self.cleanup_writes.remove(0))
        }
    }
}

impl MeleeHost for FakeHost {
    fn construct_base(&mut self, _task: &mut MeleeTask) {
        self.base_calls += 1;
    }

    fn reset_slot(&mut self, task: &mut MeleeTask, offset: u32) {
        self.resets.push(offset);
        let index = SLOT_OFFSETS.iter().position(|o| *o == offset).unwrap();
        task.slots[index].0 = [index as u8; 12];
    }

    fn set_target_node(&mut self, _task: &mut MeleeTask, first: u32, second: u32) {
        self.set_targets.push((first, second));
    }

    fn stamp_words(&mut self) -> lf_peds_tasks::melee::StampWords {
        lf_peds_tasks::melee::StampWords {
            word_38: self.stamps.0,
            word_3c: self.stamps.1,
        }
    }

    fn destroy_task(&mut self, _task: &mut MeleeTask) {
        self.destroys += 1;
    }

    fn manager(&mut self) -> u32 {
        self.manager_reads += 1;
        self.manager_word
    }

    fn free_task(&mut self, manager: u32) {
        self.frees.push(manager);
    }

    fn task_handle(&mut self, manager: u32) -> u32 {
        self.handles.push(manager);
        self.handle_answer
    }

    fn mode_value(&mut self, _task: &mut MeleeTask) -> u32 {
        self.modes += 1;
        self.mode_answer
    }

    fn flag_value(&mut self, task: &mut MeleeTask, mode_byte: u32) -> u32 {
        self.flag_modes.push(mode_byte);
        if let Some(w) = self.flag_write() {
            task.mode_word = w;
        }
        self.flag_answer
    }

    fn start_melee(&mut self, args: &StartArgs) {
        self.starts.push(*args);
    }

    fn timing_words(&mut self) -> lf_peds_tasks::melee::InitWords {
        lf_peds_tasks::melee::InitWords {
            word_dc: self.timings.0,
            word_e0: self.timings.1,
        }
    }

    fn setup_target(&mut self, _task: &mut MeleeTask, arg: u32) {
        self.setups.push(arg);
    }

    fn register(&mut self, _task: &mut MeleeTask, arg: u32, kind: u32, flag: u32) {
        self.registers.push((arg, kind, flag));
    }

    fn release_refs(&mut self, task: &mut MeleeTask) {
        self.cleanups += 1;
        if let Some(b) = self.cleanup_write() {
            task.flags_f0 = b;
        }
    }

    fn subtask_query(&mut self, sub: Option<Handle32<SubTask>>) -> u32 {
        self.queries.push(sub);
        self.query_answer
    }

    fn follow_up_sink(
        &mut self,
        _task: &mut MeleeTask,
        arg: u32,
        kind: u32,
        zero: u32,
    ) -> u32 {
        self.sinks.push((arg, kind, zero));
        self.sink_answer
    }
}

fn args() -> CtorArgs {
    CtorArgs {
        initial_word: 0x1234_5678,
        flag_two: 1,
        flag_three: 0,
        flag_four: 1,
        flag_five: 0,
        target_first: 0xAAAA_BBBB,
        target_second: 0xCCCC_DDDD,
        sub_task: Handle32::new(0x1000),
        incoming_flags_high: 0x5A,
    }
}

#[test]
fn constructor_composes_the_flag_byte_from_low_bits() {
    let mut host = FakeHost::default();
    // flag_two -> bit 1, flag_four -> bit 0: 0x80 | 0x02 | 0x01.
    let task = MeleeTask::new(&mut host, &args());
    assert_eq!(task.flags_f0, FLAG_BASE | 0x03);
    assert_eq!(task.mode_word, 0x1234_5678);
    assert_eq!(task.flag_40, 1);
    // The high nibble of the incoming byte survives; the low nibble clears.
    assert_eq!(task.flags_f1, 0x50);
    // Everything else the constructor owns is zero.
    assert_eq!(task.word_20, 0);
    assert_eq!(task.word_34, 0);
    assert_eq!(task.word_44, 0);
    assert_eq!(task.word_48, 0);
    assert_eq!(task.half_4c, 0);
    assert_eq!(task.word_50, 0);
    assert_eq!(task.word_cc, 0);
    assert_eq!(task.word_d0, 0);
    assert_eq!(task.word_e8, 0);
    assert_eq!(task.word_ec, 0);
    assert_eq!(task.flag_e4, 0);
    assert_eq!(task.filler_41, 0);
    assert_eq!(task.filler_e5, 0);
}

#[test]
fn constructor_ignores_every_flag_bit_but_the_low_one() {
    let mut host = FakeHost::default();
    let mut a = args();
    // Every word has other bits set; only the low bits count.
    a.flag_two = 0xFFFF_FFFE;
    a.flag_three = 0x8000_0001;
    a.flag_four = 0x0000_0100;
    a.flag_five = 0x7FFF_FFFF;
    let task = MeleeTask::new(&mut host, &a);
    // flag_three -> bit 2, flag_five -> bit 3.
    assert_eq!(task.flags_f0, FLAG_BASE | 0x0C);
}

#[test]
fn constructor_calls_in_order_with_arguments_in_order() {
    let mut host = FakeHost::default();
    host.stamps = (0x1111_2222, 0x3333_4444);
    let task = MeleeTask::new(&mut host, &args());
    assert_eq!(host.base_calls, 1);
    assert_eq!(host.resets, SLOT_OFFSETS.to_vec());
    assert_eq!(host.set_targets, vec![(0xAAAA_BBBB, 0xCCCC_DDDD)]);
    assert_eq!(task.word_38, 0x1111_2222);
    assert_eq!(task.word_3c, 0x3333_4444);
    // The sub-task handle is carried in untouched.
    assert_eq!(task.sub_task, Handle32::new(0x1000));
    // Each slot holds what its reset wrote.
    for (i, slot) in task.slots.iter().enumerate() {
        assert_eq!(slot.0, [i as u8; 12]);
    }
}

#[test]
fn destroy_frees_only_on_the_low_bit() {
    let mut task = MeleeTask::default();
    // Words with the low bit clear: the destructor runs, nothing is freed,
    // and the manager word is never read.
    for flags in [0, 2, 4, 0x100, 0xFFFF_FFFE] {
        let mut host = FakeHost::default();
        host.manager_word = 0x7777;
        task.destroy(&mut host, flags);
        assert_eq!(host.destroys, 1);
        assert!(host.frees.is_empty());
        assert_eq!(host.manager_reads, 0);
    }
    // Words with the low bit set: freed through the manager word.
    for flags in [1, 3, 5, 0x101, 0xFFFF_FFFF] {
        let mut host = FakeHost::default();
        host.manager_word = 0x7777;
        task.destroy(&mut host, flags);
        assert_eq!(host.destroys, 1);
        assert_eq!(host.frees, vec![0x7777]);
        assert_eq!(host.manager_reads, 1);
    }
}

#[test]
fn start_ends_quietly_on_a_null_handle() {
    let mut task = MeleeTask::default();
    task.flags_f0 = 0xFF;
    let mut host = FakeHost::default();
    host.manager_word = 0x4242;
    host.handle_answer = 0;
    task.start(&mut host);
    assert_eq!(host.handles, vec![0x4242]);
    assert_eq!(host.modes, 0);
    assert!(host.flag_modes.is_empty());
    assert!(host.starts.is_empty());
}

#[test]
fn start_splits_flags_and_masks_the_mode_byte() {
    let mut task = MeleeTask::default();
    task.flags_f0 = 0b0000_1011;
    task.mode_word = 0xDEAD_BEEF;
    let mut host = FakeHost::default();
    host.handle_answer = 0x6000;
    host.mode_answer = 0x1234_56AB;
    host.flag_answer = 0x99;
    task.start(&mut host);
    assert_eq!(host.flag_modes, vec![0xAB]);
    assert_eq!(host.starts.len(), 1);
    let s = &host.starts[0];
    assert_eq!(s.handle, 0x6000);
    assert_eq!(s.mode_word, 0xDEAD_BEEF);
    // Bits arrive in the original's order: bit 1, bit 2, bit 0, bit 3.
    assert_eq!(s.flag_bit1, 1);
    assert_eq!(s.flag_bit2, 0);
    assert_eq!(s.flag_bit0, 1);
    assert_eq!(s.flag_bit3, 1);
    assert_eq!(s.computed, 0x99);
}

#[test]
fn start_reads_the_mode_word_after_the_flag_helper() {
    let mut task = MeleeTask::default();
    task.mode_word = 0x1111_1111;
    let mut host = FakeHost::default();
    host.handle_answer = 0x6000;
    // The flag helper rewrites the mode word, as a callee may.
    host.flag_writes = vec![0x2222_2222];
    task.start(&mut host);
    assert_eq!(host.starts.len(), 1);
    assert_eq!(host.starts[0].mode_word, 0x2222_2222);
}

#[test]
fn initialise_stores_words_sets_the_flag_and_registers() {
    let mut task = MeleeTask::default();
    let mut host = FakeHost::default();
    host.timings = (0xDC_DC_DCDC, 0xE0E0_E0E0);
    task.initialize(&mut host, 0x55);
    assert_eq!(task.word_dc, 0xDC_DC_DCDC);
    assert_eq!(task.word_e0, 0xE0E0_E0E0);
    assert_eq!(task.flag_e4, 1);
    assert_eq!(host.setups, vec![0x55]);
    assert_eq!(host.registers, vec![(0x55, KIND_MELEE, 1)]);
}

#[test]
fn follow_up_returns_zero_while_disabled() {
    let mut task = MeleeTask::default();
    task.flags_f0 = DISABLE_BIT;
    task.sub_task = Handle32::new(0x2000);
    let mut host = FakeHost::default();
    host.query_answer = KIND_MELEE;
    let answer = task.follow_up(&mut host, 0x77);
    assert_eq!(answer, 0);
    // The cleanup ran, but the sub-task was never asked and the sink never ran.
    assert_eq!(host.cleanups, 1);
    assert!(host.queries.is_empty());
    assert!(host.sinks.is_empty());
}

#[test]
fn follow_up_reads_the_flag_after_the_cleanup() {
    let mut task = MeleeTask::default();
    task.flags_f0 = 0;
    task.sub_task = Handle32::new(0x2000);
    let mut host = FakeHost::default();
    // The cleanup sets the disable bit, as a callee may.
    host.cleanup_writes = vec![DISABLE_BIT];
    host.query_answer = KIND_MELEE;
    let answer = task.follow_up(&mut host, 0x77);
    assert_eq!(answer, 0);
    assert!(host.queries.is_empty());
}

#[test]
fn follow_up_accepts_only_the_two_answers() {
    for (query, sinks) in [
        (KIND_MELEE, 1),
        (FOLLOW_UP_ALT, 1),
        (0, 0),
        (1, 0),
        (KIND_MELEE - 1, 0),
        (KIND_MELEE + 1, 0),
        (FOLLOW_UP_ALT - 1, 0),
        (FOLLOW_UP_ALT + 1, 0),
        (0xFFFF_FFFF, 0),
    ] {
        let mut task = MeleeTask::default();
        task.flags_f0 = 0;
        task.sub_task = Handle32::new(0x2000);
        let mut host = FakeHost::default();
        host.query_answer = query;
        host.sink_answer = 0x5EED;
        let answer = task.follow_up(&mut host, 0x77);
        assert_eq!(host.queries, vec![Handle32::new(0x2000)]);
        assert_eq!(host.sinks.len(), sinks, "query {query:#x}");
        if sinks == 1 {
            assert_eq!(host.sinks[0], (0x77, KIND_MELEE, 0));
            assert_eq!(answer, 0x5EED);
        } else {
            assert_eq!(answer, 0);
        }
    }
}

#[test]
fn registry_counts_five_proven_methods() {
    use lf_peds_tasks::melee::registry::{METHODS, State};
    assert_eq!(METHODS.len(), 17);
    let proven = METHODS.iter().filter(|m| m.state == State::Proven).count();
    assert_eq!(proven, 5);
    let unlifted = METHODS.iter().filter(|m| m.state == State::Unlifted).count();
    assert_eq!(unlifted, 1);
}
