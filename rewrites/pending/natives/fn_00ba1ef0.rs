// original: 0x00ba1ef0 SET_DECISION_MAKER_ATTRIBUTE_SIGHT_RANGE
//! Rewrite of native handler SET_DECISION_MAKER_ATTRIBUTE_SIGHT_RANGE (original at 0x00ba1ef0).
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

/// Sets the sight-range attribute of a decision maker.
///
/// Handler behaviour: reads 2 argument words from the call context,
/// forwards slot 0 (raw word), slot 1 (raw word) to the engine routine, and returns nothing to the script.
///
/// The declared `u32` return is the observed exit value of `eax` (the
/// engine answer, left in place); the script VM ignores it.
export!(cdecl, rn23_set_decision_maker_attribute_sight_range(ctx: *const NativeContext) -> u32 {
    unsafe {
        let _caller_slot = ctx as u32;
        let ctx = &*ctx;
        let argv = ctx.args;
        let a0 = *argv.add(0);
        let a1 = *argv.add(1);
        let answer: u32 = callee_cdecl!(1, u32, a0, a1);
        answer
    }
});
