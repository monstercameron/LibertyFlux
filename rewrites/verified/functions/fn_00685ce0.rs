// original: 0x00685ce0 dof_array_deduplicate (proposed)

/// Sort a frame-pointer array, then drop adjacent duplicates.
///
/// `this+0x10` holds a 16-bit count `n` and `this+0xc` an array of `n`
/// frame pointers. Counts below 2 only clear `this+8` and return.
/// Otherwise the sort callee sees `(base, base + n*4, 0)` and the array is
/// scanned for the first adjacent pair whose key bytes (`+0x05` byte and
/// `+0x06` word) agree. With no such pair the array is left alone, `this+8`
/// is cleared and the function returns. On a match the later element is
/// released through virtual slot 0 with argument 1 and the tail is
/// compacted: equal elements are released, differing ones bump the kept
/// count and have their `+0x04`..`+0x0f` bytes copied over the next kept
/// slot. Afterwards `this+0x10` becomes the kept count; when `this+0x12`
/// is zero it takes the count too and `this+0xc` is replaced by the
/// allocator callee's answer for that count (or null when it is zero).
/// Control then falls back into the scan with the count exhausted, so a
/// trial that finds a match runs past the array end until it faults; the
/// rewrite reproduces that walk exactly (same addresses, same order).
/// `this+8` is cleared on every clean exit. The stack word is unread.
///
/// Original: 0x00685ce0 (thiscall, one unread stack word, no result).
lf_checker_rt::export!(thiscall, rw_00685CE0(this: u32, _a0: u32) -> u32 {
    unsafe {
        const SORT: u32 = 1;
        const ALLOC: u32 = 2;
        const RELEASE: u32 = 3;
        const ARR_OFF: u32 = 0x0c;
        const COUNT_OFF: u32 = 0x10;
        const CAP_OFF: u32 = 0x12;
        const CLEARED_OFF: u32 = 0x08;
        const KEY_B: u32 = 0x05;
        const KEY_W: u32 = 0x06;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u32) {
            unsafe { (a as *mut u16).write_unaligned(v as u16) }
        }
        #[inline(always)]
        unsafe fn release(elem: u32) {
            unsafe {
                // Load the slot from the element and call through it like
                // the original; both sides land on the same planted stub.
                let slot: u32 = rd32(elem);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(elem, 1);
            }
        }
        #[inline(always)]
        unsafe fn copy12(src: u32, dst: u32) {
            unsafe {
                ((dst + 4) as *mut u8).write(rd8(src + 4));
                ((dst + 5) as *mut u8).write(rd8(src + 5));
                wr16(dst + 6, rd16(src + 6));
                wr32(dst + 8, rd32(src + 8));
                wr32(dst + 12, rd32(src + 12));
            }
        }

        let n = unsafe { rd16(this + COUNT_OFF) };
        if n <= 1 {
            unsafe { wr32(this + CLEARED_OFF, 0) };
            return 0;
        }
        let base0 = unsafe { rd32(this + ARR_OFF) };
        let _s: u32 = lf_checker_rt::callee_fastcall!(
            SORT,
            u32,
            base0,
            base0.wrapping_add(n.wrapping_mul(4)),
            0u32
        );
        // NOTE: the original compares n against 1 again here; n >= 2
        // always, so that arm is dead and omitted.
        let mut kept: u32 = 0;
        let mut i: u32 = 1;
        // Outer scan; `base` is reloaded every lap like the original.
        loop {
            let base = unsafe { rd32(this + ARR_OFF) };
            kept = i.wrapping_sub(1);
            let cur = unsafe { rd32(base.wrapping_add(i.wrapping_mul(4))) };
            let prev = unsafe { rd32(base.wrapping_add(kept.wrapping_mul(4))) };
            let same = unsafe { rd8(cur + KEY_B) == rd8(prev + KEY_B) }
                && unsafe { rd16(cur + KEY_W) == rd16(prev + KEY_W) };
            if !same {
                i = i.wrapping_add(1);
                if i == n {
                    unsafe { wr32(this + CLEARED_OFF, 0) };
                    return 0;
                }
                continue;
            }
            unsafe { release(cur) };
            i = i.wrapping_add(1);
            if i != n {
                // Inner compaction; base reloaded every lap.
                while i != n {
                    let base = unsafe { rd32(this + ARR_OFF) };
                    let cur = unsafe { rd32(base.wrapping_add(i.wrapping_mul(4))) };
                    let k = unsafe { rd32(base.wrapping_add(kept.wrapping_mul(4))) };
                    let same = unsafe { rd8(cur + KEY_B) == rd8(k + KEY_B) }
                        && unsafe { rd16(cur + KEY_W) == rd16(k + KEY_W) };
                    if same {
                        unsafe { release(cur) };
                    } else {
                        kept = kept.wrapping_add(1);
                        let dst = unsafe { rd32(base.wrapping_add(kept.wrapping_mul(4))) };
                        unsafe { copy12(cur, dst) };
                    }
                    i = i.wrapping_add(1);
                }
            }
            // AFTER: publish the kept count, reallocating when the
            // capacity word is zero, then fall back into the scan.
            let count = kept.wrapping_add(1);
            if unsafe { rd16(this + CAP_OFF) } != 0 {
                unsafe { wr16(this + COUNT_OFF, count) };
            } else {
                unsafe { wr16(this + CAP_OFF, count) };
                if count == 0 {
                    unsafe { wr32(this + ARR_OFF, 0) };
                } else {
                    let p: u32 = lf_checker_rt::callee_stdcall!(ALLOC, u32, count);
                    unsafe { wr32(this + ARR_OFF, p) };
                }
                unsafe { wr16(this + COUNT_OFF, count) };
            }
            i = i.wrapping_add(1);
            if i == n {
                unsafe { wr32(this + CLEARED_OFF, 0) };
                return 0;
            }
            // else re-enter the outer scan (past the array end).
        }
    }
});
