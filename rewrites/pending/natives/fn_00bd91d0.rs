// original: 0x00bd91d0 SET_GFWL_HAS_SAFE_HOUSE
//! Rewrite of native handler SET_GFWL_HAS_SAFE_HOUSE (original at 0x00bd91d0).
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

/// Tells the live service layer whether the player has a safe house.
///
/// Handler behaviour: reads 1 argument word from the call context,
/// forwards slot 0 (flag (nonzero becomes 1)) to the engine routine, and returns nothing to the script.
///
/// The flag word pushed to the engine keeps the high bytes of this
/// handler's own incoming context-pointer slot: the original writes only
/// the low byte there (a `setne` into the dead argument slot), so the
/// pushed word is `(ctx & !0xFF) | flag`. Reproduced exactly.
///
/// The declared `u32` return is the observed exit value of `eax` (the
/// engine answer, left in place); the script VM ignores it.
export!(cdecl, rn23_set_gfwl_has_safe_house(ctx: *const NativeContext) -> u32 {
    unsafe {
        let caller_slot = ctx as u32;
        let ctx = &*ctx;
        let argv = ctx.args;
        let flag0 = u32::from(*argv.add(0) != 0);
        let a0 = (caller_slot & 0xFFFF_FF00) | flag0;
        let answer: u32 = callee_cdecl!(1, u32, a0);
        answer
    }
});
