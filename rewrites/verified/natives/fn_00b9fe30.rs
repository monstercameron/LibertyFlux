// original: 0x00b9fe30 IS_CHAR_SWIMMING
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `IS_CHAR_SWIMMING`.
///
/// Reports whether the character is swimming.
///
/// Handler mechanics: takes the native call context,
/// Forwards the character handle, then stores the low byte of the engine
/// answer in the return slot.
export!(cdecl, rw_00b9fe30(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let ret = unsafe { *(ctx as *const u32) } as *mut u32;
    let who = unsafe { *args };
    let swimming = callee_cdecl!(1, u32, who) & 0xFF;
    unsafe { *ret = swimming };
});
