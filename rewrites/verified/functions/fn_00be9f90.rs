// original: 0x00be9f90 MOTORBIKE_MAIN_COLLISIONS_COLLISIONS

/// Look up a collision record for task object `this` and release its head.
///
/// Returns 0 when the flag at `this+1` is clear. The count at `this+0x14`
/// selects the handle setup: negative counts other than -2 return 0, -2
/// skips setup, otherwise callee 1 maps the count to an object (null
/// returns 0) whose kind field at `+0x28`, bits 6..9, picks handle A
/// (kind 3) or handle B (kind 2).
///
/// The selector at `this+0x18` (minus 10, above 0xa1 returns 0) then
/// chooses one of 23 arms computing a record pointer: seven shared-table
/// scans for a zero word (three stride-0x80 pair scans, three linear
/// scans, one 32-entry pair scan, two 6-word scans, one counted stride-0x20
/// scan), ten offsets into handle A, three offsets into handle B (a null
/// handle returns 0), or one fixed address. A scan with no hit yields a
/// null pointer. One arm (selector 0x0a/0x0c) additionally runs two rounds
/// of resolve-and-report: each round resolves a cached address through
/// callee 2 unless its flag bit is already set, compares it against
/// `this+0x0c`, and on a match with shared mode 2..4 prepares an output
/// block through callee 3 and reports it with a fixed address to the fixed
/// shared object through callee 4.
///
/// The tail reads the record's head word; a non-zero head goes to callee 5
/// with a zero word and the head is cleared. Returns the record pointer
/// (a null record faults on the head read, like the original).
///
/// Original: 0x00be9f90 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00be9f90(this: u32) -> u32 {
    unsafe {
        const G_FLAG: u32 = 0x01682f60;
        const G_CACHE_A: u32 = 0x01682f5c;
        const G_CACHE_B: u32 = 0x01682f64;
        const G_MODE: u32 = 0x011f70cc;
        const G_COUNT: u32 = 0x00eaaf74;
        const G_BASE: u32 = 0x01040100;
        const FIXED_OBJ: u32 = 0x01231800;
        const FIXED_REC: u32 = 0x01288510;
        const ADDR_2A: u32 = 0x00eb9ae8;
        const ADDR_2B: u32 = 0x00eb9b30;
        const ADDR_4A: u32 = 0x00eb9b10;
        const ADDR_4B: u32 = 0x00eb9b54;

        #[inline(always)]
        unsafe fn rd32(x: u32) -> u32 {
            unsafe { (x as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(x: u32, v: u32) {
            unsafe { (x as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(x: u32) -> u8 {
            unsafe { (x as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn g(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gw(va: u32, v: u32) {
            unsafe { wr32(lf_checker_rt::relocated(va), v) }
        }

        // Stride-0x80 pair scan: 0x1f4 slots, each checked at -4 then +0.
        unsafe fn scan_pair(base: u32) -> u32 {
            unsafe {
                let mut i = 0u32;
                while i < 0x1f4 {
                    let e = base.wrapping_add(i.wrapping_mul(0x80));
                    if rd32(e.wrapping_sub(4)) == 0 {
                        return e.wrapping_sub(4);
                    }
                    if rd32(e) == 0 {
                        return e;
                    }
                    i += 1;
                }
                0
            }
        }
        // Linear scan: offsets below `limit`, hit address or 0.
        unsafe fn scan_linear(base: u32, stride: u32, limit: u32) -> u32 {
            unsafe {
                let mut off = 0u32;
                while off < limit {
                    let e = base.wrapping_add(off);
                    if rd32(e) == 0 {
                        return e;
                    }
                    off = off.wrapping_add(stride);
                }
                0
            }
        }

        if rd8(this + 1) == 0 {
            return 0;
        }
        let count = rd32(this + 0x14) as i32;
        let mut ha = 0u32;
        let mut hb = 0u32;
        if count >= 0 {
            let obj: u32 = lf_checker_rt::callee_cdecl!(1, u32, 1u32, count as u32);
            if obj == 0 {
                return 0;
            }
            let k = (rd32(obj + 0x28) >> 6) & 0x0f;
            if k == 3 {
                ha = obj;
            } else if k == 2 {
                hb = obj;
            }
        } else if count != -2 {
            return 0;
        }
        let sel = rd32(this + 0x18);
        let mut esi: u32;
        match sel {
            0x0a | 0x0c => {
                esi = scan_pair(lf_checker_rt::relocated(0x0122031c));
                // Resolve-and-report round A.
                let mut flag = g(G_FLAG);
                let r1 = if flag & 1 == 0 {
                    flag |= 1;
                    gw(G_FLAG, flag);
                    let r: u32 =
                        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(ADDR_2A), 0u32);
                    flag = g(G_FLAG);
                    gw(G_CACHE_A, r);
                    r
                } else {
                    g(G_CACHE_A)
                };
                if rd32(this + 0x0c) == r1 {
                    let m = g(G_MODE).wrapping_sub(2);
                    if m <= 2 {
                        let mut out = [0u32; 4];
                        lf_checker_rt::callee_thiscall!(3, u32, out.as_mut_ptr() as u32);
                        lf_checker_rt::callee_thiscall!(
                            4,
                            u32,
                            lf_checker_rt::relocated(FIXED_OBJ),
                            lf_checker_rt::relocated(ADDR_4A),
                            out.as_mut_ptr() as u32,
                            0xffff_ffff,
                            0u32,
                            0u32
                        );
                    }
                }
                // Round B; flag low byte carries round A's bit 0.
                let r2 = if (flag as u8) & 2 == 0 {
                    flag |= 2;
                    gw(G_FLAG, flag);
                    let r: u32 =
                        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(ADDR_2B), 0u32);
                    gw(G_CACHE_B, r);
                    r
                } else {
                    g(G_CACHE_B)
                };
                if rd32(this + 0x0c) == r2 {
                    let m = g(G_MODE).wrapping_sub(2);
                    if m <= 2 {
                        let mut out = [0u32; 4];
                        lf_checker_rt::callee_thiscall!(3, u32, out.as_mut_ptr() as u32);
                        lf_checker_rt::callee_thiscall!(
                            4,
                            u32,
                            lf_checker_rt::relocated(FIXED_OBJ),
                            lf_checker_rt::relocated(ADDR_4B),
                            out.as_mut_ptr() as u32,
                            0xffff_ffff,
                            0u32,
                            0u32
                        );
                    }
                }
            }
            0x10 => esi = scan_pair(lf_checker_rt::relocated(0x0122032c)),
            0x11 => esi = scan_pair(lf_checker_rt::relocated(0x01220324)),
            0x15 => esi = scan_linear(lf_checker_rt::relocated(0x01660370), 0x6c, 0x10e0),
            0x2e | 0x2f => {
                let n = g(G_COUNT);
                if n == 0 {
                    esi = 0;
                } else {
                    let b = g(G_BASE);
                    let mut i = 0u32;
                    let mut found = 0u32;
                    while i < n {
                        let e = b.wrapping_add(0x18).wrapping_add(i.wrapping_mul(0x20));
                        if rd32(e) == 0 {
                            found = e;
                            break;
                        }
                        i += 1;
                    }
                    esi = found;
                }
            }
            0x33 => {
                if ha == 0 {
                    return 0;
                }
                esi = ha.wrapping_add(0x3dc);
            }
            0x38 => {
                if ha == 0 {
                    return 0;
                }
                esi = ha.wrapping_add(0x558);
            }
            0x48 => {
                if ha == 0 {
                    return 0;
                }
                esi = ha.wrapping_add(0x55c);
            }
            0x4c => {
                if ha == 0 {
                    return 0;
                }
                esi = ha.wrapping_add(0x3cc);
            }
            0x4d => {
                if ha == 0 {
                    return 0;
                }
                esi = ha.wrapping_add(0x4f4);
            }
            0x4e => {
                if ha == 0 {
                    return 0;
                }
                esi = ha.wrapping_add(0x4e4);
            }
            0x4f => {
                if ha == 0 {
                    return 0;
                }
                esi = ha.wrapping_add(0x3d4);
            }
            0x55 => {
                if ha == 0 {
                    return 0;
                }
                esi = ha.wrapping_add(0x4f0);
            }
            0x58 => {
                if ha == 0 {
                    return 0;
                }
                esi = ha.wrapping_add(0x3d8);
            }
            0x5f => {
                if ha == 0 {
                    return 0;
                }
                esi = ha.wrapping_add(0x3e0);
            }
            0x66 | 0x68 => esi = scan_linear(lf_checker_rt::relocated(0x0169e7e8), 4, 24),
            0x67 => esi = scan_linear(lf_checker_rt::relocated(0x0169e800), 4, 24),
            0x6c | 0x6d => esi = lf_checker_rt::relocated(FIXED_REC),
            0x70 => esi = scan_linear(lf_checker_rt::relocated(0x0138a634), 0x120, 0x9000),
            0x8d => {
                if hb == 0 {
                    return 0;
                }
                esi = hb.wrapping_add(0x7f4);
            }
            0x95 => {
                if hb == 0 {
                    return 0;
                }
                esi = hb.wrapping_add(0xb7c);
            }
            0x98 => {
                if hb == 0 {
                    return 0;
                }
                esi = hb.wrapping_add(0xbb8);
            }
            0xaa | 0xab => {
                let b0 = lf_checker_rt::relocated(0x012924d0);
                let mut i = 0u32;
                let mut found = 0u32;
                while i < 0x20 {
                    let e = b0.wrapping_add(i.wrapping_mul(0xd0));
                    if rd32(e) == 0 {
                        found = e;
                        break;
                    }
                    if rd32(e.wrapping_add(4)) == 0 {
                        found = e.wrapping_add(4);
                        break;
                    }
                    i += 1;
                }
                esi = found;
            }
            _ => return 0,
        }
        let head = rd32(esi);
        if head != 0 {
            lf_checker_rt::callee_thiscall!(5, u32, head, 0u32);
            wr32(esi, 0);
        }
        esi
    }
});
