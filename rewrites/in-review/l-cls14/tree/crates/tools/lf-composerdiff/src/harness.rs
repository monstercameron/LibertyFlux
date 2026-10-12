//! Differential-test support: scripted callees, virtual-slot stubs, the
//! checker runtime's mirrors, and the 32-bit object layouts the verified
//! rewrites read and write.
//!
//! The rewrites are the checker-verified files, compiled unchanged against
//! the shared checker runtime (`lf_checker_rt`). This module fills in the
//! runtime's mirrors: the image base (so that a relocated vtable address
//! lands on a test-owned vtable), the callee stub table (numbered callee 1
//! lands on a recording stub), and the thread-local mirror (slot 0 names a
//! test-owned thread block). Every call the rewrites make goes through a
//! recording stub; the test compares those logs with the lifted code's
//! logs. Addresses are real, so both sides see the same objects.
//!
//! Test-support code: the lifted module stays `#![forbid(unsafe_code)]`.

// The stubs and the mirrors read and write raw test memory; each access
// stays inside an object the running case built.
#![allow(unsafe_code)]

use std::sync::{Mutex, MutexGuard};

/// One recorded call: the callee's name and its argument words.
pub type Call = (&'static str, Vec<u32>);

/// Scripted answer and call log of the trial that is running.
struct Script {
    log: Vec<Call>,
    answer: u32,
    one_name: &'static str,
    four_name: &'static str,
}

static SCRIPT: Mutex<Script> = Mutex::new(Script {
    log: Vec::new(),
    answer: 0,
    one_name: "",
    four_name: "",
});

static LOCK: Mutex<()> = Mutex::new(());

/// Callee-table mirror: entry 1 is the one-argument stub.
static mut CALLEE_TABLE: [u32; 256] = [0; 256];
/// Thread-local mirror: slot 0 names the thread block.
static mut TLS_MIRROR: [u32; 256] = [0; 256];

/// Serialises the trials of one test binary: the mirrors and the call log
/// are process-wide. A poisoned lock is taken over, so one failed case does
/// not wedge the rest.
pub fn lock() -> MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

fn script() -> MutexGuard<'static, Script> {
    SCRIPT.lock().unwrap_or_else(|e| e.into_inner())
}

/// The address of a test object, as the rewrites see it.
#[must_use]
pub fn addr_of<T>(r: &T) -> u32 {
    (r as *const T) as usize as u32
}

/// Starts a trial: the call log is cleared, the scripted answer is set for
/// every stub that answers, and the one-argument callee is recorded under
/// `one_name`.
pub fn begin(answer: u32, one_name: &'static str) {
    let mut s = script();
    s.log.clear();
    s.answer = answer;
    s.one_name = one_name;
    s.four_name = "";
}

/// Takes the call log of the trial that ran.
pub fn take_log() -> Vec<Call> {
    std::mem::take(&mut script().log)
}

/// Installs the mirrors the rewrites read: the image base, the thread
/// block of slot 0, and the one-argument callee stub.
///
/// `xbase` is the relocated base the rewrites add to file addresses
/// (see [`xbase_for`]).
pub fn install(thread_block: u32, xbase: u32) {
    // SAFETY: the test lock is held by the caller, so no other trial reads
    // or writes these mirrors while they are set.
    unsafe {
        let table = core::ptr::addr_of_mut!(CALLEE_TABLE);
        (*table)[1] = one_arg_stub as *const () as usize as u32;
        (*table)[2] = four_arg_stub as *const () as usize as u32;
        let tls = core::ptr::addr_of_mut!(TLS_MIRROR);
        (*tls)[0] = thread_block;
        core::ptr::addr_of_mut!(lf_checker_rt::CHECKER_CTABLE).write(table as *const u32);
        core::ptr::addr_of_mut!(lf_checker_rt::CHECKER_TLS).write(tls as *const u32);
        core::ptr::addr_of_mut!(lf_checker_rt::CHECKER_XBASE).write(xbase);
    }
}

/// The relocated base that maps a file address `file_va` onto `target`.
#[must_use]
pub fn xbase_for(file_va: u32, target: u32) -> u32 {
    target.wrapping_sub(file_va.wrapping_sub(0x40_0000))
}

/// Starts a trial whose composer forwarders call numbered callee 2: the
/// one-argument name is recorded as in [`begin`], and the four-argument
/// callee under `four_name`.
pub fn begin_forward(answer: u32, one_name: &'static str, four_name: &'static str) {
    let mut s = script();
    s.log.clear();
    s.answer = answer;
    s.one_name = one_name;
    s.four_name = four_name;
}

extern "thiscall" fn four_arg_stub(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    let mut s = script();
    let name = s.four_name;
    s.log.push((name, vec![this, a0, a1, a2]));
    s.answer
}

/// The composer's sync slot (offset 0x78): records the composer.
pub extern "thiscall" fn sync_stub(this: u32) -> u32 {
    script().log.push(("sync", vec![this]));
    0
}

/// A composer's vtable: slot 30 (offset 0x78) is the sync stub.
#[must_use]
pub fn composer_vtable() -> Box<[u32; 40]> {
    let mut vt = Box::new([0u32; 40]);
    vt[0x74 / 4] = sync_stub as *const () as usize as u32;
    vt[0x78 / 4] = sync_stub as *const () as usize as u32;
    vt[0x7C / 4] = check_stub as *const () as usize as u32;
    vt[0x80 / 4] = handler_stub as *const () as usize as u32;
    vt
}

/// The composer's gate slot (offset 0x7C): records the composer and the
/// token it is given, and answers with the scripted answer.
pub extern "thiscall" fn check_stub(this: u32, token: u32) -> u32 {
    let mut s = script();
    s.log.push(("check", vec![this, token]));
    s.answer
}

/// The composer's node-handler slot (offset 0x80): records the composer and
/// the node.
pub extern "thiscall" fn handler_stub(this: u32, node: u32) -> u32 {
    script().log.push(("handle_node", vec![this, node]));
    0
}

extern "thiscall" fn one_arg_stub(this: u32) -> u32 {
    let mut s = script();
    let name = s.one_name;
    s.log.push((name, vec![this]));
    s.answer
}

/// Child poll stub: the answer is the word at offset 4 of the child object.
extern "thiscall" fn child_poll_stub(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    // SAFETY: `this` is a `ChildObj` the running case built; its answer
    // word is at offset 4.
    let answer = unsafe { ((this as usize + 4) as *const u32).read_unaligned() };
    script().log.push(("child.poll", vec![this, a0, a1, a2]));
    answer
}

extern "thiscall" fn owner_register_stub(this: u32, filter: u32) -> u32 {
    script().log.push(("owner.register", vec![this, filter]));
    0
}

extern "thiscall" fn manager_free_stub(this: u32, block: u32) -> u32 {
    script().log.push(("manager.free", vec![this, block]));
    0
}

/// A 32-bit object whose first word is a vtable: its slot `index` holds a
/// stub. The vtable is leaked to the trial's lifetime by its owner.
#[repr(C)]
pub struct VtObj {
    /// The vtable pointer.
    pub vtable: u32,
    /// Spare word, so the object has the layout of the originals' small
    /// objects.
    pub spare: u32,
}

/// A child filter object: vtable, then the answer its poll returns.
#[repr(C)]
pub struct ChildObj {
    /// The vtable pointer.
    pub vtable: u32,
    /// The answer the poll stub returns for this child.
    pub answer: u32,
}

/// A vtable with four slots; slot 3 (offset 0xC) is the one the rewrites call.
pub type Vt4 = Box<[u32; 4]>;

/// A vtable whose slot 3 is the child poll stub.
#[must_use]
pub fn child_vtable() -> Vt4 {
    Box::new([0, 0, 0, child_poll_stub as *const () as usize as u32])
}

/// A vtable whose slot 3 is the owner's registration stub.
#[must_use]
pub fn owner_vtable() -> Vt4 {
    Box::new([0, 0, 0, owner_register_stub as *const () as usize as u32])
}

/// A vtable whose slot 3 is the manager's free stub.
#[must_use]
pub fn manager_vtable() -> Vt4 {
    Box::new([0, 0, 0, manager_free_stub as *const () as usize as u32])
}

/// A thread block: word 2 (offset 8) names the owner or manager.
pub type ThreadBlock = Box<[u32; 4]>;

/// Reads a word from a test buffer at a byte offset.
#[must_use]
pub fn word(buf: &[u32], byte_off: u32) -> u32 {
    buf[(byte_off / 4) as usize]
}

/// Small deterministic generator (splitmix64): the same inputs on every run.
pub struct Rng(pub u64);

impl Rng {
    /// Next 64 bits of the sequence.
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Next 32 bits of the sequence.
    pub fn u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// A word that is zero, all ones, a sign boundary or random, so that
    /// each comparison sees the edges as well as the ordinary values.
    pub fn word(&mut self) -> u32 {
        match self.u32() % 8 {
            0 => 0,
            1 => 0xFFFF_FFFF,
            2 => 0x8000_0000,
            3 => 0x7FFF_FFFF,
            4 => 1,
            _ => self.u32(),
        }
    }

    /// A word whose low byte is zero and whose high bits are not: an answer
    /// that is nonzero as a word and zero as a verdict.
    pub fn answer(&mut self) -> u32 {
        (self.u32() | 0x0100) & !0xFF
    }
}

/// Numbered callee 2 answers with the stub of the given arity: 1 for the
/// drain callee, 4 for the forwarders' callee, 6 for the table dispatch.
pub fn select_callee2(arity: u8) {
    // SAFETY: the test lock is held by the caller, as for `install`.
    unsafe {
        let table = core::ptr::addr_of_mut!(CALLEE_TABLE);
        (*table)[2] = match arity {
            1 => one_arg_stub as *const () as usize as u32,
            6 => six_arg_stub as *const () as usize as u32,
            _ => four_arg_stub as *const () as usize as u32,
        };
    }
}

/// Six-argument callee (the table dispatch): records its arguments under
/// the four-argument name, which the trial sets for it.
extern "thiscall" fn six_arg_stub(this: u32, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    let mut s = script();
    let name = s.four_name;
    s.log.push((name, vec![this, a, b, c, d, e]));
    s.answer
}

/// The inner object's forward: records the inner object and answers with
/// the scripted answer.
extern "thiscall" fn inner_forward_stub(this: u32) -> u32 {
    let mut s = script();
    s.log.push(("forward_inner", vec![this]));
    s.answer
}

/// An inner object's vtable: slot 2 (offset 0x08) is the forward stub.
#[must_use]
pub fn inner_vtable() -> Box<[u32; 4]> {
    Box::new([0, 0, inner_forward_stub as *const () as usize as u32, 0])
}
