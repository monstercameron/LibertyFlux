//! Differential tests of the lifted composer data block against its
//! verified 32-bit rewrites: the scalar deleting destructor (virtual slot 0)
//! and the table reset (slot 2).
//!
//! Each case builds real 32-bit objects, runs the rewrite and the lifted
//! method on the same inputs, and compares results, every call in order
//! with its arguments, and every word. The reset is checked word by word:
//! the words the lift holds must match what the rewrite wrote, and every
//! other word must be exactly as it was. Deliberately wrong copies of each
//! method must be caught on the same inputs.

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

use std::cell::RefCell;
use std::rc::Rc;

use lf_animation::composer::optimized_data::{
    COUNT_OFFSET, FREE_FLAG, TABLE_COUNT, TABLE_OFFSET, TABLE_STRIDE,
    TABLE_TARGET_SKEW, VALUE_COUNT, VALUE_STRIDE, VALUES_OFFSET,
};
use lf_animation::composer::{
    BlockId, ComposerDataCallees, ComposerOptimizedData, ObjectManager, TableEntry, ValueEntry,
};
use lf_composerdiff::harness::{self, Call, Rng, VtObj, addr_of};
use lf_composerdiff::rewrites::{fn_008700E0, fn_00870180};
use lf_core::boundary::Handle32;

/// Trials per case.
const TRIALS: usize = 300;
/// Words in the block the reset touches (its count word is the last word
/// of the tables; the value table ends a little past it).
const BLOCK_WORDS: usize = 0x1B20 / 4;
/// Return value the reset gives: the last table entry's target.
const LAST_TARGET: u32 = TABLE_OFFSET + (TABLE_COUNT as u32 - 1) * TABLE_STRIDE + TABLE_TARGET_SKEW;

type Log = Rc<RefCell<Vec<Call>>>;

fn new_log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

/// The block's callees, as the lifted code calls them.
struct FakeCallees {
    log: Log,
}

impl ComposerDataCallees for FakeCallees {
    fn reset_optimized(&mut self, block: Option<BlockId>) {
        self.log
            .borrow_mut()
            .push(("reset.optimized", vec![Handle32::raw_or_zero(block)]));
    }

    fn release_child_frames(&mut self, block: Option<BlockId>, offset: u32) {
        self.log.borrow_mut().push((
            "release.child.frames",
            vec![Handle32::raw_or_zero(block).wrapping_add(offset)],
        ));
    }
}

/// The thread's manager, as the lifted code frees through it.
struct FakeManager {
    addr: u32,
    log: Log,
}

impl ObjectManager for FakeManager {
    fn free_object(&mut self, block: BlockId) {
        self.log
            .borrow_mut()
            .push(("manager.free", vec![self.addr, block.get()]));
    }
}

/// Word indices the reset is allowed to write: each value entry's two
/// words, each table entry's word, and the count word.
fn modelled_words() -> Vec<bool> {
    let mut m = vec![false; BLOCK_WORDS];
    for i in 0..VALUE_COUNT as u32 {
        let at = (VALUES_OFFSET + i * VALUE_STRIDE) / 4;
        m[at as usize] = true;
        m[at as usize + 1] = true;
    }
    for i in 0..TABLE_COUNT as u32 {
        m[((TABLE_OFFSET + i * TABLE_STRIDE) / 4) as usize] = true;
    }
    m[(COUNT_OFFSET / 4) as usize] = true;
    m
}

/// What a reset leaves behind, in raw form, so the rewrite's block and the
/// lift's model can be compared word for word.
#[derive(Debug, PartialEq, Eq)]
struct Post {
    values: Vec<(u32, u32)>,
    table: Vec<u32>,
    count: u32,
    ret: u32,
    log: Vec<Call>,
}

/// The state a reset leaves, read from the rewrite's block.
fn post_from_block(buf: &[u32], ret: u32, log: Vec<Call>) -> Post {
    let values = (0..VALUE_COUNT as u32)
        .map(|i| {
            let at = ((VALUES_OFFSET + i * VALUE_STRIDE) / 4) as usize;
            (buf[at], buf[at + 1])
        })
        .collect();
    let table = (0..TABLE_COUNT as u32)
        .map(|i| buf[((TABLE_OFFSET + i * TABLE_STRIDE) / 4) as usize])
        .collect();
    Post {
        values,
        table,
        count: buf[(COUNT_OFFSET / 4) as usize],
        ret,
        log,
    }
}

/// The state the lifted reset leaves, in the same raw form.
fn post_from_lift(data: &ComposerOptimizedData, block: u32, ret: u32, log: Vec<Call>) -> Post {
    Post {
        values: data.values().iter().map(|v| (v.first, v.second)).collect(),
        table: data
            .table()
            .iter()
            .map(|t| block.wrapping_add(t.target))
            .collect(),
        count: data.count(),
        ret: block.wrapping_add(ret),
        log,
    }
}

/// The reset's words before it runs, as the lifted block holds them.
fn lift_from_buffer(buf: &[u32]) -> ComposerOptimizedData {
    let mut values = [ValueEntry::default(); VALUE_COUNT];
    for (i, v) in values.iter_mut().enumerate() {
        let at = ((VALUES_OFFSET + i as u32 * VALUE_STRIDE) / 4) as usize;
        *v = ValueEntry {
            first: buf[at],
            second: buf[at + 1],
        };
    }
    let mut table = [TableEntry::default(); TABLE_COUNT];
    for (i, t) in table.iter_mut().enumerate() {
        let at = ((TABLE_OFFSET + i as u32 * TABLE_STRIDE) / 4) as usize;
        *t = TableEntry { target: buf[at] };
    }
    ComposerOptimizedData::new(values, table, buf[(COUNT_OFFSET / 4) as usize])
}

/// A deliberately wrong reset, described as the state it would leave.
/// `mutant` 1 skews the table targets by one word, 2 keeps the count word,
/// 3 returns the first target, 4 leaves the last value entry untouched.
fn wrong_post(buf_before: &[u32], block: u32, mutant: u8, log: Vec<Call>) -> Post {
    let mut values: Vec<(u32, u32)> = vec![(0, 0); VALUE_COUNT];
    let skew = if mutant == 1 {
        TABLE_TARGET_SKEW + 4
    } else {
        TABLE_TARGET_SKEW
    };
    let table: Vec<u32> = (0..TABLE_COUNT as u32)
        .map(|i| block.wrapping_add(TABLE_OFFSET + i * TABLE_STRIDE + skew))
        .collect();
    if mutant == 4 {
        let at = ((VALUES_OFFSET + (VALUE_COUNT as u32 - 1) * VALUE_STRIDE) / 4) as usize;
        values[VALUE_COUNT - 1] = (buf_before[at], buf_before[at + 1]);
    }
    let count = if mutant == 2 {
        buf_before[(COUNT_OFFSET / 4) as usize]
    } else {
        0
    };
    let ret = if mutant == 3 {
        table[0]
    } else {
        block.wrapping_add(LAST_TARGET)
    };
    Post {
        values,
        table,
        count,
        ret,
        log,
    }
}

/// One trial of the table reset. `mutant` 0 is the lifted method.
fn reset_trial(rng: &mut Rng, mutant: u8) -> Result<(), String> {
    let mut buf: Vec<u32> = (0..BLOCK_WORDS).map(|_| rng.word()).collect();
    let before = buf.clone();
    let block = buf.as_mut_ptr() as usize as u32;

    harness::install(0, 0);
    harness::begin(0, "release.child.frames");
    let ret = fn_00870180::rw_00870180(block);
    let rewrite_log = harness::take_log();

    let modelled = modelled_words();
    for (i, (now, was)) in buf.iter().zip(before.iter()).enumerate() {
        if !modelled[i] && now != was {
            return Err(format!("reset wrote word {i}, which the lift does not model"));
        }
    }
    let rewrite_post = post_from_block(&buf, ret, rewrite_log.clone());

    let lift_post = if mutant != 0 {
        wrong_post(&before, block, mutant, rewrite_log.clone())
    } else {
        let mut data = lift_from_buffer(&before);
        let log = new_log();
        let mut callees = FakeCallees { log: log.clone() };
        let got = data.reset_tables(Handle32::new(block), &mut callees);
        let lifted_log = log.borrow().clone();
        post_from_lift(&data, block, got, lifted_log)
    };

    if lift_post == rewrite_post {
        Ok(())
    } else {
        Err(format!("reset differs: rewrite {rewrite_post:?} lift {lift_post:?}"))
    }
}

#[test]
fn table_reset_matches_rewrite() {
    let _guard = harness::lock();
    let mut rng = Rng(0x1A17_0010);
    for _ in 0..TRIALS {
        reset_trial(&mut rng, 0).unwrap();
    }
}

#[test]
fn table_reset_wrong_copies_are_caught() {
    let _guard = harness::lock();
    for mutant in 1..=4u8 {
        let mut rng = Rng(0x1A17_0010);
        let caught = (0..TRIALS).any(|_| reset_trial(&mut rng, mutant).is_err());
        assert!(caught, "wrong reset copy {mutant} was not caught");
    }
}

/// One trial of the scalar deleting destructor. `mutant` 1 frees even when
/// the block is null, 2 reads the free flag from the wrong bit.
fn destructor_trial(rng: &mut Rng, mutant: u8) -> Result<(), String> {
    let flags = if rng.u32() % 3 == 0 {
        rng.word() & !FREE_FLAG
    } else {
        rng.word() | FREE_FLAG
    };
    let block_words: Box<[u32; 4]> = Box::new([0; 4]);
    let block = if rng.u32() % 4 == 0 {
        0
    } else {
        addr_of(&block_words[0])
    };
    let mgr_vt = harness::manager_vtable();
    let mgr_obj = Box::new(VtObj {
        vtable: addr_of(&mgr_vt[0]),
        spare: 0,
    });
    let mgr_addr = addr_of(&*mgr_obj);
    let thread = Box::new([0u32, 0, mgr_addr, 0]);
    harness::install(addr_of(&thread[0]), 0);

    harness::begin(0, "reset.optimized");
    let ret = fn_008700E0::rw_008700e0(block, flags);
    let rewrite_log = harness::take_log();

    let (lift_ret, lift_log) = if mutant == 0 {
        let log = new_log();
        let mut callees = FakeCallees { log: log.clone() };
        let mut mgr = FakeManager {
            addr: mgr_addr,
            log: log.clone(),
        };
        let got = ComposerOptimizedData::scalar_deleting_destructor(
            Handle32::new(block),
            flags,
            &mut callees,
            Some(&mut mgr),
        );
        (Handle32::raw_or_zero(got), log.borrow().clone())
    } else {
        let mut l = vec![("reset.optimized", vec![block])];
        let frees = match mutant {
            1 => flags & FREE_FLAG != 0,
            _ => flags & 2 != 0,
        };
        if frees {
            l.push(("manager.free", vec![mgr_addr, block]));
        }
        (block, l)
    };

    if lift_ret == ret && lift_log == rewrite_log {
        Ok(())
    } else {
        Err(format!(
            "destructor flags {flags:#x} block {block:#x}: rewrite {ret:#x} {rewrite_log:?}, lift {lift_ret:#x} {lift_log:?}"
        ))
    }
}

#[test]
fn scalar_deleting_destructor_matches_rewrite() {
    let _guard = harness::lock();
    let mut rng = Rng(0x1A17_0011);
    for _ in 0..TRIALS {
        destructor_trial(&mut rng, 0).unwrap();
    }
}

#[test]
fn scalar_deleting_destructor_wrong_copies_are_caught() {
    let _guard = harness::lock();
    for mutant in 1..=2u8 {
        let mut rng = Rng(0x1A17_0011);
        let caught = (0..TRIALS).any(|_| destructor_trial(&mut rng, mutant).is_err());
        assert!(caught, "wrong destructor copy {mutant} was not caught");
    }
}
