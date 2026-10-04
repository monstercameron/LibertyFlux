// original: 0x006f4ac0 timer_window_init
/// Initializes a timer window from the half-open range [lo, hi].
///
/// Returns false without touching the object when lo is negative or above hi.
/// Otherwise stamps the raw shared-tick sample (fast function-pointer path
/// with a fallback) at slot 0, records the bounds, span and a default rate of
/// 100, clears the status flag, refreshes the derived value through the
/// combine helper, and returns true.
export!(thiscall, rw_006f4ac0(this: u32, lo: u32, hi: u32) -> u8 {
    unsafe {
        if (lo as i32) < 0 || lo > hi {
            return 0;
        }
        let obj = this as *mut u32;
        let raw = *global::<u32>(0x17ACD20);
        let tick = if raw == 0 {
            let fallback: extern "cdecl" fn() -> u32 =
                core::mem::transmute(*global::<u32>(0xE73474));
            fallback()
        } else {
            let prime: extern "cdecl" fn() -> u32 = core::mem::transmute(raw);
            prime();
            let convert: extern "cdecl" fn(*mut u32, *mut u32) -> u32 =
                core::mem::transmute(*global::<u32>(0x17ACD00));
            let mut words = [0u32; 3];
            convert(words.as_mut_ptr().add(1), words.as_mut_ptr());
            words[0]
        };
        *obj = tick;
        *obj.add(1) = 0;
        *((this + 0x20) as *mut u8) &= 0xFE;
        *obj.add(3) = hi;
        *obj.add(2) = lo;
        *obj.add(4) = hi.wrapping_sub(lo);
        *obj.add(7) = 100;
        callee_thiscall!(4, u32, this);
        1
    }
});
