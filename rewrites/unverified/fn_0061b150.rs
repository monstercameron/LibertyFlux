// original: 0x0061B150 net_session_desc_init (proposed)

/// Build an eight-record session descriptor on the stack, resolve it through
/// a helper, and store the answer at the object's first word.
///
/// `this` points at an object whose first word receives the helper's answer.
/// The descriptor is eight 28-byte records: record 0 holds
/// `[0, 0, 0, 0x10, 7, 0, word0]` and records 1-7 hold
/// `[0, 0, 0, 0, 6, 0, word0]`. Each record's last word is written as a
/// 16-bit store, leaving its upper half untouched. The helper is called as
/// fastcall(descriptor, 1); its answer is stored at `[this]` and also left
/// as the incidental return value.
///
/// Original: 0x0061B150 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0061B150(this: u32) -> u32 {
    unsafe {
        const NREC: usize = 8;
        const REC_LEN: usize = 28;
        const REC_TAG: u32 = 6;
        const HEAD_D3: u32 = 0x10;
        const HEAD_D4: u32 = 7;
        const HELPER: u32 = 1;
        const COOKIE: u32 = 2;

        #[inline(always)]
        unsafe fn w32(base: *mut u8, off: usize, v: u32) {
            unsafe { (base.add(off) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn w16(base: *mut u8, off: usize, v: u16) {
            unsafe { (base.add(off) as *mut u16).write_unaligned(v) }
        }

        let mut buf = core::mem::MaybeUninit::<[u8; 224]>::uninit();
        let base = buf.as_mut_ptr() as *mut u8;
        w32(base, 0x00, 0);
        w32(base, 0x04, 0);
        w32(base, 0x08, 0);
        w32(base, 0x0c, HEAD_D3);
        w32(base, 0x10, HEAD_D4);
        w32(base, 0x14, 0);
        w16(base, 0x18, 0);
        let mut r = 1usize;
        while r < NREC {
            let o = r * REC_LEN;
            w32(base, o, 0);
            w32(base, o + 4, 0);
            w32(base, o + 8, 0);
            w32(base, o + 0x0c, 0);
            w32(base, o + 0x10, REC_TAG);
            w32(base, o + 0x14, 0);
            w16(base, o + 0x18, 0);
            r += 1;
        }
        let answer: u32 = lf_checker_rt::callee_fastcall!(HELPER, u32, base as u32, 1);
        (this as *mut u32).write_unaligned(answer);
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        answer
    }
});
