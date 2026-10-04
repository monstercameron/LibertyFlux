//! The runtime the verified rewrites call, and the call recorder.
//!
//! Both targets share the recorder: every callee call a rewrite makes, and
//! every trait call a lifted function's fake translates, lands in one log
//! as a [`Call`] (slot number, argument words), and both get their answer
//! from the same scripted function ([`set_answers`]).
//!
//! On the 32-bit target the rewrites call the real `lf-checker-rt`.
//! [`with_image`] sets its relocated base so `relocated(addr)` lands in a
//! [`VaImage`], and [`set_stubs`] fills its callee table with recording
//! stubs of the right convention and arity.
//!
//! On other hosts the rewrites call the stand-in in [`crate::rewrites`],
//! whose callee macros call [`record`] directly and whose `global()` is
//! [`global`] here: a pointer into the installed [`VaImage`]. The
//! stand-in's [`relocated`] returns the file address unchanged, which is
//! consistent for every rewrite that only passes or returns relocated
//! addresses as numbers; a rewrite that dereferences one itself needs the
//! 32-bit target.
//!
//! Every case that uses the log, the answers or an image holds a
//! [`session`] guard, so cases never interleave.

use std::sync::{Mutex, MutexGuard, PoisonError};

use crate::VaImage;

/// One callee call: the slot it went through and its argument words in
/// order (for a `thiscall`, the object first).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    /// Callee slot number.
    pub slot: u32,
    /// Argument words.
    pub args: Vec<u32>,
}

type Answers = Box<dyn FnMut(u32, &[u32]) -> u32 + Send>;

static LOG: Mutex<Vec<Call>> = Mutex::new(Vec::new());
static ANSWERS: Mutex<Option<Answers>> = Mutex::new(None);
static SESSION: Mutex<()> = Mutex::new(());

/// Image the stand-in runtime resolves globals into: base address,
/// exposed address of the first byte, length.
#[cfg(not(target_arch = "x86"))]
static IMAGE: Mutex<Option<(u32, usize, usize)>> = Mutex::new(None);

/// Locks a mutex, tolerating poison: a failed case must not take the
/// recorder down for the cases after it, and a stub must never panic.
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Exclusive use of the recorder, the answers and the runtime statics for
/// one case. Dropping it clears all three.
pub struct Session {
    _guard: MutexGuard<'static, ()>,
}

impl Drop for Session {
    fn drop(&mut self) {
        reset();
    }
}

/// Starts a session (waiting for any other to end) with an empty log, no
/// answers (every callee answers zero) and no image.
#[must_use]
pub fn session() -> Session {
    let guard = lock(&SESSION);
    reset();
    Session { _guard: guard }
}

fn reset() {
    lock(&LOG).clear();
    *lock(&ANSWERS) = None;
    #[cfg(not(target_arch = "x86"))]
    {
        *lock(&IMAGE) = None;
    }
}

/// Scripts every callee's answer as a function of slot and arguments. It
/// must not depend on how often it is called: the reference and the lift
/// each run once against it.
pub fn set_answers(answers: impl FnMut(u32, &[u32]) -> u32 + Send + 'static) {
    *lock(&ANSWERS) = Some(Box::new(answers));
}

/// Logs one call and returns its scripted answer (zero when none is set).
/// Fakes may ignore the answer (a callee whose result the lift drops).
#[allow(clippy::must_use_candidate)]
pub fn record(slot: u32, args: &[u32]) -> u32 {
    lock(&LOG).push(Call {
        slot,
        args: args.to_vec(),
    });
    lock(&ANSWERS)
        .as_mut()
        .map_or(0, |answer| answer(slot, args))
}

/// Runs `f` with an empty log and returns its result with the calls made.
pub fn capture<R>(f: impl FnOnce() -> R) -> (R, Vec<Call>) {
    lock(&LOG).clear();
    let result = f();
    let calls = core::mem::take(&mut *lock(&LOG));
    (result, calls)
}

/// The address a rewrite sees for file address `file_va`: unchanged on the
/// stand-in, relocated by the installed image on the 32-bit target.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    #[cfg(target_arch = "x86")]
    {
        lf_checker_rt::relocated(file_va)
    }
    #[cfg(not(target_arch = "x86"))]
    {
        file_va
    }
}

/// Stand-in for `lf_checker_rt::global`: a pointer to `file_va` inside the
/// installed image.
///
/// # Panics
///
/// When no image is installed or `file_va` is outside it (a case set up
/// too small an image; the panic names the address).
#[cfg(not(target_arch = "x86"))]
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    let (base, start, len) = lock(&IMAGE).expect("no test image installed");
    let offset = file_va.wrapping_sub(base) as usize;
    assert!(
        offset.saturating_add(core::mem::size_of::<T>()) <= len,
        "global {file_va:#010x} is outside the test image"
    );
    core::ptr::with_exposed_provenance_mut::<T>(start + offset)
}

/// Runs `f` with `image` installed as the original's address space.
///
/// # Panics
///
/// On the 32-bit target, never in practice: the image's address always
/// fits 32 bits there.
pub fn with_image<R>(image: &mut VaImage, f: impl FnOnce() -> R) -> R {
    let start = image.as_mut_ptr().expose_provenance();
    #[cfg(target_arch = "x86")]
    {
        // relocated(va) = va - 0x400000 + xbase must equal
        // start + (va - base), so xbase = start - (base - 0x400000).
        const FILE_IMAGE_BASE: u32 = 0x0040_0000;
        let start = u32::try_from(start).expect("32-bit address");
        let xbase = start.wrapping_sub(image.base().wrapping_sub(FILE_IMAGE_BASE));
        x86::set_xbase(xbase);
        let result = f();
        x86::set_xbase(0);
        result
    }
    #[cfg(not(target_arch = "x86"))]
    {
        *lock(&IMAGE) = Some((image.base(), start, image.len()));
        let result = f();
        *lock(&IMAGE) = None;
        result
    }
}

/// Convention and arity of a callee slot, as the rewrite calls it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sig {
    /// `cdecl`, no arguments.
    Cdecl0,
    /// `cdecl`, one argument.
    Cdecl1,
    /// `cdecl`, two arguments.
    Cdecl2,
    /// `cdecl`, three arguments.
    Cdecl3,
    /// `cdecl`, four arguments.
    Cdecl4,
    /// `cdecl`, five arguments.
    Cdecl5,
    /// `thiscall`, the object only.
    This1,
    /// `thiscall`, the object and two arguments.
    This3,
}

/// Points callee slots at recording stubs of the given signatures. A
/// no-op on the stand-in, whose callee macros record directly; on the
/// 32-bit target a slot left unset faults when called, which the test
/// binary reports as a crash naming no case, so set every slot a rewrite
/// uses.
///
/// # Panics
///
/// On the 32-bit target, for a slot outside 1 to 3.
pub fn set_stubs(slots: &[(u32, Sig)]) {
    #[cfg(target_arch = "x86")]
    {
        x86::set_stubs(slots);
    }
    #[cfg(not(target_arch = "x86"))]
    {
        let _ = slots;
    }
}

/// The 32-bit side: the checker runtime's statics and the stubs.
#[cfg(target_arch = "x86")]
#[allow(unsafe_code)]
mod x86 {
    // The checker runtime exposes its relocated base and callee table as
    // `static mut`s that the worker patches; a test patches them the same
    // way. Writes happen only under the session lock, before and after the
    // rewrite runs, never during.
    use super::{Sig, record};
    use std::sync::atomic::{AtomicU32, Ordering};

    /// The callee table the runtime reads (256 stub addresses).
    static TABLE: [AtomicU32; 256] = [const { AtomicU32::new(0) }; 256];

    pub(super) fn set_xbase(xbase: u32) {
        // SAFETY: plain store to the runtime's base word; no reference to
        // the static is formed and no rewrite is running (session lock).
        unsafe {
            lf_checker_rt::CHECKER_XBASE = xbase;
        }
    }

    macro_rules! cdecl_stub {
        ($name:ident, $slot:literal $(, $a:ident)*) => {
            extern "cdecl" fn $name($($a: u32),*) -> u32 {
                record($slot, &[$($a),*])
            }
        };
    }

    macro_rules! thiscall_stub {
        ($name:ident, $slot:literal $(, $a:ident)*) => {
            extern "thiscall" fn $name($($a: u32),*) -> u32 {
                record($slot, &[$($a),*])
            }
        };
    }

    cdecl_stub!(c0_1, 1);
    cdecl_stub!(c0_2, 2);
    cdecl_stub!(c0_3, 3);
    cdecl_stub!(c1_1, 1, a);
    cdecl_stub!(c1_2, 2, a);
    cdecl_stub!(c1_3, 3, a);
    cdecl_stub!(c2_1, 1, a, b);
    cdecl_stub!(c2_2, 2, a, b);
    cdecl_stub!(c2_3, 3, a, b);
    cdecl_stub!(c3_1, 1, a, b, c);
    cdecl_stub!(c3_2, 2, a, b, c);
    cdecl_stub!(c3_3, 3, a, b, c);
    cdecl_stub!(c4_1, 1, a, b, c, d);
    cdecl_stub!(c4_2, 2, a, b, c, d);
    cdecl_stub!(c4_3, 3, a, b, c, d);
    cdecl_stub!(c5_1, 1, a, b, c, d, e);
    cdecl_stub!(c5_2, 2, a, b, c, d, e);
    cdecl_stub!(c5_3, 3, a, b, c, d, e);
    thiscall_stub!(t1_1, 1, this);
    thiscall_stub!(t1_2, 2, this);
    thiscall_stub!(t1_3, 3, this);
    thiscall_stub!(t3_1, 1, this, a, b);
    thiscall_stub!(t3_2, 2, this, a, b);
    thiscall_stub!(t3_3, 3, this, a, b);

    /// Address of the stub for `slot` with signature `sig`.
    fn stub(slot: u32, sig: Sig) -> u32 {
        type C0 = extern "cdecl" fn() -> u32;
        type C1 = extern "cdecl" fn(u32) -> u32;
        type C2 = extern "cdecl" fn(u32, u32) -> u32;
        type C3 = extern "cdecl" fn(u32, u32, u32) -> u32;
        type C4 = extern "cdecl" fn(u32, u32, u32, u32) -> u32;
        type C5 = extern "cdecl" fn(u32, u32, u32, u32, u32) -> u32;
        type T1 = extern "thiscall" fn(u32) -> u32;
        type T3 = extern "thiscall" fn(u32, u32, u32) -> u32;
        let addr = match (sig, slot) {
            (Sig::Cdecl0, 1) => c0_1 as C0 as usize,
            (Sig::Cdecl0, 2) => c0_2 as C0 as usize,
            (Sig::Cdecl0, 3) => c0_3 as C0 as usize,
            (Sig::Cdecl1, 1) => c1_1 as C1 as usize,
            (Sig::Cdecl1, 2) => c1_2 as C1 as usize,
            (Sig::Cdecl1, 3) => c1_3 as C1 as usize,
            (Sig::Cdecl2, 1) => c2_1 as C2 as usize,
            (Sig::Cdecl2, 2) => c2_2 as C2 as usize,
            (Sig::Cdecl2, 3) => c2_3 as C2 as usize,
            (Sig::Cdecl3, 1) => c3_1 as C3 as usize,
            (Sig::Cdecl3, 2) => c3_2 as C3 as usize,
            (Sig::Cdecl3, 3) => c3_3 as C3 as usize,
            (Sig::Cdecl4, 1) => c4_1 as C4 as usize,
            (Sig::Cdecl4, 2) => c4_2 as C4 as usize,
            (Sig::Cdecl4, 3) => c4_3 as C4 as usize,
            (Sig::Cdecl5, 1) => c5_1 as C5 as usize,
            (Sig::Cdecl5, 2) => c5_2 as C5 as usize,
            (Sig::Cdecl5, 3) => c5_3 as C5 as usize,
            (Sig::This1, 1) => t1_1 as T1 as usize,
            (Sig::This1, 2) => t1_2 as T1 as usize,
            (Sig::This1, 3) => t1_3 as T1 as usize,
            (Sig::This3, 1) => t3_1 as T3 as usize,
            (Sig::This3, 2) => t3_2 as T3 as usize,
            (Sig::This3, 3) => t3_3 as T3 as usize,
            _ => panic!("no stub for slot {slot}"),
        };
        u32::try_from(addr).expect("32-bit code address")
    }

    pub(super) fn set_stubs(slots: &[(u32, Sig)]) {
        for entry in &TABLE {
            entry.store(0, Ordering::Relaxed);
        }
        for &(slot, sig) in slots {
            TABLE[slot as usize].store(stub(slot, sig), Ordering::Relaxed);
        }
        // SAFETY: plain store of the table's address to the runtime's
        // table pointer; the table is a static that lives for the whole
        // run, and AtomicU32 has the layout of u32, which the runtime reads.
        unsafe {
            lf_checker_rt::CHECKER_CTABLE = TABLE.as_ptr().cast::<u32>();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recorder_logs_and_answers() {
        let _s = session();
        assert_eq!(record(1, &[5]), 0, "no answers: zero");
        set_answers(|slot, args| slot * 100 + args.iter().sum::<u32>());
        let (value, calls) = capture(|| record(2, &[3, 4]));
        assert_eq!(value, 207);
        assert_eq!(
            calls,
            [Call {
                slot: 2,
                args: vec![3, 4]
            }]
        );
        let ((), none) = capture(|| ());
        assert!(none.is_empty());
    }

    #[cfg(not(target_arch = "x86"))]
    #[test]
    #[allow(unsafe_code)]
    fn stand_in_globals_point_into_the_image() {
        let _s = session();
        let mut image = VaImage::new(0x0100_0000, 64);
        with_image(&mut image, || {
            // SAFETY: the pointer is inside the installed, aligned image,
            // which outlives this closure.
            unsafe { global::<u32>(0x0100_0010).write(0x1234_5678) };
        });
        assert_eq!(&image.bytes()[0x10..0x14], &[0x78, 0x56, 0x34, 0x12]);
        assert_eq!(relocated(0x0100_0010), 0x0100_0010);
    }
}
