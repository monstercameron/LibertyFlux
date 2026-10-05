// original: 0x00c063a0 stream_table_load
/// Load this object's table fields through the reader helper.
///
/// Reads 2 bytes through the reader (cdecl/3) from a two-byte stack slot
/// holding the low byte of the count at +0x2c (the slot's upper bytes are
/// fixed marker/leftover bytes, reproduced exactly for the call snapshot),
/// then reads one 0x50-byte span per table entry (count at +0x2c, entries at
/// the array in +0x28), then the fixed fields at +0, +0x20, +0x24, +0x30,
/// +0x34, +0x38, +0x3c, +0x40 and +0x44 with their lengths, and finishes
/// through the span helper (thiscall/1). Returns the span helper's answer.
/// Thiscall: one stack word, callee cleans 4. Reader buffer addresses are
/// skipped (the first is a frame pointer); one word of content per call is
/// snap-compared instead.
lf_checker_rt::export!(thiscall, rw_00c063a0(this: u32, arg: u32) -> u32 {
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
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        const READ: u32 = 1;
        const SPAN: u32 = 2;
        const COUNT_OFF: u32 = 0x2c;
        const ARR_OFF: u32 = 0x28;
        // First call's buffer: low byte + fixed 0x50 marker + the high bytes
        // the original's pushed ECX (== this) leaves behind.
        let head_lo = rd8(this + COUNT_OFF);
        let head_word = (head_lo as u32) | 0x50_00 | (this & 0xFFFF_0000);
        let _: u32 = lf_checker_rt::callee_cdecl!(READ, u32, arg, &head_word as *const u32 as u32, 2);
        let count = rd16(this + COUNT_OFF);
        let arr = rd32(this + ARR_OFF);
        let mut i = 0u32;
        while i < count {
            let elem = rd32(arr.wrapping_add(i.wrapping_mul(4)));
            let _: u32 = lf_checker_rt::callee_cdecl!(READ, u32, arg, elem, 0x50);
            i += 1;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(READ, u32, arg, this, 0x20);
        for off in [0x20u32, 0x24, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x44] {
            let _: u32 = lf_checker_rt::callee_cdecl!(READ, u32, arg, this + off, 4);
        }
        lf_checker_rt::callee_thiscall!(SPAN, u32, this, arg)
    }
});
