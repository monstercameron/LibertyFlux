// original: 0x005C8390 ped_task_group_reset (proposed)

/// Reset one ped task group, or re-arm every group when the reset flag is clear.
///
/// The function takes no arguments and returns 1 in `al` on the reset path and
/// on the early exit, or 0 after the re-arm scan (cdecl, no stack arguments).
///
/// Layout. A global object table maps small indices to task-group objects
/// (`TBL`, dword per index). A global index array holds one dword per slot,
/// spaced `ARR_STRIDE` bytes apart; four named slots (`IDX_A`..`IDX_D`) sit at
/// array positions, and `VAL44` is a dword value copied into a field, not an
/// index. Objects carry dword fields at `+0x3c`, `+0x40`, `+0x44` and `+0x5c`.
/// Two byte flags gate the paths (`FLAG1` at entry, `FLAG2` inside the per-slot
/// blocks); `MODE` is a dword mode, `EXT_IDX`/`EXT_TBL` an external
/// index/table pair probed for null, and `GATE` a dword gate for the re-arm scan.
///
/// Reset path (`FLAG1` set): clears `FLAG1`, notifies callee 1 with
/// `(-1, 0)`, zeroes slot B's `+0x5c` field and copies `VAL44` into its
/// `+0x44`, then sweeps the 36 array slots calling callee 2 with `(0, 0)` and
/// callee 3 with `(0, 1)` on each. Four per-slot blocks follow. Blocks A and B
/// ask callee 4 first and skip when it answers nonzero; otherwise they store
/// `0x190`/`0x1388` into `+0x3c`/`+0x40` and run the mode gate: when `FLAG2`
/// is clear, the external index is -1, its table word is null, or the mode is
/// outside 8..15, a reduced gate maps mode `<= 0` or `> 7` to a `(4, 0)`
/// follow-up call and modes 1..7 to a `(2, 0)` one; modes 8..14 go straight to
/// the `(2, 0)` follow-up; mode 15 additionally consults callee 5 (nonzero
/// keeps `(2, 0)`, zero falls back to `(4, 0)`). Each follow-up is a callee-2
/// call plus a callee-3 `(1, 0)` call; slot B's `(2, 0)` follow-up additionally
/// repeats both calls on slot A. Blocks C and D store `0x320`/`0x5dc` and make
/// the `(4, 0)` + `(1, 0)` calls when callee 4 answers zero. Returns 1.
///
/// Re-arm path (`FLAG1` clear): returns 1 at once when `GATE` is zero, else
/// sweeps 65 array slots, and for each slot whose callee-4 answer is nonzero
/// stores `0x190` into `+0x3c` and calls callee 2 with `(3, 0)` and callee 3
/// with `(0, 0)`. A final callee-4 probe on slot B stores 0 into `+0x5c` and
/// copies `VAL44` into `+0x44` when nonzero. Returns 0.
///
/// Edge cases. The sweep bounds compare signed (`jl`); the per-slot gate reads
/// the `FLAG2` comparison flags across several moves, so a clear `FLAG2`
/// selects the reduced gate whatever the mode holds; the external table is
/// only dereferenced when the external index is not -1.
///
/// Original: 0x005C8390 (cdecl, no arguments, `al` return).
lf_checker_rt::export!(cdecl, rw_005C8390() -> u32 {
    unsafe {
        const FLAG1: u32 = 0x0118E90A;
        const TBL: u32 = 0x0118E7F8;
        const IDX_A: u32 = 0x0118ED18;
        const IDX_B: u32 = 0x0118ED34;
        const VAL44: u32 = 0x0118ED4C;
        const IDX_C: u32 = 0x0118ED50;
        const IDX_D: u32 = 0x0118ED6C;
        const ARR_LO1: u32 = 0x0118EC54;
        const ARR_LO2: u32 = 0x0118E928;
        const ARR_HI: u32 = 0x0118F044;
        const ARR_STRIDE: u32 = 0x1C;
        const EXT_IDX: u32 = 0x01036F14;
        const EXT_TBL: u32 = 0x011A8808;
        const MODE: u32 = 0x017F5EA4;
        const FLAG2: u32 = 0x017F5EB0;
        const GATE: u32 = 0x011D6FA0;
        const FIXED_THIS: u32 = 0x01982018;
        const F_3C: u32 = 0x3C;
        const F_40: u32 = 0x40;
        const F_44: u32 = 0x44;
        const F_5C: u32 = 0x5C;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (lf_checker_rt::relocated(a) as *const u8).read() }
        }
        /// Object for a named index slot: index word, then the table lookup.
        #[inline(always)]
        unsafe fn obj_of(idx_global: u32) -> u32 {
            unsafe {
                let i = rd32(idx_global);
                (lf_checker_rt::relocated(TBL).wrapping_add(i.wrapping_mul(4)) as *const u32)
                    .read_unaligned()
            }
        }
        /// Object for an array slot at relocated address `p`.
        #[inline(always)]
        unsafe fn obj_at(p: u32) -> u32 {
            unsafe {
                let i = (p as *const u32).read_unaligned();
                (lf_checker_rt::relocated(TBL).wrapping_add(i.wrapping_mul(4)) as *const u32)
                    .read_unaligned()
            }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn c2(o: u32, a: u32, b: u32) {
            unsafe { lf_checker_rt::callee_thiscall!(2, u32, o, a, b) };
        }
        #[inline(always)]
        unsafe fn c3(o: u32, a: u32, b: u32) {
            unsafe { lf_checker_rt::callee_thiscall!(3, u32, o, a, b) };
        }
        #[inline(always)]
        unsafe fn c4(o: u32) -> u8 {
            unsafe { lf_checker_rt::callee_thiscall!(4, u32, o) as u8 }
        }
        /// Slot A follow-up: callee 2 with (`which`, 0), then callee 3 (1, 0).
        #[inline(always)]
        unsafe fn path_a(which: u32) {
            unsafe {
                c2(obj_of(IDX_A), which, 0);
                c3(obj_of(IDX_A), 1, 0);
            }
        }
        /// Slot B follow-up: same pair on B, and for the `(2, 0)` variant the
        /// same two calls again on slot A.
        #[inline(always)]
        unsafe fn path_b(which: u32) {
            unsafe {
                if which == 4 {
                    c2(obj_of(IDX_B), 4, 0);
                    c3(obj_of(IDX_B), 1, 0);
                } else {
                    c2(obj_of(IDX_B), 2, 0);
                    c3(obj_of(IDX_B), 1, 0);
                    c2(obj_of(IDX_A), 2, 0);
                    c3(obj_of(IDX_A), 1, 0);
                }
            }
        }
        /// The mode gate shared by blocks A and B (see the doc comment).
        #[inline(always)]
        unsafe fn gate_block(is_a: bool) {
            unsafe {
                let mode = rd32(MODE) as i32;
                // The original compares FLAG2, then loads the mode and jumps on
                // the still-set flags, so a clear FLAG2 selects the reduced
                // gate on every mode.
                let mut use_gate = rd8(FLAG2) == 0;
                if !use_gate {
                    let e = rd32(EXT_IDX);
                    if e == 0xFFFF_FFFF {
                        use_gate = true;
                    } else if (lf_checker_rt::relocated(EXT_TBL)
                        .wrapping_add(e.wrapping_mul(4))
                        as *const u32)
                        .read_unaligned()
                        == 0
                    {
                        use_gate = true;
                    } else if mode < 8 {
                        use_gate = true;
                    } else if mode > 0xF {
                        use_gate = true;
                    } else if mode != 0xF {
                        if is_a {
                            path_a(2);
                        } else {
                            path_b(2);
                        }
                        return;
                    } else if lf_checker_rt::callee_thiscall!(
                        5,
                        u32,
                        lf_checker_rt::relocated(FIXED_THIS)
                    ) as u8
                        != 0
                    {
                        if is_a {
                            path_a(2);
                        } else {
                            path_b(2);
                        }
                        return;
                    } else if is_a {
                        path_a(4);
                        return;
                    } else {
                        path_b(4);
                        return;
                    }
                }
                if mode <= 0 || mode > 7 {
                    if is_a {
                        path_a(4);
                    } else {
                        path_b(4);
                    }
                } else if is_a {
                    path_a(2);
                } else {
                    path_b(2);
                }
            }
        }

        if rd8(FLAG1) != 0 {
            (lf_checker_rt::relocated(FLAG1) as *mut u8).write(0);
            lf_checker_rt::callee_cdecl!(1, u32, 0xFFFF_FFFFu32, 0u32);
            wr32(obj_of(IDX_B).wrapping_add(F_5C), 0);
            wr32(obj_of(IDX_B).wrapping_add(F_44), rd32(VAL44));
            let mut p = lf_checker_rt::relocated(ARR_LO1);
            let hi = lf_checker_rt::relocated(ARR_HI);
            while (p as i32) < (hi as i32) {
                c2(obj_at(p), 0, 0);
                c3(obj_at(p), 0, 1);
                p = p.wrapping_add(ARR_STRIDE);
            }
            if c4(obj_of(IDX_A)) == 0 {
                wr32(obj_of(IDX_A).wrapping_add(F_3C), 0x190);
                wr32(obj_of(IDX_A).wrapping_add(F_40), 0x1388);
                gate_block(true);
            }
            if c4(obj_of(IDX_B)) == 0 {
                wr32(obj_of(IDX_B).wrapping_add(F_3C), 0x190);
                wr32(obj_of(IDX_B).wrapping_add(F_40), 0x1388);
                gate_block(false);
            }
            if c4(obj_of(IDX_C)) == 0 {
                wr32(obj_of(IDX_C).wrapping_add(F_3C), 0x320);
                wr32(obj_of(IDX_C).wrapping_add(F_40), 0x5DC);
                c2(obj_of(IDX_C), 4, 0);
                c3(obj_of(IDX_C), 1, 0);
            }
            if c4(obj_of(IDX_D)) == 0 {
                wr32(obj_of(IDX_D).wrapping_add(F_3C), 0x320);
                wr32(obj_of(IDX_D).wrapping_add(F_40), 0x5DC);
                c2(obj_of(IDX_D), 4, 0);
                c3(obj_of(IDX_D), 1, 0);
            }
            1
        } else if rd32(GATE) == 0 {
            1
        } else {
            let mut p = lf_checker_rt::relocated(ARR_LO2);
            let hi = lf_checker_rt::relocated(ARR_HI);
            while (p as i32) < (hi as i32) {
                let o = obj_at(p);
                if c4(o) != 0 {
                    wr32(o.wrapping_add(F_3C), 0x190);
                    c2(o, 3, 0);
                    c3(o, 0, 0);
                }
                p = p.wrapping_add(ARR_STRIDE);
            }
            if c4(obj_of(IDX_B)) != 0 {
                wr32(obj_of(IDX_B).wrapping_add(F_5C), 0);
                wr32(obj_of(IDX_B).wrapping_add(F_44), rd32(VAL44));
            }
            0
        }
    }
});
