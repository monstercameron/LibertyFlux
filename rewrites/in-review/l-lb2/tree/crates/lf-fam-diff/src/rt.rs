//! The 32-bit differential runtime: call recorder, scripted answers, stubs.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`), with recording stubs at the slots the target families use.
//! Test-support code: the lifted crates stay `#![forbid(unsafe_code)]`.

// Test-only runtime: raw stub tables and pointer writes through scripted
// addresses are inherent here. Every access happens under the session lock,
// never while a rewrite runs except through the stub's own arguments.
#![allow(unsafe_code)]

use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::PoisonError;

#[cfg(target_arch = "x86")]
include!("stubs_gen.rs");

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

type Answers = Box<dyn FnMut(u32, &[u32]) -> u32 + Send>;
static ANSWERS: Mutex<Option<Answers>> = Mutex::new(None);

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
        *lock(&ANSWERS) = None;
    }
}

/// Starts a session with an empty log and zero answers.
#[must_use]
pub fn session() -> Session {
    let guard = lock(&SESSION);
    lock(&LOG).clear();
    *lock(&ANSWERS) = None;
    Session { _guard: guard }
}

/// Scripts every callee's answer as a function of slot and arguments.
pub fn set_answers(f: impl FnMut(u32, &[u32]) -> u32 + Send + 'static) {
    *lock(&ANSWERS) = Some(Box::new(f));
}

/// Logs one call, answering from the script (zero when unscripted).
pub fn record(slot: u32, args: &[u32]) -> u32 {
    lock(&LOG).push(Call {
        slot,
        args: args.to_vec(),
    });
    lock(&ANSWERS)
        .as_mut()
        .map_or(0, |f| f(slot, args))
}

/// Runs `f` with an empty log and returns its result with the calls made.
pub fn capture<R>(f: impl FnOnce() -> R) -> (R, Vec<Call>) {
    lock(&LOG).clear();
    let result = f();
    let calls = core::mem::take(&mut *lock(&LOG));
    (result, calls)
}

/// Convention and arity of a stubbed callee slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sig {
    /// `cdecl` with `n` argument words.
    Cdecl(u8),
    /// `stdcall` with one argument word.
    Stdcall1,
    /// `thiscall` with the object and one argument word.
    ThisPlus1,
}

/// Stub table (256 addresses) and its pointer, mirroring `lf-checker-rt`.
#[cfg(target_arch = "x86")]
static mut TABLE: [u32; 256] = [0; 256];

/// Pointer to the stub table, mirroring `lf-checker-rt`.
#[cfg(target_arch = "x86")]
pub static mut CHECKER_CTABLE: *const u32 = core::ptr::null();

/// Relocated base, mirroring `lf-checker-rt`.
#[cfg(target_arch = "x86")]
pub static mut CHECKER_XBASE: u32 = 0;

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

/// File VA to relocated address, mirroring `lf-checker-rt`.
#[cfg(target_arch = "x86")]
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    let xbase = unsafe { core::ptr::addr_of!(CHECKER_XBASE).read() };
    file_va.wrapping_sub(0x400000).wrapping_add(xbase)
}

/// Pointer to a global at a file VA, mirroring `lf-checker-rt`.
#[cfg(target_arch = "x86")]
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    relocated(file_va) as *mut T
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

/// Non-32-bit stand-ins so host builds resolve the same names.
#[cfg(not(target_arch = "x86"))]
#[must_use]
pub fn global<T>(_file_va: u32) -> *mut T {
    unimplemented!("32-bit differential cases do not run on this host")
}

/// Sets the relocated base answers derive from.
#[cfg(target_arch = "x86")]
pub fn set_xbase(xbase: u32) {
    unsafe {
        core::ptr::addr_of_mut!(CHECKER_XBASE).write(xbase);
    }
}

/// The base that maps file address `base` onto buffer address `buf_addr`:
/// `relocated(va) = va - 0x400000 + xbase` must equal the buffer's byte.
#[cfg(target_arch = "x86")]
#[must_use]
pub fn xbase_for(buf_addr: u32, base: u32) -> u32 {
    buf_addr.wrapping_sub(base.wrapping_sub(0x400000))
}

/// Address of the recording stub for `slot` with signature `sig`.
///
/// # Panics
///
/// When the combination was never generated (a case bug: every rewrite's
/// call shape has a stub).
#[cfg(target_arch = "x86")]
#[must_use]
pub fn stub_addr(slot: u32, sig: Sig) -> u32 {
    match sig {
        Sig::Cdecl(n) => cdecl_addr(slot, n),
        Sig::Stdcall1 => {
            assert_eq!(slot, 1, "stdcall stub only on slot 1");
            u32::try_from(s1 as usize).expect("32-bit code address")
        }
        Sig::ThisPlus1 => {
            assert_eq!(slot, 1, "thiscall stub only on slot 1");
            u32::try_from(t1 as usize).expect("32-bit code address")
        }
    }
}

/// Installs one recording stub, clearing every other entry.
#[cfg(target_arch = "x86")]
pub fn install(slot: u32, sig: Sig) {
    let addr = stub_addr(slot, sig);
    unsafe {
        let table = core::ptr::addr_of_mut!(TABLE);
        (*table) = [0; 256];
        (*table)[slot as usize] = addr;
        core::ptr::addr_of_mut!(CHECKER_CTABLE).write(core::ptr::addr_of!(TABLE).cast::<u32>());
    }
}

/// Declare a rewrite export with the original's calling convention.
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

/// Call intercepted callee `id` with the cdecl convention.
#[macro_export]
macro_rules! callee_cdecl {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        let f: extern "cdecl" fn($( $crate::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($( $arg ),*)
    }};
}

/// Call intercepted callee `id` with the stdcall convention.
#[macro_export]
macro_rules! callee_stdcall {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        let f: extern "stdcall" fn($( $crate::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($( $arg ),*)
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

/// Call intercepted callee `id` with the fastcall convention.
#[macro_export]
macro_rules! callee_fastcall {
    ($id:expr, $ret:ty, $ecx_arg:expr, $edx_arg:expr $(, $arg:expr)* $(,)?) => {{
        let f: extern "fastcall" fn(u32, u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($ecx_arg, $edx_arg $(, $arg )*)
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
