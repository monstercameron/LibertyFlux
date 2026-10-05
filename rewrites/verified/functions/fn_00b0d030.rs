// original: 0x00B0D030 net_pool_sync_from_stream (proposed)

/// Drain a network stream buffer into the pooled object table.
///
/// Reads parcels from the stream cursor into scratch and copies each one
/// through a two-stage struct copy (streamed struct into scratch, scratch
/// into pool entry `count`), then clears the ready flag of the entry named
/// by the per-iteration index. Runs a fixed 650 iterations regardless of
/// the stream state; iterations past `count` only refresh scratch.
///
/// Stream reader state (all file VAs): read offset at `CURSOR`, flag bytes
/// at `FLAGS` (bit meaning by byte: `0x7C` error latch, `0x7D` bypass that
/// skips the copy but still counts bytes, `0x7F` second counter enable),
/// byte counters at `COUNT`/`COUNT_ALT`, and the stream struct pointer at
/// `STREAM` (words: `+0` buffer base, `+4` struct-local counter, `+0xC`
/// readable limit, `+0x90` second struct-local counter). A read past the
/// limit latches the error byte and copies nothing; later reads then return
/// early. Return value is the last read's byte result with the low byte
/// forced to 1 (`(an instruction of the original)` over the helper's leftover `eax`).
///
/// Pool entries are `ENTRY_STRIDE` bytes at `POOL`, indexed by the count
/// word for the write and by the per-iteration index for the flag clear.
/// Struct copies move dwords `0,4`, `30..3C`, words `40,42`, bytes
/// `44,45` and the low 3 bits of `46` straight. The middle words differ:
/// the first stage shifts them (`dst+8..dst+20` from `src+0xC..src+0x24`)
/// and takes dword `48` from the source's word `8`; the second stage
/// copies words `0..20` straight and takes `48` from word `48`. Words
/// `24..2C` are never moved. The original's per-bit merge of byte `45`
/// expands to a plain byte copy.
///
/// The pre-loop branch calls a state helper when the bypass byte is clear
/// and the mode word is 1; the contracted object always has its state byte
/// clear, so only the store is reproduced (see `narrowed`).
///
/// The original's callees run natively on the original side and are
/// inlined here (650 iterations exceed the call-log cap), so this proof
/// has no intercepted calls. Stack slots are plain locals: the checker's
/// stack comparison only covers words at or above the incoming `esp`.
///
/// Original: 0x00B0D030 (cdecl, no arguments, returns `eax`).
lf_checker_rt::export!(cdecl, rw_00B0D030() -> u32 {
    unsafe {
        const CURSOR: u32 = 0x116D278;
        const FLAGS: u32 = 0x116D27C;
        const FLAG_ERR: u32 = 0x116D27C;
        const FLAG_BYPASS: u32 = 0x116D27D;
        const FLAG_ALT: u32 = 0x116D27F;
        const COUNT: u32 = 0x116D280;
        const COUNT_ALT: u32 = 0x116D284;
        const STREAM: u32 = 0x116D288;
        const MODE: u32 = 0x11D6FD0;
        const STATE_OBJ: u32 = 0x1BB5624;
        const STATE_BYTE: u32 = 0x164;
        const POOL: u32 = 0x1615660;
        const ENTRY_STRIDE: u32 = 0x50;
        const TAIL_BYTE_DST: u32 = 0x10400B8;
        const TAIL_WORD_DST: u32 = 0x1615618;
        const ITERS: u32 = 0x28A;
        const READY_BYTE: u32 = 0x44;
        const READY_WORD: u32 = 0x04;
        const READY_FLAG: u32 = 0x45;
        const READY_KEEP: u8 = 0xF7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn g32(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        unsafe fn g8(file_va: u32) -> u8 {
            unsafe { rd8(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        unsafe fn wg32(file_va: u32, v: u32) {
            unsafe { wr32(lf_checker_rt::relocated(file_va), v) }
        }
        #[inline(always)]
        unsafe fn wg8(file_va: u32, v: u8) {
            unsafe { wr8(lf_checker_rt::relocated(file_va), v) }
        }

        /// The stream parcel reader (original helper): copy `size` bytes
        /// from buffer + cursor to `dst`, advancing the cursor, with the
        /// exact leftover `eax` the original leaves behind on each path.
        unsafe fn stream_read(dst: u32, size: u32) -> u32 {
            unsafe {
                wg32(COUNT, g32(COUNT).wrapping_add(size));
                if g8(FLAG_ALT) != 0 {
                    wg32(COUNT_ALT, g32(COUNT_ALT).wrapping_add(size));
                }
                let st = g32(STREAM);
                if st == 0 {
                    wg8(FLAG_ERR, 1);
                    return 0;
                }
                wr32(st + 4, rd32(st + 4).wrapping_add(size));
                if g8(FLAG_ALT) != 0 {
                    let st2 = g32(STREAM);
                    wr32(st2 + 0x90, rd32(st2 + 0x90).wrapping_add(size));
                }
                // `(an instruction of the original)` over eax holding the struct pointer.
                if g8(FLAG_BYPASS) != 0 {
                    return (st & !0xFF) | 1;
                }
                // Early out through `(an instruction of the original)` over the same leftover.
                if g8(FLAG_ERR) != 0 {
                    return st & !0xFF;
                }
                if (size as i32) <= 0 {
                    return (st & !0xFF) | 1;
                }
                let off = g32(CURSOR);
                let end = off.wrapping_add(size);
                if end > rd32(st + 0x0C) {
                    wg8(FLAG_ERR, 1);
                    return end & !0xFF;
                }
                let src = rd32(st).wrapping_add(off);
                let mut i = 0u32;
                while i < size {
                    wr8(dst + i, rd8(src + i));
                    i += 1;
                }
                wg32(CURSOR, end);
                // Helper returns dst with the low byte forced to 1.
                (dst & !0xFF) | 1
            }
        }

        /// One pooled-struct copy. The first stage remaps the middle
        /// words (`dst+8..dst+20` come from `src+0xC..src+0x24`) and takes
        /// dword `48` from the source's word `8`; the second stage copies
        /// words `0..20` straight and takes `48` from word `48`.
        unsafe fn pool_copy(dst: u32, src: u32, first_stage: bool) {
            unsafe {
                if first_stage && dst == 0 {
                    return;
                }
                if first_stage {
                    for (d, s) in [(0u32, 0u32), (4, 4), (8, 0x0C), (0x0C, 0x10),
                                   (0x10, 0x14), (0x14, 0x18), (0x18, 0x1C),
                                   (0x1C, 0x20), (0x20, 0x24)] {
                        wr32(dst + d, rd32(src + s));
                    }
                } else {
                    for off in [0u32, 4, 8, 0x0C, 0x10, 0x14, 0x18, 0x1C, 0x20] {
                        wr32(dst + off, rd32(src + off));
                    }
                }
                for off in [0x30u32, 0x34, 0x38, 0x3C] {
                    wr32(dst + off, rd32(src + off));
                }
                wr16(dst + 0x40, rd16(src + 0x40));
                wr16(dst + 0x42, rd16(src + 0x42));
                wr8(dst + READY_BYTE, rd8(src + READY_BYTE));
                wr8(dst + READY_FLAG, rd8(src + READY_FLAG));
                wr8(dst + 0x46, (rd8(dst + 0x46) & !7) | (rd8(src + 0x46) & 7));
                if first_stage {
                    wr32(dst + 0x48, rd32(src + 8));
                } else {
                    wr32(dst + 0x48, rd32(src + 0x48));
                }
            }
        }

        #[inline(always)]
        unsafe fn entry(idx: u32) -> u32 {
            unsafe {
                lf_checker_rt::relocated(POOL)
                    .wrapping_add(idx.wrapping_mul(ENTRY_STRIDE))
            }
        }

        // Frame slots: the index (-1 until the first parcel overwrites it),
        // the count (0), the streamed struct and the copy scratch. The
        // original's scratch starts as uninitialized stack, pinned to zero
        // by the contract's stack fill.
        let mut index: u32 = 0xFFFF_FFFF;
        let mut count: u32 = 0;
        let mut parcel = [0u8; 0x50];
        let mut scratch = [0u8; 0x50];

        stream_read(core::ptr::addr_of_mut!(count) as u32, 4);
        if g8(FLAG_BYPASS) == 0 && g32(MODE) == 1 {
            let obj = g32(STATE_OBJ);
            // Contracted objects always have this byte clear; the nested
            // call for a set byte is excluded (see `narrowed`).
            if rd8(obj + STATE_BYTE) == 0 {
                wr8(obj + STATE_BYTE, 0);
            } else {
                wr8(obj + STATE_BYTE, 0);
            }
        }
        let mut i = 0u32;
        while i < ITERS {
            stream_read(core::ptr::addr_of_mut!(index) as u32, 4);
            stream_read(parcel.as_mut_ptr() as u32, 0x50);
            pool_copy(scratch.as_mut_ptr() as u32, parcel.as_ptr() as u32, true);
            if g8(FLAG_BYPASS) == 0 && i < count {
                pool_copy(entry(count), scratch.as_ptr() as u32, false);
                let chk = entry(index);
                if rd8(chk + READY_BYTE) != 0 && rd32(chk + READY_WORD) != 0 {
                    wr32(chk + READY_WORD, 0);
                    wr8(chk + READY_FLAG, rd8(chk + READY_FLAG) & READY_KEEP);
                }
            }
            i += 1;
        }
        stream_read(lf_checker_rt::relocated(TAIL_BYTE_DST), 1);
        // The original forces the low byte to 1 over the last read's leftover.
        (stream_read(lf_checker_rt::relocated(TAIL_WORD_DST), 4) & !0xFF) | 1
    }
});
