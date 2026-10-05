// original: 0x0069B170 rage::crAnimChannelRawBool::try_build_from_samples

/// Rebuilds a raw bool channel's bit array from sample bytes.
///
/// `this` holds an array struct at `+8` (`{ptr, count}` plus capacity words
/// the allocator helper owns); the stack arguments are the sample bytes and
/// the sample `count`. A non-null old buffer is released through the thread
/// allocator reached as `tls[0] -> [+8] -> vtable[+0xC]` (callee 2,
/// thiscall: allocator, old buffer), then the pair is zeroed. The byte size
/// is `ceil(count / 8)`; a non-positive size (signed) returns 1 with an
/// empty array. Otherwise the allocator helper (callee 1, thiscall: pair,
/// size) installs the new buffer at `[pair]` and each output byte packs 8
/// samples (non-zero byte sets the bit, zero clears it), with the sample
/// index clamped to `count - 1` (signed), so a short final byte repeats the
/// last sample. Every bit of every byte is written, so the buffer's initial
/// contents are irrelevant. Counts above a small bound are excluded from the
/// contract (the original loops once per byte with no cap, so a huge count
/// hangs it). Returns 1 in `al`.
///
/// Original: 0x0069B170 (thiscall, two stack words, callee pops 8).
lf_checker_rt::export!(thiscall, rw_0069B170(this: u32, samples: u32, count: u32) -> u32 {
    unsafe {
        const PAIR_OFF: u32 = 8;
        const TLS_SLOT: usize = 0;
        const HEAPOBJ_OFF: u32 = 8;
        const FREE_SLOT: u32 = 0x0C;
        const ALLOC: u32 = 1;
        const FREE: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let esi = this + PAIR_OFF;
        let old = rd32(esi);
        if old != 0 {
            let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
            let heap_obj = rd32(tls_base + HEAPOBJ_OFF);
            let vtable = rd32(heap_obj);
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(rd32(vtable + FREE_SLOT) as usize) };
            free(heap_obj, old);
        }
        wr32(esi + 4, 0);
        wr32(esi, 0);
        let ebp = (count >> 3) + ((count & 7 != 0) as u32);
        if (ebp as i32) <= 0 {
            return 1;
        }
        let _ = lf_checker_rt::callee_thiscall!(ALLOC, u32, esi, ebp);
        let edx = count.wrapping_sub(1);
        let mut edi: u32 = 0;
        let mut ecx: u32 = 0;
        loop {
            let buf = rd32(esi);
            let mut bit = 0u32;
            while bit < 8 {
                let s = unsafe { (samples.wrapping_add(edi) as *const u8).read() };
                let addr = buf.wrapping_add(ecx);
                let cur = unsafe { (addr as *const u8).read() };
                let mask = 1u8 << bit;
                let nv = if s != 0 { cur | mask } else { cur & !mask };
                unsafe { (addr as *mut u8).write(nv) };
                let nx = edi.wrapping_add(1);
                edi = if (nx as i32) < (edx as i32) { nx } else { edx };
                bit += 1;
            }
            ecx = ecx.wrapping_add(1);
            if !((ecx as i32) < (ebp as i32)) {
                break;
            }
        }
        1
    }
});
