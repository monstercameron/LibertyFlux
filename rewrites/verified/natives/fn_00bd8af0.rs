// original: 0x00bd8af0 NETWORK_PLAYER_HAS_DIED_RECENTLY
use lf_k2_rt::{callee_cdecl, export};
//! Rewrite of native handler NETWORK_PLAYER_HAS_DIED_RECENTLY (original at 0x00bd8af0).
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

/// Reports whether a network player died recently.
///
/// Handler behaviour: reads 1 argument word from the call context,
/// forwards slot 0 (raw word) to the engine routine, and stores the engine answer's low byte as a word result.
///
/// The declared `u32` return is the observed exit value of `eax` (the
/// result-slot pointer); the script reads the stored word, not `eax`.
export!(cdecl, rn23_network_player_has_died_recently(ctx: *const NativeContext) -> u32 {
    unsafe {
        let _caller_slot = ctx as u32;
        let ctx = &*ctx;
        let argv = ctx.args;
        let a0 = *argv.add(0);
        let answer: u32 = callee_cdecl!(1, u32, a0);
        *ctx.result = answer & 0xFF;
        ctx.result as u32
    }
});
