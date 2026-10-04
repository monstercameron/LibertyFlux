// original: 0x00bc8160 STOP_CAR_BREAKING
//! Rewrite of native handler STOP_CAR_BREAKING (original at 0x00bc8160).
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

/// Stops a car braking, with an option to keep the brake lights on.
///
/// Handler behaviour: reads 2 argument words from the call context,
/// forwards slot 0 (raw word), slot 1 (flag (nonzero becomes 1)) to the engine routine, and returns nothing to the script.
///
/// The flag word pushed to the engine keeps the high bytes of this
/// handler's own incoming context-pointer slot: the original writes only
/// the low byte there (a `setne` into the dead argument slot), so the
/// pushed word is `(ctx & !0xFF) | flag`. Reproduced exactly.
///
/// The declared `u32` return is the observed exit value of `eax` (the
/// engine answer, left in place); the script VM ignores it.
export!(cdecl, rn23_stop_car_breaking(ctx: *const NativeContext) -> u32 {
    unsafe {
        let caller_slot = ctx as u32;
        let ctx = &*ctx;
        let argv = ctx.args;
        let a0 = *argv.add(0);
        let flag1 = u32::from(*argv.add(1) != 0);
        let a1 = (caller_slot & 0xFFFF_FF00) | flag1;
        let answer: u32 = callee_cdecl!(1, u32, a0, a1);
        answer
    }
});
