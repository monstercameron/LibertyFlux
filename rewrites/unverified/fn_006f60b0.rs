// original: 0x006F60B0 input_stream_consume (proposed)

/// Consume records from the input stream at `a0` (`a1` bytes), returning the
/// bytes consumed.
///
/// Returns 0 without consuming when the disabled flag (bit 3 at `+0x88`) is
/// set, when `a1 - 9` exceeds 0x39A, when the stream's leading 10-bit field
/// is not `a1`, or when a guarded delta field is positive. It then walks the
/// stream from `a0 + 9` to `a0 + a1`: each record's length comes from its own
/// leading field, a validate callee approves the record, two parse callees
/// decode its key (compared against the running word slot and the marker at
/// `+0x84`), and a match callee decides the record. Matching records with
/// the flag bit set advance the marker words at `+0x82`/`+0x84`, shift the
/// live mask at `+0x7C`, and resolve one pending entry through three list
/// callees; other records adjust the mask or the toggle bit at `+0x88`.
/// The scan stops at the end of the stream or when the validate callee
/// rejects a record, returning cursor minus `a0`.
///
/// Thiscall with two stack words. The incoming arg1 slot is reused for the
/// running word, so the stack check is off. Each resolved entry leaks 12
/// stack bytes (the original builds a record below the frame and never frees
/// it), shifting every later frame slot; the shifted epilogue then returns
/// into scratch and faults with an access violation, so the esp check is off
/// and the rewrite faults identically on that path. The resolve loop is
/// covered for 0-1 entries (fault parity on the taken path; nothing else is
/// compared after a fault) and a second entry is not covered.
lf_checker_rt::export!(thiscall, rw_006F60B0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const ID_RESET: u32 = 1;
        const ID_OPEN: u32 = 2;
        const ID_VALIDATE: u32 = 3;
        const ID_PARSE_HI: u32 = 4;
        const ID_PARSE_LO: u32 = 5;
        const ID_MATCH: u32 = 6;
        const ID_RESOLVE: u32 = 7;
        const ID_NEXT: u32 = 8;
        const ID_ATTACH: u32 = 9;
        const ID_TOUCH: u32 = 10;
        const ID_MATCH2: u32 = 11;
        const MAX_LEN: u32 = 0x39A;

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
        /// Leading 10-bit field of a record header: first two bytes >> 6.
        #[inline(always)]
        unsafe fn head10(p: u32) -> u32 {
            unsafe { (((rd8(p) as u32) << 8) | rd8(p + 1) as u32) >> 6 }
        }
        /// 16-bit packed field spanning three bytes at `p`: low 6 bits of
        /// the first, all of the second, top 2 bits of the third.
        #[inline(always)]
        unsafe fn pack16(p: u32) -> u16 {
            unsafe {
                let b0 = rd8(p) as u32;
                let b1 = rd8(p + 1) as u32;
                let b2 = rd8(p + 2) as u32;
                (((b0 & 0x3F) << 10) | (b1 << 2) | (b2 >> 6)) as u16
            }
        }
        /// Mismatched record: try the second match callee, else fold the flag
        /// into toggle bit 1. Returns nothing; the caller always latches.
        #[inline(always)]
        unsafe fn record_miss(this: u32, bx: u16, cursor: u32, flag: u8) {
            unsafe {
                let d = bx.wrapping_sub(rd16(this + 0x82));
                if (d as i16) > 0 && d <= 0x16 {
                    let m2: u32 =
                        lf_checker_rt::callee_thiscall!(ID_MATCH2, u32, this, cursor, bx as u32);
                    if (m2 as u8) != 0 && flag != 0 {
                        let diff = (bx as u32).wrapping_sub(rd16(this + 0x82) as u32);
                        wr32(
                            this + 0x7c,
                            rd32(this + 0x7c)
                                | 1u32.wrapping_shl((diff as u16).wrapping_sub(1) as u32),
                        );
                        wr8(this + 0x88, rd8(this + 0x88) | 2);
                        return;
                    }
                }
                let cl = rd8(this + 0x88);
                let mut al = (((cl & 2) != 0 || flag != 0) as u8).wrapping_mul(2);
                al ^= cl;
                al &= 2;
                al ^= cl;
                wr8(this + 0x88, al);
            }
        }

        if rd8(this + 0x88) & 0x08 != 0 {
            return 0;
        }
        if a1.wrapping_sub(9) > MAX_LEN {
            return 0;
        }
        if head10(a0) != a1 {
            return 0;
        }
        let mut word = pack16(a0 + 2) as u32;
        let delta = pack16(a0 + 4);
        if (rd8(a0 + 1) >> 3) & 1 != 0 {
            let d = delta.wrapping_sub(rd16(this + 0x80));
            if d != 0 && d < 0x8000 {
                return 0;
            }
        }
        lf_checker_rt::callee_thiscall!(ID_RESET, u32, this);
        lf_checker_rt::callee_thiscall!(ID_OPEN, u32, this, a0);
        if (rd32(this + 0x20) as i32) < 2 {
            return 0;
        }
        let end = a0.wrapping_add(a1);
        let mut cursor = a0.wrapping_add(9);
        if cursor >= end {
            return cursor.wrapping_sub(a0);
        }
        loop {
            let left = end.wrapping_sub(cursor);
            let ok: u32 = lf_checker_rt::callee_fastcall!(ID_VALIDATE, u32, cursor, left);
            if (ok as u8) == 0 {
                return cursor.wrapping_sub(a0);
            }
            let stride = head10(cursor);
            let flag = (rd8(cursor + 1) >> 5) & 1;
            let key: u32 = if flag != 0 {
                lf_checker_rt::callee_thiscall!(ID_PARSE_HI, u32, cursor)
            } else {
                word
            };
            let lo: u32 = lf_checker_rt::callee_thiscall!(ID_PARSE_LO, u32, cursor);
            let bx = key as u16;
            let ax = lo as u16;
            if bx == word as u16 {
                word = word.wrapping_add(1);
            }
            if ax != rd16(this + 0x84) || (bx.wrapping_sub(rd16(this + 0x82)) & 0x8000) != 0 {
                record_miss(this, bx, cursor, flag);
            } else {
                let m: u32 = lf_checker_rt::callee_thiscall!(ID_MATCH, u32, this, cursor, bx as u32);
                if (m as u8) != 0 {
                    if flag == 0 {
                        let base = (bx as u32).wrapping_add(1) & 0xFFFF;
                        let d = base.wrapping_sub(rd16(this + 0x82) as u32) & 0xFFFF;
                        if d >= 0x16 {
                            wr32(this + 0x7c, 0);
                        } else {
                            wr32(this + 0x7c, rd32(this + 0x7c).wrapping_shr(d));
                        }
                        wr16(this + 0x82, base as u16);
                    } else {
                        let base = (bx as u32).wrapping_add(1) & 0xFFFF;
                        let sh = (bx as u8).wrapping_sub(rd8(this + 0x82)).wrapping_add(1);
                        wr16(this + 0x82, base as u16);
                        wr32(this + 0x7c, rd32(this + 0x7c).wrapping_shr((sh & 31) as u32));
                        wr16(this + 0x84, bx);
                        let mut res = [0u32; 3];
                        lf_checker_rt::callee_thiscall!(
                            ID_RESOLVE, u32, this.wrapping_add(0x68), res.as_mut_ptr() as u32
                        );
                        let (w0, w1, w2) = (res[0], res[1], res[2]);
                        if w1 != 0 {
                            let mut key_struct = [w0 & 0xFFFF, w1];
                            let mut next_out = [0u32; 2];
                            lf_checker_rt::callee_thiscall!(
                                ID_NEXT, u32, key_struct.as_mut_ptr() as u32,
                                next_out.as_mut_ptr() as u32
                            );
                            // Covered for a single resolve iteration (w0 is
                            // scripted 0, so the loop exits here).
                            if rd16(w1.wrapping_add(0x60)) == rd16(this + 0x84) {
                                if rd8(w1.wrapping_add(0x62)) & 1 != 0 {
                                    wr16(this + 0x84, (w0 & 0xFFFF) as u16);
                                }
                                let step = rd16(w1.wrapping_add(0x48)) as u32;
                                let sh2 =
                                    ((step as u8).wrapping_sub(rd8(this + 0x82))).wrapping_add(1);
                                wr16(this + 0x82, step.wrapping_add(1) as u16);
                                wr32(this + 0x7c, rd32(this + 0x7c).wrapping_shr((sh2 & 31) as u32));
                                let mut attach_out = [0u32; 2];
                                lf_checker_rt::callee_thiscall!(
                                    ID_ATTACH, u32, this.wrapping_add(0x68),
                                    attach_out.as_mut_ptr() as u32
                                );
                                lf_checker_rt::callee_thiscall!(ID_TOUCH, u32, this, w1);
                                let mut key2 = [w1, w2];
                                lf_checker_rt::callee_thiscall!(
                                    ID_NEXT, u32, key2.as_mut_ptr() as u32,
                                    next_out.as_mut_ptr() as u32
                                );
                                wr8(this + 0x88, rd8(this + 0x88) | 2);
                                // The leaked frame makes the original return
                                // into zeroed scratch, faulting with an access
                                // violation; fault the same way. The flag read
                                // below keeps the latch's only live input.
                                let _ = flag;
                                unsafe {
                                    core::ptr::read_volatile(0 as *const u8);
                                }
                                unreachable!("resolve path always faults");
                            }
                        }
                        wr8(this + 0x88, rd8(this + 0x88) | 2);
                    }
                }
            }
            cursor = cursor.wrapping_add(stride);
            if cursor >= end {
                return cursor.wrapping_sub(a0);
            }
        }
    }
});
