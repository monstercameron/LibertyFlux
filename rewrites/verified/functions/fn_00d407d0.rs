// original: 0x00d407d0 CTaskSimpleSidewaysDive::vf5

/// Decide whether a sideways-dive task accepts the requested transition.
///
/// `kind` (arg1) is the requested transition and `next` (arg2) a task
/// object; incoming `this`, arg0 unused. Returns 1 at once unless `kind`
/// equals 1. On 1 with a null `next`, returns 0. Otherwise the virtual
/// slot at `CHECK_SLOT` (+0x18) of `next` is called (thiscall on `next`,
/// no stack words) and the result is 1 when its low byte is nonzero, else
/// 0. Only al carries the result.
///
/// Original: 0x00d407d0 (thiscall shape, three stack words; ecx unread).
lf_checker_rt::export!(thiscall, rw_00d407d0(_this: u32, _a0: u32, kind: u32, next: u32) -> u32 {
    unsafe {
        const CHECK_SLOT: u32 = 0x18;

        if kind != 1 {
            return 1;
        }
        if next == 0 {
            return 0;
        }
        let vtable = (next as *const u32).read_unaligned();
        let slot = ((vtable + CHECK_SLOT) as *const u32).read_unaligned();
        let check: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if check(next) & 0xff != 0 { 1 } else { 0 }
    }
});
