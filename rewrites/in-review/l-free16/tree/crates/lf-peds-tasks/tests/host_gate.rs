//! Host tests of the task request gate: the edge cases a reader would ask
//! about. The differential proof against the verified 32-bit rewrite lives
//! in the `lf-pedmoverdiff` test crate.

use lf_peds_tasks::ped_task::{GateCandidate, GateStatus, GateWords, admits};

/// A status probe that records its calls and answers with a fixed value.
struct Probe {
    answer: u8,
    asked: u32,
}

impl GateStatus for Probe {
    type Handle = u32;

    fn status(&mut self, _task: u32) -> u8 {
        self.asked += 1;
        self.answer
    }
}

fn open_words() -> GateWords {
    // The vtable flag bit clear, no presence, no busy byte: the probe is
    // not checked unless the mode is nonzero.
    GateWords {
        vtable_flag: 0,
        presence: 0,
        busy: 0,
    }
}

fn idle() -> GateCandidate {
    GateCandidate {
        state: 0,
        sub_flags: 0,
    }
}

#[test]
fn the_reject_bit_closes_the_gate_even_for_an_idle_candidate() {
    let mut probe = Probe { answer: 1, asked: 0 };
    assert!(!admits(&mut probe, 0, &open_words(), &idle(), 0x1000, 0));
}

#[test]
fn an_unchecked_request_with_no_mode_and_no_presence_skips_the_probe() {
    let mut probe = Probe { answer: 0, asked: 0 };
    assert!(admits(&mut probe, 0, &open_words(), &idle(), 0, 0));
    assert_eq!(probe.asked, 0);
}

#[test]
fn a_nonzero_mode_asks_the_probe_and_a_zero_answer_closes_the_gate() {
    let mut probe = Probe { answer: 0, asked: 0 };
    assert!(!admits(&mut probe, 0, &open_words(), &idle(), 0, 1));
    assert_eq!(probe.asked, 1);
}

#[test]
fn the_vtable_flag_bit_skips_the_probe_and_the_byte_check() {
    let words = GateWords {
        vtable_flag: 0x0080_0000,
        presence: 1,
        busy: 0x00ff_0000,
    };
    let mut probe = Probe { answer: 0, asked: 0 };
    assert!(admits(&mut probe, 0, &words, &idle(), 0, 1));
    assert_eq!(probe.asked, 0);
}

#[test]
fn a_busy_byte_closes_a_checked_gate_after_a_ready_probe() {
    let words = GateWords {
        vtable_flag: 0,
        presence: 1,
        busy: 0x0001_0000,
    };
    let mut probe = Probe { answer: 1, asked: 0 };
    assert!(!admits(&mut probe, 0, &words, &idle(), 0, 0));
    assert_eq!(probe.asked, 1);
}

#[test]
fn a_low_byte_of_the_busy_word_does_not_close_the_gate() {
    // Only the byte at bit sixteen is read; the low bytes are ignored.
    let words = GateWords {
        vtable_flag: 0,
        presence: 1,
        busy: 0x0000_ffff,
    };
    let mut probe = Probe { answer: 1, asked: 0 };
    assert!(admits(&mut probe, 0, &words, &idle(), 0, 0));
}

#[test]
fn a_busy_candidate_passes_only_with_the_sub_bit_clear() {
    let busy = |sub_flags| GateCandidate {
        state: 0xc0,
        sub_flags,
    };
    let mut probe = Probe { answer: 0, asked: 0 };
    assert!(!admits(&mut probe, 0, &open_words(), &busy(0x10), 0, 0));
    assert!(admits(&mut probe, 0, &open_words(), &busy(0xef), 0, 0));
}

#[test]
fn only_the_masked_state_decides_busy() {
    // Bits outside the mask do not make a candidate busy.
    let candidate = GateCandidate {
        state: 0xc0 | 0x400,
        sub_flags: 0,
    };
    let mut probe = Probe { answer: 0, asked: 0 };
    assert!(admits(&mut probe, 0, &open_words(), &candidate, 0, 0));
}
