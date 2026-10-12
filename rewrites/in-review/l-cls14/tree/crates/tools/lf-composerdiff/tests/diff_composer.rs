//! Differential tests of the lifted composer sequence and forwarding methods
//! against their verified 32-bit rewrites: the sequence step (virtual slot 20),
//! the sequence step that returns a word (slot 25), the two-word forwards
//! (slots 19 and 24) and the one-word forwards (slots 21 and 26).
//!
//! Each case builds a real 32-bit composer object, runs the rewrite and the
//! lifted method on the same inputs, and compares results, every call in
//! order with its arguments, and the object's words. The immediate-mode
//! byte takes zero, one and a value with other bits set, so the sync is
//! both taken and skipped. Each method has a deliberately wrong copy that
//! must be caught on the same inputs.

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

use std::cell::RefCell;
use std::rc::Rc;

use lf_animation::composer::optimized::SEQUENCE_STEP;
use lf_animation::composer::{
    ComposerCallees, ComposerId, ComposerOptimized, HandlerId, InnerId, ItemStack, NodeId, OutId,
};
use lf_composerdiff::harness::{self, Call, Rng, addr_of};
use lf_composerdiff::rewrites::{
    fn_00877860, fn_00877890, fn_00878180, fn_00878200, fn_00878250, fn_00879030, fn_008781B0,
    fn_008781D0, fn_00878290,
    fn_008782C0,
    fn_008782E0,
};
use lf_core::boundary::Handle32;

/// Trials per method.
const TRIALS: usize = 400;
/// Bytes of the composer object the methods touch (offset 0x98 is the last).
const OBJECT_BYTES: usize = 0xA0;
/// Byte offsets of the words the methods read and write.
const WORD_18: usize = 0x18;
const SEQUENCE: usize = 0x72;
const IMMEDIATE: usize = 0x98;
const LINKED: usize = 0x11;
const INNER: usize = 0x0C;
const FREE_NODE: usize = 0x88;

/// The six methods under test, in the order of the table below.
const METHODS: [&str; 6] = [
    "vf20 advance sequence",
    "vf25 advance and read word",
    "vf19 two-word forward (pose)",
    "vf24 two-word forward (tracks)",
    "vf21 one-word forward (copy)",
    "vf26 one-word forward (setup)",
];

/// The callee name each forwarder's numbered callee is recorded under.
const CALLEE_NAMES: [&str; 6] = [
    "unused",
    "unused",
    "apply_pose_direct",
    "apply_static_tracks_to_pose",
    "copy_dof_pair",
    "setup_identity_frame",
];

type Log = Rc<RefCell<Vec<Call>>>;

/// The composer's callees, logging each call as the rewrite records it.
struct FakeCallees {
    log: Log,
    answer: u32,
}

fn raw(id: Option<ComposerId>) -> u32 {
    Handle32::raw_or_zero(id)
}

/// The raw value of an out-word identity.
fn raw_out(id: Option<OutId>) -> u32 {
    Handle32::raw_or_zero(id)
}

impl ComposerCallees for FakeCallees {
    fn sync(&mut self, composer: Option<ComposerId>) {
        self.log.borrow_mut().push(("sync", vec![raw(composer)]));
    }

    fn check(&mut self, composer: Option<ComposerId>, token: Option<OutId>) -> u32 {
        self.log
            .borrow_mut()
            .push(("check", vec![raw(composer), raw_out(token)]));
        self.answer
    }

    fn handle_node(&mut self, composer: Option<ComposerId>, node: Option<NodeId>) {
        self.log
            .borrow_mut()
            .push(("handle_node", vec![raw(composer), Handle32::raw_or_zero(node)]));
    }

    fn handle_table_entry(&mut self, entry: Option<HandlerId>, a: u32, b: u32, c: u32, d: u32, e: u32) {
        self.log.borrow_mut().push((
            "handle_table_entry",
            vec![Handle32::raw_or_zero(entry), a, b, c, d, e],
        ));
    }

    fn drain(&mut self, composer: Option<ComposerId>) {
        self.log.borrow_mut().push(("drain", vec![raw(composer)]));
    }

    fn forward_inner(&mut self, inner: Option<InnerId>) -> u32 {
        self.log
            .borrow_mut()
            .push(("forward_inner", vec![Handle32::raw_or_zero(inner)]));
        self.answer
    }

    fn apply_pose_direct(&mut self, composer: Option<ComposerId>, a0: u32, a1: u32, last: Option<ComposerId>) {
        self.log
            .borrow_mut()
            .push(("apply_pose_direct", vec![raw(composer), a0, a1, raw(last)]));
    }

    fn apply_static_tracks_to_pose(
        &mut self,
        composer: Option<ComposerId>,
        a0: u32,
        a1: u32,
        last: Option<ComposerId>,
    ) {
        self.log.borrow_mut().push((
            "apply_static_tracks_to_pose",
            vec![raw(composer), a0, a1, raw(last)],
        ));
    }

    fn copy_dof_pair(&mut self, composer: Option<ComposerId>, a0: u32, a1: u32, last: Option<ComposerId>) {
        self.log
            .borrow_mut()
            .push(("copy_dof_pair", vec![raw(composer), a0, a1, raw(last)]));
    }

    fn setup_identity_frame(
        &mut self,
        composer: Option<ComposerId>,
        a0: u32,
        a1: u32,
        last: Option<ComposerId>,
    ) {
        self.log
            .borrow_mut()
            .push(("setup_identity_frame", vec![raw(composer), a0, a1, raw(last)]));
    }
}

/// Runs the rewrite of method `m` on the object at `this`.
fn run_rewrite(m: usize, this: u32, a0: u32, a1: u32) -> u32 {
    match m {
        0 => fn_008781B0::rw_008781B0(this, a0, a1),
        1 => fn_008782C0::rw_008782C0(this, a0, a1),
        2 => fn_00878180::rw_00878180(this, a0, a1, 0),
        3 => fn_00878290::rw_00878290(this, a0, a1, 0),
        4 => fn_008781D0::rw_008781D0(this, a0, 0),
        _ => fn_008782E0::rw_008782E0(this, a0, 0),
    }
}

fn read_u16(buf: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([buf[at], buf[at + 1]])
}

fn read_u32(buf: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([buf[at], buf[at + 1], buf[at + 2], buf[at + 3]])
}

/// The composer's words as the lifted method is told them.
fn lift_from(buf: &[u8]) -> ComposerOptimized {
    ComposerOptimized::new(
        buf[IMMEDIATE] != 0,
        buf[LINKED] != 0,
        read_u16(buf, SEQUENCE),
        read_u32(buf, WORD_18),
        Handle32::new(read_u32(buf, FREE_NODE)),
        Handle32::new(read_u32(buf, INNER)),
    )
}

/// The immediate byte the trial writes: zero, one, or a value with other bits.
fn immediate_byte(rng: &mut Rng) -> u8 {
    match rng.u32() % 3 {
        0 => 0,
        1 => 1,
        _ => 0x80,
    }
}

/// Runs the lifted method `m` (or a wrong copy when `mutant` is nonzero) and
/// returns its result and the sequence counter it leaves.
fn run_lift(m: usize, mutant: u8, this: u32, buf: &[u8], a0: u32, a1: u32, log: &Log) -> (u32, u16) {
    let composer = Handle32::new(this);
    let mut callees = FakeCallees {
        log: log.clone(),
        answer: 0,
    };
    let mut comp = lift_from(buf);
    if mutant != 0 {
        return run_mutant(m, mutant, this, &comp, a0, a1, log);
    }
    let ret = match m {
        0 => {
            comp.advance_sequence(composer, &mut callees);
            0
        }
        1 => comp.advance_and_read_word(composer, &mut callees),
        2 => {
            comp.forward_two_words_pose(composer, a0, a1, &mut callees);
            0
        }
        3 => {
            comp.forward_two_words_tracks(composer, a0, a1, &mut callees);
            0
        }
        4 => {
            comp.forward_one_word_copy(composer, a0, &mut callees);
            0
        }
        _ => {
            comp.forward_one_word_setup(composer, a0, &mut callees);
            0
        }
    };
    (ret, comp.sequence())
}

/// A wrong copy of method `m`: the sequence step is one, the word or the
/// sequence is returned in the wrong place, the arguments are swapped, the
/// sync condition is inverted or dropped, or the zero is the first word.
fn run_mutant(m: usize, mutant: u8, this: u32, comp: &ComposerOptimized, a0: u32, a1: u32, log: &Log) -> (u32, u16) {
    let sync = |log: &Log| {
        log.borrow_mut().push(("sync", vec![this]));
    };
    let want_sync = match mutant {
        2 => comp.immediate(),
        3 => false,
        _ => !comp.immediate(),
    };
    let mut seq = comp.sequence();
    let ret = match m {
        0 => {
            if want_sync {
                sync(log);
            }
            seq = seq.wrapping_add(if mutant == 1 { 1 } else { SEQUENCE_STEP });
            0
        }
        1 => {
            if want_sync {
                sync(log);
            }
            seq = seq.wrapping_add(SEQUENCE_STEP);
            if mutant == 1 { u32::from(seq) } else { comp.word_18() }
        }
        _ => {
            if want_sync {
                sync(log);
            }
            let (x, y) = if mutant == 1 { (a1, a0) } else { (a0, a1) };
            let name = CALLEE_NAMES[m];
            let zero = if m >= 4 && mutant == 4 { a0 } else { 0 };
            let second = if m >= 4 { zero } else { y };
            let first = if m >= 4 { a0 } else { x };
            log.borrow_mut().push((name, vec![this, first, second, 0]));
            0
        }
    };
    (ret, seq)
}

/// One trial of method `m`; `mutant` 0 runs the lifted method.
fn trial(rng: &mut Rng, m: usize, mutant: u8) -> Result<(), String> {
    let mut buf: Box<[u8; OBJECT_BYTES]> = Box::new([0; OBJECT_BYTES]);
    for b in buf.iter_mut() {
        *b = rng.u32() as u8;
    }
    let vt = harness::composer_vtable();
    buf[0..4].copy_from_slice(&addr_of(&vt[0]).to_le_bytes());
    buf[IMMEDIATE] = immediate_byte(rng);
    let this = addr_of(&buf[0]);
    let before = *buf;
    let a0 = rng.word();
    let a1 = rng.word();

    harness::install(0, 0);
    harness::begin_forward(0, "unused", CALLEE_NAMES[m]);
    let ret = run_rewrite(m, this, a0, a1);
    let rewrite_log = harness::take_log();
    let rewrite_seq = read_u16(&*buf, SEQUENCE);

    // The rewrite writes only the sequence word, and only in the two
    // sequence methods.
    let seq_writes = m < 2;
    for (i, (now, was)) in buf.iter().zip(before.iter()).enumerate() {
        let seq_bytes = i == SEQUENCE || i == SEQUENCE + 1;
        if now != was && !(seq_writes && seq_bytes) {
            return Err(format!("{}: rewrite wrote byte {i:#x}", METHODS[m]));
        }
    }

    let log = Rc::new(RefCell::new(Vec::new()));
    let (lift_ret, lift_seq) = run_lift(m, mutant, this, &before[..], a0, a1, &log);
    let lift_log = log.borrow().clone();

    let want_seq = if seq_writes { rewrite_seq } else { read_u16(&before, SEQUENCE) };
    let ok = lift_ret == ret
        && lift_log == rewrite_log
        && (!seq_writes || (lift_seq == want_seq && lift_seq == before_seq_plus_step(&before)));
    if ok {
        Ok(())
    } else {
        Err(format!(
            "{} imm {} seq {:#x} a0 {a0:#x} a1 {a1:#x}: rewrite {ret:#x} {rewrite_log:?} seq {rewrite_seq:#x}, lift {lift_ret:#x} {lift_log:?} seq {lift_seq:#x}",
            METHODS[m],
            before[IMMEDIATE],
            read_u16(&before, SEQUENCE),
        ))
    }
}

/// The sequence counter as the rewrite must leave it, from the counter the
/// trial started with.
fn before_seq_plus_step(before: &[u8]) -> u16 {
    read_u16(before, SEQUENCE).wrapping_add(SEQUENCE_STEP)
}

#[test]
fn every_method_matches_its_rewrite() {
    let _guard = harness::lock();
    for m in 0..METHODS.len() {
        let mut rng = Rng(0x1A17_0020 + m as u64);
        for _ in 0..TRIALS {
            trial(&mut rng, m, 0).unwrap();
        }
    }
}

#[test]
fn every_method_has_a_wrong_copy_that_is_caught() {
    let _guard = harness::lock();
    for m in 0..METHODS.len() {
        let mutant: u8 = match m {
            0 => 1,
            1 => 1,
            2 | 3 => 1,
            4 | 5 => 4,
            _ => 1,
        };
        let mut rng = Rng(0x1A17_0020 + m as u64);
        assert!(
            (0..TRIALS).any(|_| trial(&mut rng, m, mutant).is_err()),
            "wrong copy of {} was not caught",
            METHODS[m]
        );
    }
}

#[test]
fn sync_skipped_when_immediate_is_caught() {
    let _guard = harness::lock();
    for m in 0..METHODS.len() {
        let mut rng = Rng(0x1A17_0020 + m as u64);
        assert!(
            (0..TRIALS).any(|_| trial(&mut rng, m, 3).is_err()),
            "dropped sync in {} was not caught",
            METHODS[m]
        );
    }
}

/// Words of the inner object the stack read uses: the table pointer at
/// offset 4 and the top index at offset 0x14.
const INNER_TABLE: usize = 0x04 / 4;
const INNER_TOP: usize = 0x14 / 4;
/// Table entries in the inner stack the stack-read trials build.
const TABLE_WORDS: usize = 8;

/// Writes a little-endian word into a byte object.
fn put_u32(buf: &mut [u8], at: usize, v: u32) {
    buf[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

/// A composer buffer with a vtable whose sync slot is the sync stub.
fn fresh_composer(rng: &mut Rng, vt: &[u32; 40]) -> Box<[u8; OBJECT_BYTES]> {
    let mut buf: Box<[u8; OBJECT_BYTES]> = Box::new([0; OBJECT_BYTES]);
    for b in buf.iter_mut() {
        *b = rng.u32() as u8;
    }
    put_u32(&mut *buf, 0, addr_of(&vt[0]));
    buf[IMMEDIATE] = immediate_byte(rng);
    buf
}

/// One trial of the drain-and-forward method (slot 2 of the class). `mutant`
/// 1 ignores the linked flag; 2 leaves the free-node cache in place.
fn drain_trial(rng: &mut Rng, mutant: u8) -> Result<(), String> {
    let vt = harness::composer_vtable();
    let mut buf = fresh_composer(rng, &vt);
    buf[LINKED] = u8::from(rng.u32() % 4 == 0);
    let inner_vt = harness::inner_vtable();
    let mut inner: Box<[u32; 6]> = Box::new([0; 6]);
    for w in inner.iter_mut() {
        *w = rng.word();
    }
    inner[0] = addr_of(&inner_vt[0]);
    put_u32(&mut *buf, INNER, addr_of(&inner[0]));
    put_u32(&mut *buf, FREE_NODE, rng.word() | 1);
    let before = *buf;
    let this = addr_of(&buf[0]);
    let answer = rng.word();

    harness::install(0, 0);
    harness::select_callee2(1);
    harness::begin_forward(answer, "drain", "unused");
    let ret = fn_00877860::rw_00877860(this);
    let rewrite_log = harness::take_log();
    let after = *buf;

    // The rewrite changes only the free-node word, and only when not linked.
    let linked = before[LINKED] != 0;
    for (i, (now, was)) in after.iter().zip(before.iter()).enumerate() {
        let free_word = (FREE_NODE..FREE_NODE + 4).contains(&i);
        if now != was && (linked || !free_word) {
            return Err(format!("drain wrote byte {i:#x}"));
        }
    }

    let log = new_log();
    let mut callees = FakeCallees {
        log: log.clone(),
        answer,
    };
    let mut comp = if mutant == 1 {
        ComposerOptimized::new(
            lift_from(&before).immediate(),
            false,
            read_u16(&before, SEQUENCE),
            read_u32(&before, WORD_18),
            Handle32::new(read_u32(&before, FREE_NODE)),
            Handle32::new(read_u32(&before, INNER)),
        )
    } else {
        lift_from(&before)
    };
    let id = Handle32::new(this);
    let got = comp.drain_and_forward(id, &mut callees);
    let lift_log = log.borrow().clone();
    // Mutant 2 keeps the free-node word it started with.
    let free_after = if mutant == 2 {
        lift_from(&before).free_node()
    } else {
        comp.free_node()
    };
    let free_ok = Handle32::raw_or_zero(free_after) == read_u32(&after, FREE_NODE);

    if got == ret && lift_log == rewrite_log && free_ok {
        Ok(())
    } else {
        Err(format!(
            "drain linked {linked}: rewrite {ret:#x} {rewrite_log:?}, lift {got:#x} {lift_log:?}, free ok {free_ok}"
        ))
    }
}

#[test]
fn drain_and_forward_matches_rewrite() {
    let _guard = harness::lock();
    let mut rng = Rng(0x1A17_0030);
    for _ in 0..TRIALS {
        drain_trial(&mut rng, 0).unwrap();
    }
}

#[test]
fn drain_and_forward_wrong_copies_are_caught() {
    let _guard = harness::lock();
    for mutant in 1..=2u8 {
        let mut rng = Rng(0x1A17_0030);
        assert!(
            (0..TRIALS).any(|_| drain_trial(&mut rng, mutant).is_err()),
            "wrong drain copy {mutant} was not caught"
        );
    }
}

/// One trial of the stack read (slot 23 of the class). `mutant` 1 inverts
/// the pop decision; 2 runs the sync when immediate mode is on.
fn peek_trial(rng: &mut Rng, mutant: u8) -> Result<(), String> {
    let vt = harness::composer_vtable();
    let mut buf = fresh_composer(rng, &vt);
    let mut table: Box<[u32; TABLE_WORDS]> = Box::new([0; TABLE_WORDS]);
    for w in table.iter_mut() {
        *w = rng.word();
    }
    let mut inner: Box<[u32; 6]> = Box::new([0; 6]);
    inner[INNER_TABLE] = addr_of(&table[0]);
    inner[INNER_TOP] = rng.u32() % TABLE_WORDS as u32;
    put_u32(&mut *buf, INNER, addr_of(&inner[0]));
    let mode = if rng.u32() % 2 == 0 {
        rng.word() & !0xFF
    } else {
        rng.word() | 0x01
    };
    let before = *buf;
    let inner_before = *inner;
    let table_before = *table;
    let this = addr_of(&buf[0]);

    harness::install(0, 0);
    harness::begin_forward(0, "unused", "unused");
    let ret = fn_00878250::rw_00878250(this, mode);
    let rewrite_log = harness::take_log();
    let inner_after = *inner;

    if *buf != before {
        return Err("stack read wrote the composer".into());
    }
    if *table != table_before {
        return Err("stack read wrote the table".into());
    }
    for (i, (now, was)) in inner_after.iter().zip(inner_before.iter()).enumerate() {
        if i != INNER_TOP && now != was {
            return Err(format!("stack read wrote inner word {i}"));
        }
    }

    let log = new_log();
    let mut callees = FakeCallees {
        log: log.clone(),
        answer: 0,
    };
    let lifted = lift_from(&before);
    let mut stack = ItemStack {
        table: table_before.to_vec(),
        top: inner_before[INNER_TOP],
    };
    let lift_mode = if mutant == 1 {
        if (mode as u8) == 0 { 1 } else { 0 }
    } else {
        mode
    };
    let got = lifted.peek_or_pop(Some(Handle32::new(this).unwrap()), lift_mode, &mut stack, &mut callees);
    let mut lift_log = log.borrow().clone();
    if mutant == 2 && lifted.immediate() {
        lift_log.insert(0, ("sync", vec![this]));
    }

    let top_ok = stack.top == inner_after[INNER_TOP];
    if got == ret && lift_log == rewrite_log && top_ok {
        Ok(())
    } else {
        Err(format!(
            "stack read mode {mode:#x}: rewrite {ret:#x} {rewrite_log:?}, lift {got:#x} {lift_log:?}, top ok {top_ok}"
        ))
    }
}

#[test]
fn peek_or_pop_matches_rewrite() {
    let _guard = harness::lock();
    let mut rng = Rng(0x1A17_0031);
    for _ in 0..TRIALS {
        peek_trial(&mut rng, 0).unwrap();
    }
}

#[test]
fn peek_or_pop_wrong_copies_are_caught() {
    let _guard = harness::lock();
    for mutant in 1..=2u8 {
        let mut rng = Rng(0x1A17_0031);
        assert!(
            (0..TRIALS).any(|_| peek_trial(&mut rng, mutant).is_err()),
            "wrong stack-read copy {mutant} was not caught"
        );
    }
}

/// A new call log for one lifted run.
fn new_log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

/// Word at offset 0x70 of the composer: a 16-bit value the table dispatch
/// passes on.
const WORD_70: usize = 0x70;

/// One trial of the table dispatch (slot 22 of the class). `mutant` 1 reads
/// the entry one place past the top; 2 advances the sequence by two.
fn dispatch_trial(rng: &mut Rng, mutant: u8) -> Result<(), String> {
    let vt = harness::composer_vtable();
    let mut buf = fresh_composer(rng, &vt);
    let mut table: Box<[u32; TABLE_WORDS + 1]> = Box::new([0; TABLE_WORDS + 1]);
    for w in table.iter_mut() {
        *w = rng.word();
    }
    let mut inner: Box<[u32; 6]> = Box::new([0; 6]);
    inner[INNER_TABLE] = addr_of(&table[0]);
    inner[INNER_TOP] = rng.u32() % TABLE_WORDS as u32;
    put_u32(&mut *buf, INNER, addr_of(&inner[0]));
    let a0 = rng.word();
    let before = *buf;
    let inner_before = *inner;
    let table_before = *table;
    let this = addr_of(&buf[0]);

    harness::install(0, 0);
    harness::select_callee2(6);
    harness::begin_forward(0, "unused", "handle_table_entry");
    let ret = fn_00878200::rw_00878200(this, a0, 0);
    let rewrite_log = harness::take_log();
    let rewrite_seq = read_u16(&*buf, SEQUENCE);

    // The rewrite changes only the sequence word.
    for (i, (now, was)) in buf.iter().zip(before.iter()).enumerate() {
        let seq_bytes = i == SEQUENCE || i == SEQUENCE + 1;
        if now != was && !seq_bytes {
            return Err(format!("dispatch wrote composer byte {i:#x}"));
        }
    }
    if *inner != inner_before || *table != table_before {
        return Err("dispatch wrote the inner object or its table".into());
    }

    let log = new_log();
    let mut callees = FakeCallees {
        log: log.clone(),
        answer: 0,
    };
    let mut lifted = lift_from(&before).with_word_70(read_u16(&before, WORD_70));
    // The wrong copy reads one entry further on; the table has room for it.
    let stack = ItemStack {
        table: table_before.to_vec(),
        top: inner_before[INNER_TOP] + u32::from(mutant == 1),
    };
    lifted.dispatch_table_entry(Some(Handle32::new(this).unwrap()), a0, &stack, &mut callees);
    let lift_log = log.borrow().clone();
    let lift_seq = if mutant == 2 {
        lifted.sequence().wrapping_add(1)
    } else {
        lifted.sequence()
    };

    if ret == 0 && lift_seq == rewrite_seq && lift_log == rewrite_log {
        Ok(())
    } else {
        Err(format!(
            "dispatch: rewrite ret {ret:#x} seq {rewrite_seq:#x} {rewrite_log:?}, lift seq {lift_seq:#x} {lift_log:?}"
        ))
    }
}

#[test]
fn table_dispatch_matches_rewrite() {
    let _guard = harness::lock();
    let mut rng = Rng(0x1A17_0040);
    for _ in 0..TRIALS {
        dispatch_trial(&mut rng, 0).unwrap();
    }
}

#[test]
fn table_dispatch_wrong_copies_are_caught() {
    let _guard = harness::lock();
    for mutant in 1..=2u8 {
        let mut rng = Rng(0x1A17_0040);
        assert!(
            (0..TRIALS).any(|_| dispatch_trial(&mut rng, mutant).is_err()),
            "wrong dispatch copy {mutant} was not caught"
        );
    }
}

/// Symbol that stands for the out-word's address in compared logs: the
/// rewrite and the lift each hold their own out-word at a different address.
const OUT_SYMBOL: u32 = 0xD0D0_0001;

/// Replaces every argument equal to `addr` with [`OUT_SYMBOL`].
fn symbolise(log: Vec<Call>, addr: u32) -> Vec<Call> {
    log.into_iter()
        .map(|(name, args)| {
            let args = args
                .into_iter()
                .map(|a| if a == addr { OUT_SYMBOL } else { a })
                .collect();
            (name, args)
        })
        .collect()
}

/// One trial of the gate-drain-forward method (slot 6 of the class). `mutant`
/// 1 reads the whole gate answer as the verdict; 2 leaves the out-word alone.
fn gate_trial(rng: &mut Rng, mutant: u8) -> Result<(), String> {
    let vt = harness::composer_vtable();
    let mut buf = fresh_composer(rng, &vt);
    buf[LINKED] = u8::from(rng.u32() % 4 == 0);
    let inner_vt = harness::inner_vtable();
    let mut inner: Box<[u32; 6]> = Box::new([0; 6]);
    inner[0] = addr_of(&inner_vt[0]);
    put_u32(&mut *buf, INNER, addr_of(&inner[0]));
    put_u32(&mut *buf, FREE_NODE, rng.word() | 1);
    let before = *buf;
    let this = addr_of(&buf[0]);
    // The gate's answer: a verdict in its low byte, or only high bits.
    let answer = if rng.u32() % 2 == 0 {
        rng.answer()
    } else {
        rng.u32() | 0x01
    };
    let out_before = rng.word();

    let out_rw = out_before;
    let rw_addr = addr_of(&out_rw);
    harness::install(0, 0);
    harness::select_callee2(1);
    harness::begin_forward(answer, "drain", "unused");
    let ret = fn_00877890::rw_00877890(this, rw_addr);
    let rewrite_log = symbolise(harness::take_log(), rw_addr);

    // Writes: the free-node word, the linked byte and the out-word, and
    // only when the gate accepts.
    let accepted = (answer as u8) != 0;
    let after = *buf;
    for (i, (now, was)) in after.iter().zip(before.iter()).enumerate() {
        let free_word = (FREE_NODE..FREE_NODE + 4).contains(&i);
        let linked_byte = i == LINKED;
        if now != was && !(accepted && (free_word || linked_byte)) {
            return Err(format!("gate wrote composer byte {i:#x}"));
        }
    }
    if (out_rw != out_before) && !accepted {
        return Err("gate wrote the out-word on a rejection".into());
    }

    let log = new_log();
    let mut callees = FakeCallees {
        log: log.clone(),
        answer,
    };
    let mut comp = lift_from(&before);
    let mut out_lift = out_before;
    let lift_addr = addr_of(&out_lift);
    let lift_ret = if mutant == 1 {
        // Reads the whole word as the verdict: an answer with only high bits
        // set is accepted. The lift sees such an answer as its low byte, so
        // the wrong copy hands it an answer whose low byte is one.
        if answer != 0 {
            let mut wrong = FakeCallees {
                log: log.clone(),
                answer: 1,
            };
            comp.gate_drain_forward(
                Handle32::new(this),
                Handle32::new(lift_addr),
                &mut out_lift,
                &mut wrong,
            )
        } else {
            false
        }
    } else {
        comp.gate_drain_forward(
            Handle32::new(this),
            Handle32::new(lift_addr),
            &mut out_lift,
            &mut callees,
        )
    };
    let lift_log = symbolise(log.borrow().clone(), lift_addr);
    let lift_out_ok = if mutant == 2 { out_before } else { out_lift };
    let want_out = if accepted { 0 } else { out_before };
    let state_ok = lift_out_ok == want_out
        && (lift_ret == accepted)
        && (accepted == (lift_ret))
        && comp.linked() == (before[LINKED] != 0 && !accepted)
        && Handle32::raw_or_zero(comp.free_node()) == read_u32(&after, FREE_NODE);
    let ret_ok = ret == u32::from(accepted);

    if ret_ok && lift_log == rewrite_log && state_ok {
        Ok(())
    } else {
        Err(format!(
            "gate answer {answer:#x}: rewrite {ret:#x} {rewrite_log:?}, lift {lift_ret} {lift_log:?}, state ok {state_ok}"
        ))
    }
}

#[test]
fn gate_drain_forward_matches_rewrite() {
    let _guard = harness::lock();
    let mut rng = Rng(0x1A17_0050);
    for _ in 0..TRIALS {
        gate_trial(&mut rng, 0).unwrap();
    }
}

#[test]
fn gate_drain_forward_wrong_copies_are_caught() {
    let _guard = harness::lock();
    for mutant in 1..=2u8 {
        let mut rng = Rng(0x1A17_0050);
        assert!(
            (0..TRIALS).any(|_| gate_trial(&mut rng, mutant).is_err()),
            "wrong gate copy {mutant} was not caught"
        );
    }
}

/// One trial of the node handler (slot 33 of the class). `mutant` 1 leaves
/// the done word alone; 2 reports failure.
fn handler_trial(rng: &mut Rng, mutant: u8) -> Result<(), String> {
    let vt = harness::composer_vtable();
    let buf = fresh_composer(rng, &vt);
    let before = *buf;
    let this = addr_of(&buf[0]);
    let node = rng.word();

    let done_rw = rng.word();
    let done_addr = addr_of(&done_rw);
    harness::install(0, 0);
    harness::begin_forward(0, "unused", "unused");
    let ret = fn_00879030::rw_00879030(this, node, done_addr);
    let rewrite_log = harness::take_log();
    if *buf != before {
        return Err("handler wrote the composer".into());
    }



    let log = new_log();
    let mut callees = FakeCallees {
        log: log.clone(),
        answer: 0,
    };
    let mut done_lift = rng.word();
    let start = done_lift;
    let ok = lift_from(&before).handle_pending_node(
        Some(Handle32::new(this).unwrap()),
        Handle32::new(node),
        &mut done_lift,
        &mut callees,
    );
    let lift_log = log.borrow().clone();
    let lift_ret = if mutant == 2 { 0 } else { u32::from(ok) };
    let lift_done = if mutant == 1 { start } else { done_lift };
    let done_ok = lift_done == 0 && done_rw == 0;

    if ret == lift_ret && lift_log == rewrite_log && done_ok {
        Ok(())
    } else {
        Err(format!(
            "handler node {node:#x}: rewrite {ret} {rewrite_log:?} done {done_rw:#x}, lift {lift_ret} {lift_log:?} done {lift_done:#x}"
        ))
    }
}

#[test]
fn handle_pending_node_matches_rewrite() {
    let _guard = harness::lock();
    let mut rng = Rng(0x1A17_0060);
    for _ in 0..TRIALS {
        handler_trial(&mut rng, 0).unwrap();
    }
}

#[test]
fn handle_pending_node_wrong_copies_are_caught() {
    let _guard = harness::lock();
    for mutant in 1..=2u8 {
        let mut rng = Rng(0x1A17_0060);
        assert!(
            (0..TRIALS).any(|_| handler_trial(&mut rng, mutant).is_err()),
            "wrong handler copy {mutant} was not caught"
        );
    }
}
