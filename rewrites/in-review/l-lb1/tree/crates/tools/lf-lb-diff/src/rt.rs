//! The 32-bit differential runtime: call recorder, scripted answers, stubs.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`), plus the
//! out-block-filling fetch stubs this family needs. Test-support code: the
//! lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw stub tables and pointer writes through scripted
// addresses are inherent here. Every access happens under the session lock,
// never while a rewrite runs except through the stub's own arguments.
#![allow(unsafe_code)]

use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::PoisonError;

/// One callee call: slot number and argument words in order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    /// Callee slot number.
    pub slot: u32,
    /// Argument words.
    pub args: Vec<u32>,
}

static LOG: Mutex<Vec<Call>> = Mutex::new(Vec::new());
static SESSION: Mutex<()> = Mutex::new(());

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Exclusive use of the recorder and the scripted answers for one case.
pub struct Session {
    _guard: MutexGuard<'static, ()>,
}

impl Drop for Session {
    fn drop(&mut self) {
        lock(&LOG).clear();
    }
}

/// Starts a session with an empty log.
#[must_use]
pub fn session() -> Session {
    let guard = lock(&SESSION);
    lock(&LOG).clear();
    Session { _guard: guard }
}

/// Logs one call.
pub fn record(slot: u32, args: &[u32]) {
    lock(&LOG).push(Call {
        slot,
        args: args.to_vec(),
    });
}

/// Runs `f` with an empty log and returns its result with the calls made.
pub fn capture<R>(f: impl FnOnce() -> R) -> (R, Vec<Call>) {
    lock(&LOG).clear();
    let result = f();
    let calls = core::mem::take(&mut *lock(&LOG));
    (result, calls)
}

/// The scripted fetch answer: success flag, count word, table addresses.
#[derive(Clone, Copy, Debug)]
pub struct FetchScript {
    /// Whether the fetch reports success (low byte 1 vs 0).
    pub ok: bool,
    /// Count word written to the block.
    pub count: u32,
    /// Primary table address written to the block.
    pub table_a: u32,
    /// Second table address written to the block.
    pub table_b: u32,
}

/// Which block words the fetch stub fills (per slot; `None` fills nothing).
#[derive(Clone, Copy, Debug)]
pub struct FetchLayout {
    /// Word index of the count, if the slot reads one.
    pub count_idx: Option<u32>,
    /// Word index of the primary table pointer.
    pub table_a_idx: u32,
    /// Word index of the second table pointer, if the slot reads one.
    pub table_b_idx: Option<u32>,
}

static FETCH: Mutex<FetchScript> = Mutex::new(FetchScript {
    ok: false,
    count: 0,
    table_a: 0,
    table_b: 0,
});
static LAYOUT: Mutex<FetchLayout> = Mutex::new(FetchLayout {
    count_idx: None,
    table_a_idx: 0,
    table_b_idx: None,
});

type ClassifyFn = Box<dyn FnMut(u32) -> u32 + Send>;
static CLASSIFY: Mutex<Option<ClassifyFn>> = Mutex::new(None);

/// Scripts the next fetch answers until changed.
pub fn set_fetch(script: FetchScript) {
    *lock(&FETCH) = script;
}

/// Sets which block words the fetch stub fills.
pub fn set_layout(layout: FetchLayout) {
    *lock(&LAYOUT) = layout;
}

/// Scripts the classifier as a pure function of the entry value.
pub fn set_classify(f: impl FnMut(u32) -> u32 + Send + 'static) {
    *lock(&CLASSIFY) = Some(Box::new(f));
}

/// The fetch stub's work: log, fill the out-block, answer ok/zero.
///
/// `out` is the rewrite's own scratch block address (real on the 32-bit
/// target). On fetch failure the block is left untouched: the rewrites
/// return without reading it.
#[cfg(target_arch = "x86")]
pub fn fetch_hook(slot: u32, board: u32, out: u32) -> u32 {
    record(slot, &[board, out]);
    let script = *lock(&FETCH);
    if script.ok {
        let layout = *lock(&LAYOUT);
        if let Some(idx) = layout.count_idx {
            unsafe { (out.wrapping_add(idx * 4) as *mut u32).write(script.count) };
        }
        unsafe { (out.wrapping_add(layout.table_a_idx * 4) as *mut u32).write(script.table_a) };
        if let Some(idx) = layout.table_b_idx {
            unsafe { (out.wrapping_add(idx * 4) as *mut u32).write(script.table_b) };
        }
        1
    } else {
        0
    }
}

/// The classify stub's work: log and answer from the script.
pub fn class_hook(slot: u32, args: &[u32]) -> u32 {
    record(slot, args);
    lock(&CLASSIFY)
        .as_mut()
        .map_or(0, |f| f(args.first().copied().unwrap_or(0)))
}

/// Marker slot for the key-stub calls: a vtable dispatch through the
/// fabricated object, not a numbered callee slot.
pub const VTABLE_SLOT: u32 = 0xFFFF_FFFE;

/// The key stub's scripted answer (probe-slot cases only).
#[cfg(target_arch = "x86")]
static KEY: Mutex<u32> = Mutex::new(0);

/// Scripts the key stub's answer.
#[cfg(target_arch = "x86")]
pub fn set_key(key: u32) {
    *lock(&KEY) = key;
}

/// Sets the relocated base the `relocated` answers derive from.
#[cfg(target_arch = "x86")]
pub fn set_xbase(xbase: u32) {
    unsafe {
        core::ptr::addr_of_mut!(CHECKER_XBASE).write(xbase);
    }
}

/// The key stub planted in fabricated vtables: logs the dispatch and
/// answers from the script.
#[cfg(target_arch = "x86")]
pub extern "thiscall" fn key_stub(this: u32) -> u32 {
    record(VTABLE_SLOT, &[this]);
    *lock(&KEY)
}

/// Address of [`key_stub`] for fabricated vtables.
#[cfg(target_arch = "x86")]
#[must_use]
pub fn key_stub_addr() -> u32 {
    key_stub as usize as u32
}

/// Stub table (256 addresses) and its pointer, mirroring `lf-checker-rt`.
#[cfg(target_arch = "x86")]
static mut TABLE: [u32; 256] = [0; 256];

/// Pointer to the stub table, mirroring `lf-checker-rt`.
#[cfg(target_arch = "x86")]
pub static mut CHECKER_CTABLE: *const u32 = core::ptr::null();

/// Raw stub address for callee `id` (0 when unset: calling it faults).
#[cfg(target_arch = "x86")]
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    unsafe {
        let table = core::ptr::addr_of!(CHECKER_CTABLE).read();
        if table.is_null() {
            return 0;
        }
        table.add(id as usize).read()
    }
}

/// Relocated base, mirroring `lf-checker-rt` (unused by these slots; the
/// table slots take no relocated addresses, but probe/constructor lifts
/// will need it).
#[cfg(target_arch = "x86")]
pub static mut CHECKER_XBASE: u32 = 0;

/// File VA to relocated address, mirroring `lf-checker-rt`.
#[cfg(target_arch = "x86")]
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    let xbase = unsafe { core::ptr::addr_of!(CHECKER_XBASE).read() };
    file_va.wrapping_sub(0x400000).wrapping_add(xbase)
}

/// Non-32-bit stand-ins so host builds resolve the same names.
#[cfg(not(target_arch = "x86"))]
#[must_use]
pub fn callee_addr(_id: u32) -> u32 {
    unimplemented!("32-bit differential cases do not run on this host")
}

/// Non-32-bit stand-ins so host builds resolve the same names.
#[cfg(not(target_arch = "x86"))]
#[must_use]
pub fn relocated(_file_va: u32) -> u32 {
    unimplemented!("32-bit differential cases do not run on this host")
}

/// Installs the fetch stub at `fetch_slot` and the classify stub at
/// `class_slot` (`u32::MAX` installs none), clearing every other entry.
#[cfg(target_arch = "x86")]
pub fn install(fetch_slot: u32, class_slot: u32, class_fastcall: bool) {
    assert!(
        fetch_slot <= 8,
        "fetch slot {fetch_slot} has no stub (slots 0..=8 covered)"
    );
    assert!(
        class_slot == u32::MAX || class_slot <= 8,
        "classify slot {class_slot} has no stub (slots 0..=8 covered)"
    );
    macro_rules! addr {
        ($f:expr) => {
            $f as usize as u32
        };
    }
    let fetch = [
        addr!(x86::fetch_s0),
        addr!(x86::fetch_s1),
        addr!(x86::fetch_s2),
        addr!(x86::fetch_s3),
        addr!(x86::fetch_s4),
        addr!(x86::fetch_s5),
        addr!(x86::fetch_s6),
        addr!(x86::fetch_s7),
        addr!(x86::fetch_s8),
    ];
    let classt = [
        addr!(x86::classt_s0),
        addr!(x86::classt_s1),
        addr!(x86::classt_s2),
        addr!(x86::classt_s3),
        addr!(x86::classt_s4),
        addr!(x86::classt_s5),
        addr!(x86::classt_s6),
        addr!(x86::classt_s7),
        addr!(x86::classt_s8),
    ];
    let classf = [
        addr!(x86::classf_s0),
        addr!(x86::classf_s1),
        addr!(x86::classf_s2),
        addr!(x86::classf_s3),
        addr!(x86::classf_s4),
        addr!(x86::classf_s5),
        addr!(x86::classf_s6),
        addr!(x86::classf_s7),
        addr!(x86::classf_s8),
    ];
    unsafe {
        let table = core::ptr::addr_of_mut!(TABLE);
        (*table) = [0; 256];
        (*table)[fetch_slot as usize] = fetch[fetch_slot as usize];
        if class_slot != u32::MAX {
            (*table)[class_slot as usize] = if class_fastcall {
                classf[class_slot as usize]
            } else {
                classt[class_slot as usize]
            };
        }
        core::ptr::addr_of_mut!(CHECKER_CTABLE).write(core::ptr::addr_of!(TABLE).cast::<u32>());
    }
}

/// The stubs each slot may need: a block-filling fastcall fetch, a
/// thiscall classify, and a fastcall-declared classify (some lanes declare
/// the classifier fastcall with a dummy second word).
#[cfg(target_arch = "x86")]
mod x86 {
    use crate::rt::class_hook;
    use crate::rt::fetch_hook;

    macro_rules! fetch_stubs {
        ($($name:ident, $slot:literal;)*) => {$(
            pub(super) extern "fastcall" fn $name(board: u32, out: u32) -> u32 {
                fetch_hook($slot, board, out)
            }
        )*};
    }

    macro_rules! classt_stubs {
        ($($name:ident, $slot:literal;)*) => {$(
            pub(super) extern "thiscall" fn $name(value: u32) -> u32 {
                class_hook($slot, &[value])
            }
        )*};
    }

    macro_rules! classf_stubs {
        ($($name:ident, $slot:literal;)*) => {$(
            pub(super) extern "fastcall" fn $name(value: u32, _dummy: u32) -> u32 {
                class_hook($slot, &[value, _dummy])
            }
        )*};
    }

    fetch_stubs!(
        fetch_s0, 0; fetch_s1, 1; fetch_s2, 2; fetch_s3, 3; fetch_s4, 4;
        fetch_s5, 5; fetch_s6, 6; fetch_s7, 7; fetch_s8, 8;
    );
    classt_stubs!(
        classt_s0, 0; classt_s1, 1; classt_s2, 2; classt_s3, 3; classt_s4, 4;
        classt_s5, 5; classt_s6, 6; classt_s7, 7; classt_s8, 8;
    );
    classf_stubs!(
        classf_s0, 0; classf_s1, 1; classf_s2, 2; classf_s3, 3; classf_s4, 4;
        classf_s5, 5; classf_s6, 6; classf_s7, 7; classf_s8, 8;
    );
}

/// Declare a rewrite export with the original's calling convention.
///
/// On the 32-bit target this is the checker's form (`extern "<conv>"`); on
/// other hosts the differential cases do not build at all.
#[macro_export]
macro_rules! export {
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        #[unsafe(no_mangle)]
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
    (stdcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        #[unsafe(no_mangle)]
        pub extern "stdcall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (fastcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        #[unsafe(no_mangle)]
        pub extern "fastcall" fn $name($($arg : $ty),*) -> $ret $body
    };
}

/// Call intercepted callee `id` with the fastcall convention.
#[macro_export]
macro_rules! callee_fastcall {
    ($id:expr, $ret:ty, $ecx_arg:expr, $edx_arg:expr $(, $arg:expr)* $(,)?) => {{
        let f: extern "fastcall" fn(u32, u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($ecx_arg, $edx_arg $(, $arg )*)
    }};
}

/// Call intercepted callee `id` with the thiscall convention.
#[macro_export]
macro_rules! callee_thiscall {
    ($id:expr, $ret:ty, $this_arg:expr $(, $arg:expr)* $(,)?) => {{
        let f: extern "thiscall" fn(u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($this_arg $(, $arg )*)
    }};
}

/// Every callee argument word is `u32` (mirrors `lf-checker-rt`).
#[doc(hidden)]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}
