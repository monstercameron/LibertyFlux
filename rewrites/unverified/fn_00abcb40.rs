// original: 0x00abcb40 collect_slot_rows (proposed)

/// Scan an array of slots and collect a row per accepted slot into a buffer.
///
/// `this` points to a collector object: buffer pointer at `+0x0c`, collected
/// count (16-bit) at `+0x10`, a nonzero gate half at `+0x12`, an over-limit
/// flag byte at `+0x14` and the last array pointer at `+0x18`. When the gate
/// half is zero it and the buffer pointer are cleared first; the count is
/// always cleared.
///
/// Each of the `iters` slots is 0x80 bytes starting at `arr`, with a kind word
/// at `+0x44`. Slots whose kind is 0 or 2 are accepted: a helper callee fills
/// an 8-word scratch buffer from the slot, and words of it are copied to the
/// next 32-byte row (`count * 0x20` past the buffer) as
/// `[w2, w1, w2, index, w4, w5, w6, w15]`, where `index` is the 0-based slot
/// number. Word 15 lies past what the callee writes, so it is whatever the
/// stack held (zero under the checker's defined stack fill); word 11 is
/// overwritten with the index before being read. Other kinds are skipped.
///
/// Afterwards the flag byte records whether `count` exceeds `limit` (both
/// compared signed); when it does, a second callee is invoked with
/// `(this, buffer, buffer + count * 0x20, 4)`. The array pointer is stored at
/// `+0x18` and returned.
///
/// Original: 0x00abcb40 (thiscall, three stack words; returns `arr`).
lf_checker_rt::export!(thiscall, rw_00abcb40(this: u32, arr: u32, iters: u32, limit: u32) -> u32 {
    unsafe {
        const BUF_PTR: u32 = 0x0c;
        const COUNT: u32 = 0x10;
        const GATE: u32 = 0x12;
        const OVER: u32 = 0x14;
        const LAST_ARR: u32 = 0x18;
        const SLOT_STRIDE: u32 = 0x80;
        const KIND_OFF: u32 = 0x44;
        const ROW_STRIDE: u32 = 0x20;
        const FILL_CALLEE: u32 = 1;
        const EMIT_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        if rd16(this.wrapping_add(GATE)) == 0 {
            wr16(this.wrapping_add(GATE), 0);
            wr32(this.wrapping_add(BUF_PTR), 0);
        }
        wr16(this.wrapping_add(COUNT), 0);
        let n = iters as i32;
        if n > 0 {
            let mut i = 0i32;
            while i < n {
                let slot = arr.wrapping_add((i as u32).wrapping_mul(SLOT_STRIDE));
                let kind = rd32(slot.wrapping_add(KIND_OFF));
                if kind == 2 || kind == 0 {
                    let mut scratch = [0u32; 16];
                    let _ = lf_checker_rt::callee_cdecl!(
                        FILL_CALLEE,
                        u32,
                        slot,
                        scratch.as_mut_ptr() as u32
                    );
                    let at = rd16(this.wrapping_add(COUNT));
                    let dst = rd32(this.wrapping_add(BUF_PTR))
                        .wrapping_add((at as u32).wrapping_mul(ROW_STRIDE));
                    wr32(dst, scratch[2]);
                    wr32(dst.wrapping_add(4), scratch[1]);
                    wr32(dst.wrapping_add(8), scratch[2]);
                    wr32(dst.wrapping_add(0x0c), i as u32);
                    wr32(dst.wrapping_add(0x10), scratch[4]);
                    wr32(dst.wrapping_add(0x14), scratch[5]);
                    wr32(dst.wrapping_add(0x18), scratch[6]);
                    wr32(dst.wrapping_add(0x1c), scratch[15]);
                    wr16(this.wrapping_add(COUNT), at.wrapping_add(1));
                }
                i += 1;
            }
        }
        let count = rd16(this.wrapping_add(COUNT));
        let over = (count as i32) > (limit as i32);
        unsafe { ((this.wrapping_add(OVER)) as *mut u8).write(over as u8) };
        if over {
            let base = rd32(this.wrapping_add(BUF_PTR));
            let end = base.wrapping_add((count as u32).wrapping_mul(ROW_STRIDE));
            let _ = lf_checker_rt::callee_thiscall!(EMIT_CALLEE, u32, this, base, end, 4u32);
        }
        wr32(this.wrapping_add(LAST_ARR), arr);
        arr
    }
});
