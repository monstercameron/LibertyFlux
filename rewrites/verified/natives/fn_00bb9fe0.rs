// original: 0x00bb9fe0 TASK_LOOK_AT_OBJECT
use lf_k2_rt::{callee_cdecl, export};
//! Rewrite of native handler TASK_LOOK_AT_OBJECT (original at 0x00bb9fe0).
//!
//! Script call context: the handler receives one pointer. At +0 sits the
//! result-slot pointer where a return value is stored, at +8 the argument
//! array. The handler forwards the arguments to one engine routine and, for
//! natives with a return value, stores the answer in the result slot.


/// Call context handed to a native handler by the script VM.
#[repr(C)]
pub struct NativeContext {
    /// Where a return value is stored (only used by natives that return one).
    pub result: *mut u32,
    /// Unused by this handler.
    pub _reserved: u32,
    /// Pointer to the argument array, one word per script argument.
    pub args: *const u32,
}

/// Tasks a character with looking at an object for a duration.
///
/// Handler behaviour: reads 4 argument words from the call context,
/// forwards slot 0 (raw word), slot 1 (raw word), slot 2 (raw word), slot 3 (raw word) to the engine routine, and returns nothing to the script.
///
/// The declared `u32` return is the observed exit value of `eax` (the
/// engine answer, left in place); the script VM ignores it.
export!(cdecl, rn23_task_look_at_object(ctx: *const NativeContext) -> u32 {
    unsafe {
        let _caller_slot = ctx as u32;
        let ctx = &*ctx;
        let argv = ctx.args;
        let a0 = *argv.add(0);
        let a1 = *argv.add(1);
        let a2 = *argv.add(2);
        let a3 = *argv.add(3);
        let answer: u32 = callee_cdecl!(1, u32, a0, a1, a2, a3);
        answer
    }
});
