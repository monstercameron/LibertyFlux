// original: 0x00c7a670 ctask_mobile_chat_dtor

/// Destroy a scenario task and clamp the shared cooldown timer.
/// Installs `VTABLE`, then reads the float timer: when it is already
/// above `TIMER_MAX` (10.0) it is kept, otherwise the slot is reset to
/// 10.0. A NaN timer also resets (an unordered compare is not above).
/// Tail-calls the base destructor and returns its result.
/// Original: 0x00c7a670 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c7a670(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ED5F74;
        const TIMER: u32 = 0x0104B974;
        const TIMER_MAX: u32 = 0x00FE8B08;
        const TEN_BITS: u32 = 0x41200000;
        const BASE_DTOR: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this, lf_checker_rt::relocated(VTABLE));
        let slot = lf_checker_rt::global::<u32>(TIMER);
        let cur = f32::from_bits(slot.read());
        let max = f32::from_bits(rd32(lf_checker_rt::relocated(TIMER_MAX)));
        if !(cur > max) {
            slot.write(TEN_BITS);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
