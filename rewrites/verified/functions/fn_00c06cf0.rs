// original: 0x00c06cf0 stream_view_register_copy
/// Register a fresh view object and copy `src` into it.
///
/// Same prologue as the register-and-init helper (prepare, allocate 0x11c,
/// default-initialise or null, store at `arr`+`count`*4-4), then copies `src`
/// into the fresh slot through the view-copy helper (thiscall/1) and returns
/// the slot. Thiscall: one stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00c06cf0(this: u32, src: u32) -> u32 {
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

        const PREP: u32 = 1;
        const ALLOC: u32 = 2;
        const VIEW: u32 = 3;
        const COPY: u32 = 4;
        const VIEW_SIZE: u32 = 0x11c;
        let _: u32 = lf_checker_rt::callee_thiscall!(PREP, u32, this + 8, 0x10);
        let raw: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, VIEW_SIZE);
        let slot: u32 = if raw != 0 {
            lf_checker_rt::callee_thiscall!(VIEW, u32, raw)
        } else {
            0
        };
        let arr = rd32(this + 8);
        let count = rd16(this + 0x0c);
        wr32(arr.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4), slot);
        // The original re-reads the count here; with count 0 the store above
        // has clobbered it, and the callee address comes from the re-read.
        let count2 = rd16(this + 0x0c);
        let target = rd32(arr.wrapping_add(count2.wrapping_mul(4)).wrapping_sub(4));
        let _: u32 = lf_checker_rt::callee_thiscall!(COPY, u32, target, src);
        rd32(arr.wrapping_add(rd16(this + 0x0c).wrapping_mul(4)).wrapping_sub(4))
    }
});
