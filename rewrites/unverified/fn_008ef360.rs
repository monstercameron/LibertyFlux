// original: 0x008EF360 grid_occupancy_rebuild (proposed)

/// Rebuild the two occupancy bit-grids and reconcile the resident tables.
///
/// `this` is the owner object (tables at `+0x804`, tuning window around
/// `+0x1a8c`, flag byte at `+0x1ac0`, word at `+0x1c08`); the single stack
/// argument contributes only its low byte. When that byte is zero and the
/// xor of the two state globals has no bits outside the low nine set, the
/// xor itself is returned without doing anything.
///
/// Otherwise both 64-byte grids (eight 8-byte rows each) are zeroed, the
/// current ped is fetched (callee 1, always asked with 0) and, when one is
/// present, a probe through the locator callee (callee 2, three scripted
/// words into a frame buffer) feeds the range callee (callee 3) with the
/// radius 225.0. A set flag byte sends a second range query (callee 4)
/// over a zeroed buffer with radius 7000.0.
///
/// The group table (global) is then walked from its last row to its first
/// (SIGNED down-count with an early-out on a zero count): a row whose tag
/// byte has the high bit set, whose base pointer is null, or whose kind
/// word at `+0x1304` is 2, 3, 4 or 5 is skipped; any other row either
/// issues its range query (callee 5, radius 300.0) directly when its mode
/// byte at `+0x10b8` is 2, or first asks the two gate callees (callees 6
/// and 7, the loop index in ECX) when its permit bit at `+0xf1f` allows.
///
/// Next, three cells are examined: when the flag byte for cell `i` is set,
/// four coordinates around the cell cursor are each mapped to a grid
/// index by the quantize callee (callees 8-11, one per coordinate). With
/// the indices named A (low window), B (low edge), C (high edge) and D
/// (first row), all compared SIGNED: rows `D` through `A` are painted
/// when `D <= A`, and each painted row sets `C - B + 1` bytes to 1 at the
/// same relative span in both grids (nothing is painted when `B > C`).
///
/// Finally every grid bit is reconciled with the resident pointer at the
/// matching table slot: a set bit with a null pointer re-requests the
/// cell (callee 12) with priority 6 when the second grid agrees and 10
/// when it does not (the second shape never fires: both grids always
/// carry identical fills, so the argument is always 6), while a clear bit
/// with a resident pointer releases it (callee 13). The function returns
/// the table cursor just past the last reconciled slot.
///
/// Original: 0x008EF360 (thiscall, one stack word, full EAX return).
lf_checker_rt::export!(thiscall, rw_008EF360(this: u32, arg: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x804;
        const GRID_A: u32 = 0x1176db8;
        const GRID_B: u32 = 0x1176df8;
        const GRID_ROWS: u32 = 8;
        const ROW_STRIDE: u32 = 8;
        const FLAG_BASE: u32 = 0x1a8c;
        const CURSOR_BASE: u32 = 0x1aa8;
        const CELLS: u32 = 3;
        const RADIUS_PROBE: u32 = 0x43e10000;
        const RADIUS_WIDE: u32 = 0x45dac000;
        const RADIUS_ROW: u32 = 0x43960000;
        const CALLEE_PED: u32 = 1;
        const CALLEE_PROBE: u32 = 2;
        const CALLEE_RANGE: u32 = 3;
        const CALLEE_RANGE2: u32 = 4;
        const CALLEE_RANGE3: u32 = 5;
        const CALLEE_GATE: u32 = 6;
        const CALLEE_GATE2: u32 = 7;
        const CALLEE_QA: u32 = 8;
        const CALLEE_QB: u32 = 9;
        const CALLEE_QC: u32 = 10;
        const CALLEE_QD: u32 = 11;
        const CALLEE_REQ: u32 = 12;
        const CALLEE_REL: u32 = 13;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn paint(grid_file_va: u32, off: u32, n: u32) {
            unsafe {
                let base = lf_checker_rt::relocated(grid_file_va);
                let mut k = 0u32;
                while k < n {
                    ((base.wrapping_add(off).wrapping_add(k)) as *mut u8).write(1);
                    k += 1;
                }
            }
        }

        if (arg as u8) == 0 {
            let x = *lf_checker_rt::global::<u32>(0x1173598)
                ^ *lf_checker_rt::global::<u32>(0x1173594);
            if x & 0xffff_fe00 == 0 {
                return x;
            }
        }
        let mut r = 0u32;
        while r < GRID_ROWS {
            ((lf_checker_rt::relocated(GRID_A).wrapping_add(r * ROW_STRIDE)) as *mut u64)
                .write_unaligned(0);
            ((lf_checker_rt::relocated(GRID_B).wrapping_add(r * ROW_STRIDE)) as *mut u64)
                .write_unaligned(0);
            r += 1;
        }
        let ped: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PED, u32, 0);
        if ped != 0 {
            let mut buf = [0u32; 3];
            let loc: u32 =
                lf_checker_rt::callee_cdecl!(CALLEE_PROBE, u32, buf.as_mut_ptr() as u32);
            lf_checker_rt::callee_thiscall!(CALLEE_RANGE, u32, this, loc, RADIUS_PROBE);
        }
        if ((this.wrapping_add(0x1ac0)) as *const u8).read() != 0 {
            let buf = [0u32; 3];
            lf_checker_rt::callee_thiscall!(
                CALLEE_RANGE2,
                u32,
                this,
                buf.as_ptr() as u32,
                RADIUS_WIDE
            );
        }
        let g = *lf_checker_rt::global::<u32>(0x12e22a4);
        let count = rd32(g.wrapping_add(8));
        if count != 0 {
            let row_base = rd32(g);
            let tag_base = rd32(g.wrapping_add(4));
            let stride = rd32(g.wrapping_add(0xc));
            let mut i = count;
            loop {
                i = i.wrapping_sub(1);
                if ((tag_base.wrapping_add(i)) as *const u8).read() & 0x80 == 0 {
                    let row = row_base.wrapping_add(stride.wrapping_mul(i));
                    if row != 0 {
                        let kind = rd32(row.wrapping_add(0x1304));
                        if kind != 4 && kind != 5 && kind != 2 && kind != 3 {
                            if ((row.wrapping_add(0x10b8)) as *const u8).read() == 2 {
                                let a = rd32(row.wrapping_add(0x20)).wrapping_add(0x30);
                                lf_checker_rt::callee_thiscall!(
                                    CALLEE_RANGE3, u32, this, a, RADIUS_ROW
                                );
                            } else if ((row.wrapping_add(0xf1f)) as *const u8).read() & 0x40
                                != 0
                            {
                                let p: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GATE, u32, i);
                                let ok: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GATE2, u32, p);
                                if ok != 0 {
                                    let a = rd32(row.wrapping_add(0x20)).wrapping_add(0x30);
                                    lf_checker_rt::callee_thiscall!(
                                        CALLEE_RANGE3, u32, this, a, RADIUS_ROW
                                    );
                                }
                            }
                        }
                    }
                }
                if i == 0 {
                    break;
                }
            }
        }
        let mut i = 0u32;
        while i < CELLS {
            if ((this.wrapping_add(FLAG_BASE).wrapping_add(i)) as *const u8).read() != 0 {
                let c = this.wrapping_add(CURSOR_BASE).wrapping_add(i * 4);
                let fa = rd32(c.wrapping_sub(0xc));
                let fb = rd32(c);
                let fc = rd32(c.wrapping_add(0xc));
                let fd = rd32(c.wrapping_sub(0x18));
                let ia = lf_checker_rt::callee_stdcall!(CALLEE_QA, u32, fa) as i32;
                let ib = lf_checker_rt::callee_stdcall!(CALLEE_QB, u32, fb) as i32;
                let ic = lf_checker_rt::callee_stdcall!(CALLEE_QC, u32, fc) as i32;
                let id = lf_checker_rt::callee_stdcall!(CALLEE_QD, u32, fd) as i32;
                // All SIGNED: rows D..=A paint while D <= A, each row
                // painting C-B+1 bytes, and nothing paints while B > C.
                if id <= ia && ib <= ic {
                    let n = (ic.wrapping_sub(ib).wrapping_add(1)) as u32;
                    let mut d = id;
                    while d <= ia {
                        let off = (ib.wrapping_add(d.wrapping_mul(8))) as u32;
                        paint(GRID_B, off, n);
                        paint(GRID_A, off, n);
                        d = d.wrapping_add(1);
                    }
                }
            }
            i += 1;
        }
        let mut d2 = 0u32;
        while d2 < GRID_ROWS {
            let oc = this.wrapping_add(TABLE_BASE).wrapping_add(d2 * 4);
            let mut f = 0u32;
            while f < GRID_ROWS {
                let bit = f.wrapping_add(d2.wrapping_mul(8));
                let s = d2.wrapping_add(f.wrapping_mul(8));
                let slot = rd32(oc.wrapping_add(f.wrapping_mul(0x20)));
                let ga = (lf_checker_rt::relocated(GRID_A).wrapping_add(bit)) as *const u8;
                if ga.read() != 0 {
                    if slot == 0 {
                        let gb =
                            (lf_checker_rt::relocated(GRID_B).wrapping_add(bit)) as *const u8;
                        let prio = if gb.read() != 0 { 6u32 } else { 10u32 };
                        lf_checker_rt::callee_cdecl!(
                            CALLEE_REQ,
                            u32,
                            s,
                            rd32(this.wrapping_add(0x1c08)),
                            prio
                        );
                    }
                } else if slot != 0 {
                    lf_checker_rt::callee_cdecl!(
                        CALLEE_REL,
                        u32,
                        s,
                        rd32(this.wrapping_add(0x1c08))
                    );
                }
                f += 1;
            }
            d2 += 1;
        }
        this.wrapping_add(0x824)
    }
});
