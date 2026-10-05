// original: 0x00c07320 stream_compact_slots_from
/// Rebuild every slot from `index` on and drop the slot count by one.
///
/// For each slot position from the low 16 bits of `index` while below
/// `count - 1` (signed), where `count` is the 16-bit word at `this`+4, calls
/// the slot-rebuild helper (thiscall/1) with the slot address and the
/// following slot's address. Then decrements the count word (wrapping) and
/// returns 0xFFFF. Thiscall: `this` in ECX, one stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00c07320(this: u32, index: u32) -> u32 {
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

        const REBUILD: u32 = 1;
        const SLOTS_OFF: u32 = 0;
        const COUNT_OFF: u32 = 4;
        const SLOT_STRIDE: u32 = 80;
        const DONE: u32 = 0xFFFF;
        let mut i = index & 0xFFFF;
        loop {
            let count = rd16(this + COUNT_OFF) as i32;
            if (i as i32) >= count - 1 {
                break;
            }
            let slot = rd32(this + SLOTS_OFF).wrapping_add(i.wrapping_mul(SLOT_STRIDE));
            let _: u32 = lf_checker_rt::callee_thiscall!(REBUILD, u32, slot, slot.wrapping_add(SLOT_STRIDE));
            i = (i + 1) & 0xFFFF;
        }
        let count = rd16(this + COUNT_OFF);
        wr16(this + COUNT_OFF, count.wrapping_sub(1) as u16);
        DONE
    }
});
