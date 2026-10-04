// original: 0x00bbaa40 TASK_SMART_FLEE_POINT_PREFERRING_PAVEMENTS
//! Rewrite of native handler TASK_SMART_FLEE_POINT_PREFERRING_PAVEMENTS (original at 0x00bbaa40).
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

/// Tasks a character with fleeing from a point, preferring pavements.
///
/// Handler behaviour: reads 6 argument words from the call context,
/// forwards slot 0 (raw word), slot 1 (float bits), slot 2 (float bits), slot 3 (float bits), slot 4 (float bits), slot 5 (raw word) to the engine routine, and returns nothing to the script.
///
/// The declared `u32` return is the observed exit value of `eax` (the
/// engine answer, left in place); the script VM ignores it.
export!(cdecl, rn23_task_smart_flee_point_preferring_pavements(ctx: *const NativeContext) -> u32 {
    unsafe {
        let _caller_slot = ctx as u32;
        let ctx = &*ctx;
        let argv = ctx.args;
        let a0 = *argv.add(0);
        let a1 = *argv.add(1); // float bit pattern, forwarded unchanged
        let a2 = *argv.add(2); // float bit pattern, forwarded unchanged
        let a3 = *argv.add(3); // float bit pattern, forwarded unchanged
        let a4 = *argv.add(4); // float bit pattern, forwarded unchanged
        let a5 = *argv.add(5);
        let answer: u32 = callee_cdecl!(1, u32, a0, a1, a2, a3, a4, a5);
        answer
    }
});
