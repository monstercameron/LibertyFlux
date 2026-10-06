// original: 0x00a9c170 filemem_close_all_handles_b

/// Close every open handle in the second table, then reset its count.
///
/// Same shape as `rw_00a9b3f0`: `this` points to a table whose signed
/// 32-bit count sits at `+0x3010`, with one entry per slot from `+0x3020`,
/// each 0x30 bytes. A slot whose handle word is non-null is processed:
/// when its object at `+0x34` is non-null and its status word nonzero it
/// is first flushed through the flush callee; then, whenever the
/// (re-read) handle word is still non-null, the slot is torn down through
/// the close callee and its handle word cleared. A non-positive count
/// closes nothing. Both bounds are SIGNED.
///
/// Original: 0x00A9C170 (thiscall, no stack arguments; two direct callees).
lf_checker_rt::export!(thiscall, rw_00a9c170(this: u32) -> u32 {
    unsafe {
        /// Slot count (signed), from the table base.
        const COUNT_OFF: u32 = 0x3010;
        /// First slot, from the table base; slots are 0x30 bytes apart.
        const SLOTS_OFF: u32 = 0x3020;
        const STRIDE: u32 = 0x30;
        /// Object pointer, from a handle.
        const HANDLE_OBJ: u32 = 0x34;
        const FLUSH: u32 = 1;
        const CLOSE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let n = rd32(this.wrapping_add(COUNT_OFF)) as i32;
        if n <= 0 {
            wr32(this.wrapping_add(COUNT_OFF), 0);
            return 0;
        }
        let mut i: i32 = 0;
        let mut slot = this.wrapping_add(SLOTS_OFF);
        loop {
            let h = rd32(slot);
            if h != 0 {
                let obj = rd32(h.wrapping_add(HANDLE_OBJ));
                if obj != 0 && rd32(obj) != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(FLUSH, u32, this, slot);
                }
                let h2 = rd32(slot);
                if h2 != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(CLOSE, u32, h2, slot);
                    wr32(slot, 0);
                }
            }
            i = i.wrapping_add(1);
            slot = slot.wrapping_add(STRIDE);
            if !(i < n) {
                break;
            }
        }
        wr32(this.wrapping_add(COUNT_OFF), 0);
        0
    }
});
