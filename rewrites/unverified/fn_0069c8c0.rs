// original: 0x0069C8C0 rage::crAnimChannelCurveFloat::copy_segment

/// Duplicates one curve-float key segment into fresh storage.
///
/// `this` is the destination segment and the stack argument the source.
/// The header (word at `+0`, bytes at `+2`/`+3`) is copied, then
/// `(count + 1) * 4` bytes are allocated through the thread allocator
/// reached as `tls[0] -> [+8] -> vtable[+8]` (callee 1, thiscall:
/// allocator, bytes, `0x10`, `0`; the original's multiply-overflow guard
/// cannot trigger for a byte count) and the `count + 1` words are copied
/// from the source words at `[src+4]`. Returns the last word copied.
///
/// Original: 0x0069C8C0 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_0069C8C0(this: u32, src: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const ALLOC_OBJ_OFF: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const ALLOC_HINT: u32 = 0x10;
        const COUNT_OFF: u32 = 2;
        const WORDS_OFF: u32 = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        unsafe {
            let w0 = ((src) as *const u16).read_unaligned();
            ((this) as *mut u16).write_unaligned(w0);
            let b2 = ((src + 2) as *const u8).read();
            ((this + 2) as *mut u8).write(b2);
            let b3 = ((src + 3) as *const u8).read();
            ((this + 3) as *mut u8).write(b3);
        }
        let n = unsafe { ((this + COUNT_OFF) as *const u8).read() as u32 };
        let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
        let heap_obj = rd32(tls_base + ALLOC_OBJ_OFF);
        let vtable = rd32(heap_obj);
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vtable + ALLOC_SLOT) as usize) };
        let block = alloc(heap_obj, n.wrapping_add(1).wrapping_mul(4), ALLOC_HINT, 0);
        wr32(this + WORDS_OFF, block);
        let srcbase = rd32(src + WORDS_OFF);
        let mut i: u32 = 0;
        let mut last: u32 = 0;
        loop {
            last = rd32(srcbase.wrapping_add(i.wrapping_mul(4)));
            wr32(block.wrapping_add(i.wrapping_mul(4)), last);
            i += 1;
            if i > n {
                break;
            }
        }
        last
    }
});
