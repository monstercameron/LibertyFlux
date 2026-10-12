//! Differential tests of the lifted combining filter against its verified
//! 32-bit rewrites: initialise (virtual slot 0), poll (slot 3) and fold
//! (slot 4).
//!
//! Each case builds real 32-bit objects, runs the rewrite and the lifted
//! method on the same inputs, and compares results, every call in order
//! with its arguments, and every word the rewrite writes. A deliberately
//! wrong copy of each method runs on the same inputs and must be caught.
//! Child counts are generated from zero to eight: the original's child list
//! is inline, from offset 12 to offset 44, so a count past eight would read
//! the count and mode words as children and is outside the proof.

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

use std::cell::RefCell;
use std::rc::Rc;

use lf_animation::composer::combining_filter::MODE_BITS;
use lf_animation::composer::{BaseFilterCallees, CombiningFilter, FilterOwner, PollChild};
use lf_composerdiff::harness::{self, Call, ChildObj, Rng, VtObj, addr_of};
use lf_composerdiff::rewrites::{fn_00878640, fn_00878670, fn_008786E0};
use lf_core::boundary::Handle32;

/// Trials per case: enough for every branch of each method several times.
const TRIALS: usize = 400;
/// File address of the combining filter's class vtable (the rewrite of
/// initialise writes its relocated address into the object).
const CLASS_VTABLE_VA: u32 = 0x00E8_6AFC;
/// Word index of the child count (offset 0x2C) and of the mode word (0x30).
const COUNT_WORD: usize = 0x2C / 4;
const MODE_WORD: usize = 0x30 / 4;
/// Children the original's inline list holds: offsets 0x0C to 0x2B.
const INLINE_CHILDREN: usize = 8;

type Log = Rc<RefCell<Vec<Call>>>;

/// A child whose poll the lifted code calls; the answer is fixed per child.
struct ScriptedChild {
    addr: u32,
    answer: u32,
    log: Log,
}

impl PollChild for ScriptedChild {
    fn poll(&mut self, a0: u32, a1: u32, a2: u32) -> u32 {
        self.log
            .borrow_mut()
            .push(("child.poll", vec![self.addr, a0, a1, a2]));
        self.answer
    }
}

/// The owner the lifted code registers with.
struct FakeOwner {
    addr: u32,
    log: Log,
}

impl FilterOwner for FakeOwner {
    fn register_filter(&mut self, filter: Option<Handle32<CombiningFilter>>) {
        self.log.borrow_mut().push((
            "owner.register",
            vec![self.addr, Handle32::raw_or_zero(filter)],
        ));
    }
}

/// The base class's answer, scripted per trial.
struct FakeBase {
    answer: u32,
    log: Log,
}

impl BaseFilterCallees for FakeBase {
    fn base_answer(&mut self, filter: Option<Handle32<CombiningFilter>>) -> u32 {
        self.log
            .borrow_mut()
            .push(("base.answer", vec![Handle32::raw_or_zero(filter)]));
        self.answer
    }
}

/// A new log for one lifted run.
fn new_log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

/// Three poll arguments that are pairwise different.
fn distinct_args(rng: &mut Rng) -> [u32; 3] {
    loop {
        let a = [rng.word(), rng.word(), rng.word()];
        if a[0] != a[1] && a[1] != a[2] && a[0] != a[2] {
            return a;
        }
    }
}

/// A child answer: zero, a nonzero low byte, or nonzero high bits with a
/// zero low byte (not a verdict for the rewrite, which reads the low byte).
fn child_answer(rng: &mut Rng) -> u32 {
    match rng.u32() % 3 {
        0 => 0,
        1 => rng.u32() | 0x01,
        _ => rng.answer(),
    }
}

/// One trial of initialise. `wrong` runs a deliberately wrong copy that
/// registers on the wrong flag bit.
fn initialise_trial(rng: &mut Rng, wrong: bool) -> Result<(), String> {
    let flags = rng.word();
    let class_vt: Box<[u32; 1]> = Box::new([0]);
    let xbase = harness::xbase_for(CLASS_VTABLE_VA, addr_of(&class_vt[0]));
    let owner_vt = harness::owner_vtable();
    let owner_obj = Box::new(VtObj {
        vtable: addr_of(&owner_vt[0]),
        spare: 0,
    });
    let owner_addr = addr_of(&*owner_obj);
    let thread = Box::new([0u32, 0, owner_addr, 0]);
    harness::install(addr_of(&thread[0]), xbase);

    let mut filter_buf: Box<[u32; 16]> = Box::new([0; 16]);
    for w in filter_buf.iter_mut() {
        *w = rng.word();
    }
    let before = *filter_buf;
    let block = addr_of(&filter_buf[0]);

    harness::begin(0, "unused");
    let ret = fn_00878640::rw_00878640(block, flags);
    let rewrite_log = harness::take_log();

    // The rewrite writes exactly the vtable word and the count word.
    if filter_buf[0] != addr_of(&class_vt[0]) {
        return Err(format!("vtable word {:#x} is not the class vtable", filter_buf[0]));
    }
    if filter_buf[COUNT_WORD] != 0 {
        return Err("count word not cleared".into());
    }
    for (i, (now, was)) in filter_buf.iter().zip(before.iter()).enumerate() {
        if i != 0 && i != COUNT_WORD && now != was {
            return Err(format!("word {i} changed"));
        }
    }

    let log = new_log();
    let (lift_ret, lift_log, lift_count, lift_mode) = if wrong {
        // The wrong copy registers when the second flag bit is set.
        let mut l = Vec::new();
        if flags & 2 != 0 {
            l.push(("owner.register", vec![owner_addr, block]));
        }
        (block, l, 0u32, before[MODE_WORD])
    } else {
        let mut filter = CombiningFilter::new(
            vec![Box::new(ScriptedChild {
                addr: 0,
                answer: 0,
                log: log.clone(),
            })],
            before[MODE_WORD],
        );
        let mut owner = FakeOwner {
            addr: owner_addr,
            log: log.clone(),
        };
        let owner_arg = if flags & 1 != 0 { Some(&mut owner) } else { None };
        let got = filter.initialise(Handle32::new(block), flags, owner_arg);
        let l = log.borrow().clone();
        (
            Handle32::raw_or_zero(got),
            l,
            filter.child_count() as u32,
            filter.mode(),
        )
    };

    let checked = if wrong {
        lift_ret == ret && lift_log == rewrite_log
    } else {
        lift_ret == ret
            && lift_log == rewrite_log
            && lift_count == filter_buf[COUNT_WORD]
            && lift_mode == before[MODE_WORD]
    };
    if checked {
        Ok(())
    } else {
        Err(format!(
            "initialise flags {flags:#x}: rewrite ret {ret:#x} log {rewrite_log:?}, lift ret {lift_ret:#x} log {lift_log:?}"
        ))
    }
}

#[test]
fn initialise_matches_rewrite() {
    let _guard = harness::lock();
    let mut rng = Rng(0x1A17_0001);
    for _ in 0..TRIALS {
        initialise_trial(&mut rng, false).unwrap();
    }
}

#[test]
fn initialise_wrong_flag_bit_is_caught() {
    let _guard = harness::lock();
    let mut rng = Rng(0x1A17_0001);
    assert!((0..TRIALS).any(|_| initialise_trial(&mut rng, true).is_err()));
}

/// The answer a lifted poll gives for children `kids` (address, answer),
/// logging each child's poll. `mutant` selects a wrong copy: 1 swaps the
/// OR and AND terms, 2 reads the mode bit one place along, 3 reads the
/// whole answer as the verdict.
fn poll_model(
    mutant: u8,
    kids: &[(u32, u32)],
    mode: u32,
    args: [u32; 3],
    log: &mut Vec<Call>,
) -> bool {
    for (index, &(addr, answer)) in kids.iter().enumerate() {
        log.push(("child.poll", vec![addr, args[0], args[1], args[2]]));
        let bit_index = if mutant == 2 { index + 1 } else { index };
        let mode_bit = index < MODE_BITS
            && mode.checked_shr(bit_index as u32).unwrap_or(0) & 1 != 0;
        let or_term = if mutant == 1 { !mode_bit } else { mode_bit };
        let verdict = if mutant == 3 { answer != 0 } else { (answer as u8) != 0 };
        if or_term {
            if verdict {
                return true;
            }
        } else if !verdict {
            return false;
        }
    }
    true
}

/// One trial of poll: random child count, mode, arguments and answers.
fn poll_trial(rng: &mut Rng, mutant: u8) -> Result<(), String> {
    let count: u32 = match rng.u32() % 12 {
        9 => 0xFFFF_FFFF,
        10 => 0x8000_0000,
        n => n % 9,
    };
    let mode = rng.word();
    let args = distinct_args(rng);

    let child_vt = harness::child_vtable();
    let child_vt_addr = addr_of(&child_vt[0]);
    let children: Vec<Box<ChildObj>> = (0..INLINE_CHILDREN)
        .map(|_| {
            Box::new(ChildObj {
                vtable: child_vt_addr,
                answer: child_answer(rng),
            })
        })
        .collect();

    let mut buf: Box<[u32; 16]> = Box::new([0; 16]);
    for w in buf.iter_mut() {
        *w = rng.word();
    }
    for (i, child) in children.iter().enumerate() {
        buf[3 + i] = addr_of(&**child);
    }
    buf[COUNT_WORD] = count;
    buf[MODE_WORD] = mode;

    let live = if (count as i32) > 0 {
        (count as usize).min(INLINE_CHILDREN)
    } else {
        0
    };

    harness::install(0, 0);
    harness::begin(0, "unused");
    let ret = fn_00878670::rw_00878670(addr_of(&buf[0]), args[0], args[1], args[2]);
    let rewrite_log = harness::take_log();

    let (lift_ret, lift_log) = if mutant != 0 {
        let kids: Vec<(u32, u32)> = children[..live]
            .iter()
            .map(|c| (addr_of(&**c), c.answer))
            .collect();
        let mut l = Vec::new();
        let r = if live == 0 {
            true
        } else {
            poll_model(mutant, &kids, mode, args, &mut l)
        };
        (u32::from(r), l)
    } else {
        let log = new_log();
        let lifted: Vec<Box<dyn PollChild>> = children[..live]
            .iter()
            .map(|c| {
                Box::new(ScriptedChild {
                    addr: addr_of(&**c),
                    answer: c.answer,
                    log: log.clone(),
                }) as Box<dyn PollChild>
            })
            .collect();
        let mut filter = CombiningFilter::new(lifted, mode);
        let r = filter.poll(args[0], args[1], args[2]);
        let l = log.borrow().clone();
        (u32::from(r), l)
    };

    if lift_ret == ret && lift_log == rewrite_log {
        Ok(())
    } else {
        Err(format!(
            "poll count {count:#x} mode {mode:#x}: rewrite {ret} {rewrite_log:?}, lift {lift_ret} {lift_log:?}"
        ))
    }
}

#[test]
fn poll_matches_rewrite() {
    let _guard = harness::lock();
    let mut rng = Rng(0x1A17_0003);
    for _ in 0..TRIALS {
        poll_trial(&mut rng, 0).unwrap();
    }
}

#[test]
fn poll_wrong_copies_are_caught() {
    let _guard = harness::lock();
    for mutant in 1..=3u8 {
        let mut rng = Rng(0x1A17_0003);
        assert!(
            (0..TRIALS).any(|_| poll_trial(&mut rng, mutant).is_err()),
            "wrong poll copy {mutant} was not caught"
        );
    }
}

/// One trial of fold. `wrong` leaves the rotation out.
fn fold_trial(rng: &mut Rng, wrong: bool) -> Result<(), String> {
    let mut buf: Box<[u32; 16]> = Box::new([0; 16]);
    for w in buf.iter_mut() {
        *w = rng.word();
    }
    let before = *buf;
    let block = addr_of(&buf[0]);
    let answer = rng.word();

    harness::install(0, 0);
    harness::begin(answer, "base.answer");
    let ret = fn_008786E0::rw_008786E0(block);
    let rewrite_log = harness::take_log();
    if *buf != before {
        return Err("fold wrote to the filter".into());
    }

    let log = new_log();
    let lift_ret = if wrong {
        before[MODE_WORD] ^ answer
    } else {
        let mut base = FakeBase {
            answer,
            log: log.clone(),
        };
        let filter = CombiningFilter::new(Vec::new(), before[MODE_WORD]);
        filter.fold(Handle32::new(block), &mut base)
    };
    let lift_log = log.borrow().clone();

    if lift_ret == ret && (wrong || lift_log == rewrite_log) {
        Ok(())
    } else {
        Err(format!(
            "fold answer {answer:#x}: rewrite {ret:#x} {rewrite_log:?}, lift {lift_ret:#x} {lift_log:?}"
        ))
    }
}

#[test]
fn fold_matches_rewrite() {
    let _guard = harness::lock();
    let mut rng = Rng(0x1A17_0004);
    for _ in 0..TRIALS {
        fold_trial(&mut rng, false).unwrap();
    }
}

#[test]
fn fold_without_rotation_is_caught() {
    let _guard = harness::lock();
    let mut rng = Rng(0x1A17_0004);
    assert!((0..TRIALS).any(|_| fold_trial(&mut rng, true).is_err()));
}
