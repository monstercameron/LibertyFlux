//! The 32-bit differential runtime for the mirror-reflection phase.
//!
//! It mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`, `global`).
//! Numbered callees dispatch through stubs of the matching convention and
//! arity; each stub records the call under the callee's name, with its
//! non-pointer arguments and the words its pointer arguments point at, then
//! applies any write the callee's specification scripts and answers from a
//! queue by name. The virtual slot the phase calls lands on one stub that
//! records the same way. Every state is thread-local except the globals
//! arena, which the harness serialises.
//!
//! Test-support code: the lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: rewrites take raw addresses, and the stubs read and
// write the memory the harness built for each case. Every access stays
// inside that memory.
#![allow(unsafe_code)]

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};

/// One recorded call: the callee's name, its non-pointer arguments in
/// order, and the words its pointer arguments pointed at when it was called.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    /// The callee's name in the method's specification.
    pub name: &'static str,
    /// Non-pointer arguments, in the order the original pushes them.
    pub args: Vec<u32>,
    /// The words the pointer arguments pointed at, before any write.
    pub data: Vec<u32>,
}

/// The calling convention of a numbered callee.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Conv {
    /// Thiscall: the first argument is in the register.
    This,
    /// Cdecl: all arguments on the stack.
    Cdecl,
}

/// How one numbered callee of a rewrite is called and recorded.
#[derive(Clone, Debug)]
pub struct CalleeSpec {
    /// The callee number the rewrite file uses.
    pub id: u32,
    /// The name the call is recorded and answered under.
    pub name: &'static str,
    /// Calling convention.
    pub conv: Conv,
    /// Number of arguments, including the register argument of thiscall.
    pub nargs: usize,
    /// Pointer arguments as (argument index, words the pointer covers).
    pub ptrs: Vec<(usize, usize)>,
    /// Writes the callee performs through a pointer argument, as (argument
    /// index, first word, words), applied after the call is recorded.
    pub writes: Vec<(usize, usize, Vec<u32>)>,
}

thread_local! {
    static CURRENT_ID: Cell<u32> = const { Cell::new(0) };
    static SPECS: RefCell<Vec<CalleeSpec>> = const { RefCell::new(Vec::new()) };
    static ANSWERS: RefCell<HashMap<&'static str, VecDeque<u32>>> = RefCell::new(HashMap::new());
    static LOG: RefCell<Vec<Call>> = const { RefCell::new(Vec::new()) };
}

/// Installs the callee specifications and their answer queues for one run,
/// and clears the call log.
pub fn install(specs: Vec<CalleeSpec>, answers: HashMap<&'static str, VecDeque<u32>>) {
    SPECS.with(|s| *s.borrow_mut() = specs);
    ANSWERS.with(|a| *a.borrow_mut() = answers);
    LOG.with(|l| l.borrow_mut().clear());
}

/// Takes the recorded calls, clearing the log.
#[must_use]
pub fn take_log() -> Vec<Call> {
    LOG.with(|l| std::mem::take(&mut *l.borrow_mut()))
}

/// Records a call under `name` in the log.
pub fn record(name: &'static str, args: Vec<u32>, data: Vec<u32>) {
    LOG.with(|l| l.borrow_mut().push(Call { name, args, data }));
}

/// Pops the next answer queued under `name`; zero when the queue is empty.
#[must_use]
pub fn answer(name: &'static str) -> u32 {
    ANSWERS.with(|a| {
        a.borrow_mut()
            .get_mut(name)
            .and_then(VecDeque::pop_front)
            .unwrap_or(0)
    })
}

/// The stub address for numbered callee `id`, by its installed convention
/// and arity. It also marks which callee the stub is answering.
///
/// # Panics
///
/// When no specification covers `id`, or the arity has no stub: a case bug.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    let (conv, nargs) = SPECS.with(|s| {
        let specs = s.borrow();
        let spec = specs
            .iter()
            .find(|c| c.id == id)
            .unwrap_or_else(|| panic!("no callee specification for id {id}"));
        (spec.conv, spec.nargs)
    });
    CURRENT_ID.with(|c| c.set(id));
    let addr = match (conv, nargs) {
        (Conv::This, 0) => tc0 as *const () as usize,
        (Conv::This, 1) => tc1 as *const () as usize,
        (Conv::This, 2) => tc2 as *const () as usize,
        (Conv::This, 3) => tc3 as *const () as usize,
        (Conv::This, 7) => tc7 as *const () as usize,
        (Conv::Cdecl, 0) => cc0 as *const () as usize,
        (Conv::Cdecl, 1) => cc1 as *const () as usize,
        (Conv::Cdecl, 2) => cc2 as *const () as usize,
        (Conv::Cdecl, 3) => cc3 as *const () as usize,
        (Conv::Cdecl, 5) => cc5 as *const () as usize,
        (Conv::Cdecl, 6) => cc6 as *const () as usize,
        _ => panic!("no stub for {conv:?} with {nargs} arguments"),
    };
    addr as u32
}

/// The shared body of every numbered stub: records the call, applies the
/// specified writes, and answers.
fn numbered(args: &[u32]) -> u32 {
    let id = CURRENT_ID.with(Cell::get);
    let spec = SPECS
        .with(|s| s.borrow().iter().find(|c| c.id == id).cloned())
        .unwrap_or_else(|| panic!("stub called for unknown id {id}"));
    assert_eq!(args.len(), spec.nargs, "stub arity differs from the specification");
    let plain: Vec<u32> = args
        .iter()
        .enumerate()
        .filter(|(i, _)| !spec.ptrs.iter().any(|(p, _)| p == i))
        .map(|(_, v)| *v)
        .collect();
    let mut data = Vec::new();
    for (index, words) in &spec.ptrs {
        let base = args[*index] as usize as *const u32;
        assert!(!base.is_null(), "callee {} got a null pointer argument", spec.name);
        for k in 0..*words {
            // SAFETY: the harness allocated the memory this pointer covers,
            // and the specification names no more words than that memory has.
            data.push(unsafe { base.add(k).read_unaligned() });
        }
    }
    record(spec.name, plain, data);
    for (index, first, words) in &spec.writes {
        let base = args[*index] as usize as *mut u32;
        for (k, word) in words.iter().enumerate() {
            // SAFETY: as for the reads above; the scripted writes cover words
            // the specification's pointer argument covers.
            unsafe { base.add(first + k).write_unaligned(*word) };
        }
    }
    answer(spec.name)
}

macro_rules! thiscall_stub {
    ($name:ident; $($arg:ident),*) => {
        extern "thiscall" fn $name($($arg: u32),*) -> u32 {
            numbered(&[$($arg),*])
        }
    };
}

macro_rules! cdecl_stub {
    ($name:ident; $($arg:ident),*) => {
        extern "cdecl" fn $name($($arg: u32),*) -> u32 {
            numbered(&[$($arg),*])
        }
    };
}

thiscall_stub!(tc0;);
thiscall_stub!(tc1; a);
thiscall_stub!(tc2; a, b);
thiscall_stub!(tc3; a, b, c);
thiscall_stub!(tc7; a, b, c, d, e, f, g);
cdecl_stub!(cc0;);
cdecl_stub!(cc1; a);
cdecl_stub!(cc2; a, b);
cdecl_stub!(cc3; a, b, c);
cdecl_stub!(cc5; a, b, c, d, e);
cdecl_stub!(cc6; a, b, c, d, e, f);

/// The virtual slot at byte offset 0x30 of the phase's collaborators: records
/// the call as `slot30` with its object and argument, and answers.
extern "thiscall" fn slot30_stub(this: u32, arg: u32) -> u32 {
    record("slot30", vec![this, arg], Vec::new());
    answer("slot30")
}

/// The address the virtual slot stub is reached at.
#[must_use]
pub fn slot30_addr() -> u32 {
    slot30_stub as *const () as usize as u32
}

/// Words in the globals arena (the test's stand-in for global data).
pub const ARENA_WORDS: usize = 256;

/// The globals arena. Every global the rewrites read or write lives here at
/// the offset [`arena_offset`] gives.
static mut ARENA: [u32; ARENA_WORDS] = [0; ARENA_WORDS];

/// The arena's base address.
#[must_use]
pub fn arena_base() -> u32 {
    core::ptr::addr_of_mut!(ARENA) as *mut u32 as usize as u32
}

/// Pointer to the arena's words, for the harness to read and write.
#[must_use]
pub fn arena_ptr() -> *mut u32 {
    core::ptr::addr_of_mut!(ARENA) as *mut u32
}

/// The byte offset in the arena of a file address the mirror-reflection
/// rewrites reach through `global` or `relocated`.
///
/// # Panics
///
/// For any other address: a case bug, never a guess.
#[must_use]
pub fn arena_offset(file_va: u32) -> u32 {
    match file_va {
        0x0166_DA20 => 0x000, // render gate (byte)
        0x0166_DA21 => 0x001, // skip gate (byte)
        0x0166_DA22 => 0x002, // selector flag a (byte)
        0x0179_7695 => 0x003, // selector flag b (byte)
        0x0105_6B50 => 0x004, // float block flag (byte)
        0x0179_7694 => 0x005, // scale flag (byte)
        0x00FE_88E8 => 0x034, // one, as float bits
        0x00FE_8D94 => 0x030, // scale, as float bits
        0x0166_DA68 => 0x010, // swap enabled
        0x0166_DA6C => 0x014, // swap index
        0x0166_DA70 => 0x020, // published selector
        0x0166_DA74 => 0x024, // published count
        0x0103_27A0 => 0x028, // stamp counter
        0x0161_4C90 => 0x038, // session object (address only)
        0x0166_DA3C => 0x100, // parameter table, count word of an entry
        0x0166_DA40 => 0x104, // parameter table, selector word of an entry
        0x00E7_E048 => 0x040, // first stamp tag (address only)
        0x00E7_E080 => 0x044, // final stamp tag (address only)
        0x009C_B6A0 => 0x048, // first trailer (address only)
        0x00C1_06F0 => 0x04C, // second trailer (address only)
        _ => panic!("unexpected global address {file_va:#x}"),
    }
}

/// Address of a file address's cell in the arena, mirroring the checker's
/// `relocated`.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    arena_base().wrapping_add(arena_offset(file_va))
}

/// Pointer to the global at a file address, mirroring the checker's `global`.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    relocated(file_va) as usize as *mut T
}

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`.
#[macro_export]
macro_rules! export {
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (cdecl).
        #[unsafe(no_mangle)]
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
}

/// Call intercepted callee `id` with the cdecl convention.
#[macro_export]
macro_rules! callee_cdecl {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        #[allow(unused_unsafe)]
        let f: extern "cdecl" fn($( $crate::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($( $arg ),*)
    }};
}

/// Call intercepted callee `id` with the thiscall convention.
#[macro_export]
macro_rules! callee_thiscall {
    ($id:expr, $ret:ty, $this_arg:expr $(, $arg:expr)* $(,)?) => {{
        #[allow(unused_unsafe)]
        let f: extern "thiscall" fn(u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($this_arg $(, $arg )*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}
