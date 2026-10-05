// original: 0x00c06860 stream_slot_insert_at
/// Insert a fresh slot at position `at`, shifting later slots up.
///
/// Grows the array through the grow helper (thiscall/1) when the slot count
/// at +4 already equals the capacity at +6 (the grow decrements the count as
/// part of its protocol). Then rebuilds every slot from the end down to
/// `at` through the rebuild helper (thiscall/1), bumps the count, initialises
/// the new slot through the slot helper (thiscall/0), copies slot `src` over
/// it through the rebuild helper, and returns the new slot's address.
/// Thiscall: `this` in ECX, two stack words, callee cleans 8.
lf_checker_rt::export!(thiscall, rw_00c06860(this: u32, src: u32, at: u32) -> u32 {
    unsafe {
#[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        const GROW: u32 = 1;
        const REBUILD: u32 = 2;
        const SLOT_FN: u32 = 3;
        const COUNT_OFF: u32 = 4;
        const CAP_OFF: u32 = 6;
        const SLOT_STRIDE: u32 = 80;
        let mut count = rd16(this + COUNT_OFF);
        if count == rd16(this + CAP_OFF) {
            let _: u32 = lf_checker_rt::callee_thiscall!(GROW, u32, this, 0x10);
            count = count.wrapping_sub(1) & 0xFFFF;
            wr16(this + COUNT_OFF, count as u16);
        }
        let mut i = count as i32;
        while i > at as i32 {
            let slot = rd32(this).wrapping_add((i as u32).wrapping_mul(SLOT_STRIDE));
            let _: u32 = lf_checker_rt::callee_thiscall!(REBUILD, u32, slot, slot.wrapping_sub(SLOT_STRIDE));
            i -= 1;
        }
        wr16(this + COUNT_OFF, count.wrapping_add(1) as u16);
        let fresh = rd32(this).wrapping_add((at as u32).wrapping_mul(SLOT_STRIDE));
        let _: u32 = lf_checker_rt::callee_thiscall!(SLOT_FN, u32, fresh);
        let from = rd32(this).wrapping_add((src as u32).wrapping_mul(SLOT_STRIDE));
        let _: u32 = lf_checker_rt::callee_thiscall!(REBUILD, u32, fresh, from);
        fresh
    }
});
