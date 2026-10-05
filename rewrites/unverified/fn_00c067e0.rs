// original: 0x00c067e0 stream_object_adopt
/// Adopt the object `src` into `this`, refreshing replaced slots.
///
/// Copies the 0x20 header bytes from `src` to `this` and returns `this` when
/// `src == this`. Otherwise, when the 16-bit tags at +0x24 differ and this
/// object's slot count at +0x26 is non-zero, refreshes each old slot through
/// the slot helper (thiscall/0) and releases the old slot array (cdecl/1),
/// then installs `src`'s slots through the slot-array helper (thiscall/1).
/// Thiscall: one stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00c067e0(this: u32, src: u32) -> u32 {
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

        const SLOT_FN: u32 = 1;
        const RELEASE: u32 = 2;
        const INSTALL: u32 = 3;
        const HEAD: u32 = 0x20;
        const SLOTS_OFF: u32 = 0x20;
        const TAG_OFF: u32 = 0x24;
        const COUNT_OFF: u32 = 0x26;
        const SLOT_STRIDE: u32 = 80;
        for i in 0..HEAD {
            wr8(this + i, rd8(src + i));
        }
        if this + SLOTS_OFF == src + SLOTS_OFF {
            return this;
        }
        if rd16(this + TAG_OFF) != rd16(src + TAG_OFF) {
            let n = rd16(this + COUNT_OFF);
            if n != 0 {
                let arr = rd32(this + SLOTS_OFF);
                let mut slot = arr;
                let mut left = n;
                while left != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(SLOT_FN, u32, slot);
                    slot = slot.wrapping_add(SLOT_STRIDE);
                    left -= 1;
                }
                let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, arr);
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(INSTALL, u32, this + SLOTS_OFF, src + SLOTS_OFF);
        this
    }
});
