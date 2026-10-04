//! Differential harness: runs the verified 32-bit-form rewrites against the
//! lifted crate on the same inputs. i686 only (the rewrites are 32-bit
//! `extern "thiscall"` items).
//!
//! The shim below is the smallest stand-in for the checker runtime that
//! works: VA mapping for `relocated`/`global`, and scriptable callee stubs
//! for `callee_addr`. Real heap addresses back the 32-bit side, so pointer
//! round-trips through `u32` are exact on this target.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

/// Lock a shim mutex, tolerating poisoning from should-panic tests.
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

// ---- checker-runtime stand-in (same surface as lf_checker_rt) ----

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

macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}

macro_rules! callee_cdecl {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        let f: extern "cdecl" fn($( __ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute(callee_addr($id) as usize) };
        f($( $arg ),*)
    }};
}

/// File-VA to test-arena address map. Unregistered VAs read back as
/// themselves (stable cookies for vtable stamps and code pointers).
static VA_MAP: Mutex<Option<HashMap<u32, u32>>> = Mutex::new(None);

/// Register a file VA as backed by a test address.
pub fn map_va(file_va: u32, host_addr: u32) {
    lock(&VA_MAP).get_or_insert_with(HashMap::new).insert(file_va, host_addr);
}

/// Convert a file VA to the test mapping.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    if let Some(a) = lock(&VA_MAP).as_ref().and_then(|m| m.get(&file_va)) {
        return *a;
    }
    file_va
}

/// Pointer to a global at a file VA.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    relocated(file_va) as *mut T
}

/// Callee stub signatures used by this cluster.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sig {
    /// `extern "cdecl" fn(u32) -> u32`.
    Cdecl1,
    /// `extern "cdecl" fn(u32, u32) -> u32`.
    Cdecl2,
    /// `extern "cdecl" fn(u32, u32, u32, u32) -> u32`.
    Cdecl4,
    /// `extern "thiscall" fn(u32) -> u32`.
    This0,
    /// `extern "thiscall" fn(u32, u32) -> u32`.
    This1,
    /// `extern "thiscall" fn(u32, u32, u32) -> u32`.
    This2,
    /// `extern "thiscall" fn(u32, u32, u32, u32) -> u32`.
    This3,
    /// `extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32`.
    This4,
}

type Script = Box<dyn FnMut(&[u32]) -> u32 + Send>;
static SIGS: Mutex<[Option<Sig>; 4]> = Mutex::new([None, None, None, None]);
static SCRIPTS: Mutex<[Option<Script>; 4]> = Mutex::new([None, None, None, None]);

/// Install the script and signature for callee `id` (1..=3).
pub fn set_callee(id: u32, sig: Sig, f: Script) {
    lock(&SIGS)[id as usize] = Some(sig);
    lock(&SCRIPTS)[id as usize] = Some(f);
}

fn dispatch(id: u32, args: &[u32]) -> u32 {
    (lock(&SCRIPTS)[id as usize].as_mut().expect("callee fired without a script"))(args)
}

// Per-(id, signature) stubs. Only combos used by the cluster exist.
extern "cdecl" fn s1_c1(a: u32) -> u32 {
    dispatch(1, &[a])
}
extern "cdecl" fn s1_c4(a: u32, b: u32, c: u32, d: u32) -> u32 {
    dispatch(1, &[a, b, c, d])
}
extern "thiscall" fn s1_t0(t: u32) -> u32 {
    dispatch(1, &[t])
}
extern "thiscall" fn s1_t1(t: u32, a: u32) -> u32 {
    dispatch(1, &[t, a])
}
extern "thiscall" fn s1_t2(t: u32, a: u32, b: u32) -> u32 {
    dispatch(1, &[t, a, b])
}
extern "thiscall" fn s1_t4(t: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    dispatch(1, &[t, a, b, c, d])
}
extern "cdecl" fn s2_c1(a: u32) -> u32 {
    dispatch(2, &[a])
}
extern "thiscall" fn s2_t0(t: u32) -> u32 {
    dispatch(2, &[t])
}
extern "thiscall" fn s2_t1(t: u32, a: u32) -> u32 {
    dispatch(2, &[t, a])
}
extern "thiscall" fn s2_t3(t: u32, a: u32, b: u32, c: u32) -> u32 {
    dispatch(2, &[t, a, b, c])
}
extern "cdecl" fn s3_c2(a: u32, b: u32) -> u32 {
    dispatch(3, &[a, b])
}
extern "thiscall" fn s3_t0(t: u32) -> u32 {
    dispatch(3, &[t])
}
extern "thiscall" fn s3_t1(t: u32, a: u32) -> u32 {
    dispatch(3, &[t, a])
}

/// Raw stub address for callee `id`.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    let sig = lock(&SIGS)[id as usize].expect("callee_addr for undeclared id");
    match (id, sig) {
        (1, Sig::Cdecl1) => s1_c1 as usize as u32,
        (1, Sig::Cdecl4) => s1_c4 as usize as u32,
        (1, Sig::This0) => s1_t0 as usize as u32,
        (1, Sig::This1) => s1_t1 as usize as u32,
        (1, Sig::This2) => s1_t2 as usize as u32,
        (1, Sig::This4) => s1_t4 as usize as u32,
        (2, Sig::Cdecl1) => s2_c1 as usize as u32,
        (2, Sig::This0) => s2_t0 as usize as u32,
        (2, Sig::This1) => s2_t1 as usize as u32,
        (2, Sig::This3) => s2_t3 as usize as u32,
        (3, Sig::Cdecl2) => s3_c2 as usize as u32,
        (3, Sig::This0) => s3_t0 as usize as u32,
        (3, Sig::This1) => s3_t1 as usize as u32,
        _ => panic!("no stub for callee {id} with {sig:?}"),
    }
}

/// Serialise tests: the shim globals are per-process.
static TEST_LOCK: Mutex<()> = Mutex::new(());

/// Hold for a whole differential test; resets all shim state.
pub fn test_guard() -> std::sync::MutexGuard<'static, ()> {
    let guard = lock(&TEST_LOCK);
    *lock(&VA_MAP) = None;
    *lock(&SIGS) = [None, None, None, None];
    *lock(&SCRIPTS) = [None, None, None, None];
    guard
}

// ---- 32-bit arena: real addresses, stable for the whole test ----

/// Bump allocator over a fixed heap box. Addresses are real and stable,
/// so `u32` link round-trips are exact on i686.
pub struct Arena32 {
    /// Backing words (boxed: never moves).
    pub mem: Box<[u32]>,
    top: usize,
}

impl Arena32 {
    /// Fresh 256 KiB arena.
    #[must_use]
    pub fn new() -> Self {
        Self { mem: vec![0u32; 65536].into_boxed_slice(), top: 0 }
    }
    /// Allocate zeroed bytes, return the real address.
    pub fn alloc(&mut self, bytes: usize) -> u32 {
        let words = (bytes + 3) / 4;
        assert!(self.top + words <= self.mem.len(), "arena exhausted");
        let addr = self.mem.as_ptr() as u32 + (self.top * 4) as u32;
        for i in 0..words {
            self.mem[self.top + i] = 0;
        }
        self.top += words;
        addr
    }
    /// Read a word.
    #[must_use]
    pub fn w(&self, addr: u32) -> u32 {
        unsafe { *(addr as *const u32) }
    }
    /// Write a word.
    pub fn set(&mut self, addr: u32, v: u32) {
        unsafe { *(addr as *mut u32) = v; }
    }
    /// Read a byte.
    #[must_use]
    pub fn b(&self, addr: u32) -> u8 {
        unsafe { *(addr as *const u8) }
    }
    /// Write a byte.
    pub fn setb(&mut self, addr: u32, v: u8) {
        unsafe { *(addr as *mut u8) = v; }
    }
    /// Read a half word.
    #[must_use]
    pub fn h(&self, addr: u32) -> u16 {
        unsafe { *(addr as *const u16) }
    }
}

impl Default for Arena32 {
    fn default() -> Self {
        Self::new()
    }
}

/// Deterministic xorshift for input generation (no dependencies).
pub struct Rng(pub u64);
impl Rng {
    /// Next word.
    pub fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    /// Value below `n`.
    pub fn below(&mut self, n: u32) -> u32 {
        (self.next() % n as u64) as u32
    }
    /// Any u32.
    pub fn any(&mut self) -> u32 {
        self.next() as u32
    }
}

// ---- the 32-bit-form rewrites under test (copies, read-only) ----

include!("rw/fn_00a7c6d0.rs");
include!("rw/fn_00a7c700.rs");
include!("rw/fn_00a7c720.rs");
include!("rw/fn_00a7c8d0.rs");
include!("rw/fn_00a7c900.rs");
include!("rw/fn_00a7c940.rs");
include!("rw/fn_00a7c9c0.rs");
include!("rw/fn_00a7ca00.rs");
include!("rw/fn_00a7cac0.rs");
include!("rw/fn_00a7cd00.rs");
include!("rw/fn_00a7ce10.rs");
include!("rw/fn_00d69590.rs");
include!("rw/fn_00d695d0.rs");
include!("rw/fn_00d69600.rs");
include!("rw/fn_00d69650.rs");
include!("rw/fn_00d69670.rs");
include!("rw/fn_00d696a0.rs");
include!("rw/fn_00d696d0.rs");
include!("rw/fn_00d69850.rs");
include!("rw/fn_00d698c0.rs");
include!("rw/fn_00d698d0.rs");
include!("rw/fn_00d698e0.rs");
include!("rw/fn_00d698f0.rs");
include!("rw/fn_00d69910.rs");
include!("rw/fn_00d69930.rs");
include!("rw/fn_00d69950.rs");
include!("rw/fn_00d69990.rs");
include!("rw/fn_00d699b0.rs");
include!("rw/fn_009dcf80.rs");
include!("rw/fn_009dd000.rs");
include!("rw/fn_009dd080.rs");
include!("rw/fn_009dd100.rs");
include!("rw/fn_009dd180.rs");
include!("rw/fn_009dd210.rs");
include!("rw/fn_009dd280.rs");
include!("rw/fn_009ddf00.rs");
include!("rw/fn_009de2c0.rs");
