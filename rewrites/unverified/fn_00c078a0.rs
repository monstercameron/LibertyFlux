// original: 0x00c078a0 stream_slot_ptr_at
/// Return a pointer to the `index`-th 80-byte slot, or null when out of range.
///
/// The slot count is the 16-bit word at `this`+4 and the slot array starts at
/// the pointer in `this`+0. The comparison is signed: a negative index takes
/// the in-range path with wrapping arithmetic. Thiscall: `this` in ECX, one
/// stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00c078a0(this: u32, index: u32) -> u32 {
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
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        const COUNT_OFF: u32 = 4;
        const SLOTS_OFF: u32 = 0;
        const SLOT_STRIDE: i32 = 80;
        let count = rd16(this + COUNT_OFF) as i32;
        if (index as i32) < count {
            let base = rd32(this + SLOTS_OFF);
            base.wrapping_add((index as i32).wrapping_mul(SLOT_STRIDE) as u32)
        } else {
            0
        }
    }
});
