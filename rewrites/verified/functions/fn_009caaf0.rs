// original: 0x009CAAF0 row_table_sweep_update (proposed)

/// Sweep the row table, refreshing every row whose time window is current.
///
/// `time` is the current tick as a float. The row count comes from the count
/// global and each row pointer from the row table; the function returns the
/// count, or zero when it is not positive. Rows with a zero flag word take
/// the direct path, rows with a nonzero flag the matched path (taken only
/// when the row's owner equals the index global).
///
/// Direct path: the row is skipped unless its gate byte (selected by the
/// index global) is set and the float sum of the two window globals falls
/// inside the row's integer window. The row's eight work floats are then
/// either copied straight from the row (marker set) or computed through the
/// pair callee and two out callee calls (marker clear), and handed to the
/// build callee together with two frame buffers; a resolved handle is stored
/// back into the row, or, when the row already carries one, the refresh
/// callee runs instead. Matched path: same shape with a wider window (upper
/// bound one thousand past the lower), six work floats and a byte-sized
/// handle. A row whose window has passed has its handle released (and, on
/// the direct path, the release callee runs first).
///
/// After a fresh handle is stored, up to four lanes fire: each set lane byte
/// calls the lane callee with the lane's float, its address and the handle.
/// Float operation order is the original's, pinned through `black_box`
/// helpers; the window tests are inverted comparisons, matching `jb`/`jbe`
/// after `comiss` exactly, including NaN (which always misses the window).
///
/// Original: 0x009CAAF0 (cdecl, one float stack word).
lf_checker_rt::export!(cdecl, rw_009CAAF0(time: f32) -> u32 {
    unsafe {
        const IDX_GLOB: u32 = 0x1294730;
        const ROW_TAB: u32 = 0x12948F0;
        const COUNT_GLOB: u32 = 0x1295728;
        const WIN_A: u32 = 0x1295740;
        const WIN_B: u32 = 0x1295744;
        const MID_TAB: u32 = 0x1293E58;
        const DST_TAB: u32 = 0x1293E98;
        const MID_OFF: u32 = 0xFA98;
        const THIS_VAL: u32 = 0x1394D60;
        const PLUS_THOUSAND: u32 = 1000;
        const INNER_N: u32 = 4;
        const INNER_FSTEP: u32 = 4;
        const INNER_BSTEP: u32 = 0x14;
        const LANE_FLOATS: u32 = 0x148;
        const LANE_BYTES: u32 = 0xF8;
        const CAL_PAIR: u32 = 1;
        const CAL_OUT: u32 = 2;
        const CAL_BUILD_D: u32 = 3;
        const CAL_BUILD_B: u32 = 4;
        const CAL_HANDLE_D: u32 = 5;
        const CAL_LANE: u32 = 6;
        const CAL_REFRESH: u32 = 7;
        const CAL_RELEASE: u32 = 8;
        const CAL_HANDLE_B: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let arg_bits = time.to_bits();
        let count = rd32(lf_checker_rt::relocated(COUNT_GLOB));
        if (count as i32) <= 0 {
            return 0;
        }
        let this_v = lf_checker_rt::relocated(THIS_VAL);
        let win_sum = add(
            f32::from_bits(rd32(lf_checker_rt::relocated(WIN_B))),
            f32::from_bits(rd32(lf_checker_rt::relocated(WIN_A))),
        );
        let idx = rd32(lf_checker_rt::relocated(IDX_GLOB));
        let mut i = 0u32;
        loop {
            let row = rd32(lf_checker_rt::relocated(ROW_TAB).wrapping_add(i.wrapping_mul(4)));
            if row != 0 {
                if rd32(row) != 0 {
                    // Matched path.
                    if idx == rd32(row.wrapping_add(4)) {
                        let lo = (rd32(row.wrapping_add(0x58)) as i32) as f32;
                        if win_sum >= lo {
                            let hi = (rd32(row.wrapping_add(0x58)) as i32)
                                .wrapping_add(PLUS_THOUSAND as i32)
                                as f32;
                            if hi > win_sum {
                                let mut blk = [
                                    rd32(row.wrapping_add(0x70)),
                                    rd32(row.wrapping_add(0x74)),
                                    rd32(row.wrapping_add(0x78)),
                                    rd32(row.wrapping_add(0x80)),
                                    rd32(row.wrapping_add(0x84)),
                                    rd32(row.wrapping_add(0x88)),
                                ];
                                let ridx = rd32(row.wrapping_add(0x54));
                                let ecx_var: u32;
                                if (ridx as i32) == -1 {
                                    let mut slot = [0u32; 1];
                                    let r = lf_checker_rt::callee_cdecl!(
                                        CAL_PAIR,
                                        u32,
                                        slot.as_mut_ptr() as u32,
                                        idx
                                    );
                                    blk[0] = add(
                                        f32::from_bits(rd32(r)),
                                        f32::from_bits(rd32(row.wrapping_add(0x70))),
                                    )
                                    .to_bits();
                                    blk[1] = add(
                                        f32::from_bits(rd32(r.wrapping_add(4))),
                                        f32::from_bits(rd32(row.wrapping_add(0x74))),
                                    )
                                    .to_bits();
                                    blk[2] = add(
                                        f32::from_bits(rd32(r.wrapping_add(8))),
                                        f32::from_bits(rd32(row.wrapping_add(0x78))),
                                    )
                                    .to_bits();
                                    ecx_var = 0;
                                } else {
                                    let mid = rd32(
                                        lf_checker_rt::relocated(MID_TAB)
                                            .wrapping_add(idx.wrapping_mul(4)),
                                    );
                                    let dst_i = rd32(
                                        mid.wrapping_add(MID_OFF).wrapping_add(ridx.wrapping_mul(4)),
                                    );
                                    ecx_var = if (dst_i as i32) == -1 {
                                        0
                                    } else {
                                        rd32(
                                            lf_checker_rt::relocated(DST_TAB)
                                                .wrapping_add(dst_i.wrapping_mul(4)),
                                        )
                                    };
                                }
                                if rd32(row.wrapping_add(0x90)) == 0 {
                                    let r40: u32 = lf_checker_rt::callee_thiscall!(
                                        CAL_BUILD_B,
                                        u32,
                                        ecx_var,
                                        row.wrapping_add(0x94),
                                        0,
                                        ecx_var,
                                        blk.as_mut_ptr().wrapping_add(0) as u32,
                                        blk.as_mut_ptr().wrapping_add(3) as u32
                                    );
                                    let h: u32 = lf_checker_rt::callee_thiscall!(
                                        CAL_HANDLE_B,
                                        u32,
                                        this_v,
                                        r40
                                    );
                                    let hb = h & 0xFF;
                                    wr32(row.wrapping_add(0x90), hb);
                                    if hb != 0 {
                                        let mut f = row.wrapping_add(LANE_FLOATS);
                                        let mut b = row.wrapping_add(LANE_BYTES);
                                        let mut k = INNER_N;
                                        while k != 0 {
                                            if rd8(b) != 0 {
                                                let _: u32 = lf_checker_rt::callee_thiscall!(
                                                    CAL_LANE,
                                                    u32,
                                                    this_v,
                                                    hb,
                                                    b,
                                                    rd32(f)
                                                );
                                            }
                                            f = f.wrapping_add(INNER_FSTEP);
                                            b = b.wrapping_add(INNER_BSTEP);
                                            k -= 1;
                                        }
                                    }
                                }
                            } else if rd32(row.wrapping_add(0x90)) != 0 {
                                wr32(row.wrapping_add(0x90), 0);
                            }
                        } else if rd32(row.wrapping_add(0x90)) != 0 {
                            wr32(row.wrapping_add(0x90), 0);
                        }
                    }
                } else {
                    // Direct path.
                    if rd8(row.wrapping_add(idx.wrapping_add(0x44))) != 0 {
                        let lo = (rd32(row.wrapping_add(0x58)) as i32) as f32;
                        let hi = (rd32(row.wrapping_add(0x5C)) as i32) as f32;
                        if win_sum >= lo && hi >= win_sum {
                            let mut blk_a = [0u32; 4];
                            let mut blk_b = [0u32; 4];
                            if rd32(row.wrapping_add(0x60)) != 0xFFFF_FFFF {
                                blk_a[0] = rd32(row.wrapping_add(0x70));
                                blk_a[1] = rd32(row.wrapping_add(0x74));
                                blk_a[2] = rd32(row.wrapping_add(0x78));
                                blk_a[3] = rd32(row.wrapping_add(0x7C));
                                blk_b[0] = rd32(row.wrapping_add(0x80));
                                blk_b[1] = rd32(row.wrapping_add(0x84));
                                blk_b[2] = rd32(row.wrapping_add(0x88));
                                blk_b[3] = rd32(row.wrapping_add(0x8C));
                            } else {
                                let mut slot = [0u32; 1];
                                let r = lf_checker_rt::callee_cdecl!(
                                    CAL_PAIR,
                                    u32,
                                    slot.as_mut_ptr() as u32,
                                    idx
                                );
                                let t0 = add(
                                    f32::from_bits(rd32(row.wrapping_add(0x70))),
                                    f32::from_bits(rd32(r)),
                                );
                                let t1 = add(
                                    f32::from_bits(rd32(row.wrapping_add(0x74))),
                                    f32::from_bits(rd32(r.wrapping_add(4))),
                                );
                                let t2 = add(
                                    f32::from_bits(rd32(row.wrapping_add(0x78))),
                                    f32::from_bits(rd32(r.wrapping_add(8))),
                                );
                                let elem = rd32(
                                    row.wrapping_add(8).wrapping_add(idx.wrapping_mul(4)),
                                );
                                let mut obuf = [0u32; 4];
                                let rc1 = lf_checker_rt::callee_thiscall!(
                                    CAL_OUT,
                                    u32,
                                    elem,
                                    obuf.as_mut_ptr() as u32,
                                    0,
                                    arg_bits
                                );
                                let u0 = add(
                                    f32::from_bits(rd32(rc1.wrapping_add(8))),
                                    t2,
                                );
                                let u1 = add(f32::from_bits(rd32(rc1)), t0);
                                let u2 = add(
                                    f32::from_bits(rd32(rc1.wrapping_add(4))),
                                    t1,
                                );
                                blk_a[0] = u1.to_bits();
                                blk_a[1] = u2.to_bits();
                                blk_a[2] = u0.to_bits();
                                blk_a[3] = obuf[3];
                                let mut obuf2 = [0u32; 4];
                                let rc2 = lf_checker_rt::callee_thiscall!(
                                    CAL_OUT,
                                    u32,
                                    elem,
                                    obuf2.as_mut_ptr() as u32,
                                    0x8F,
                                    arg_bits
                                );
                                blk_b[0] = rd32(rc2);
                                blk_b[1] = rd32(rc2.wrapping_add(4));
                                blk_b[2] = rd32(rc2.wrapping_add(8));
                                blk_b[3] = rd32(rc2.wrapping_add(12));
                            }
                            let h = rd32(row.wrapping_add(0x90));
                            if h == 0 {
                                let ridx = rd32(row.wrapping_add(0x54));
                                let mut edx_var = 0u32;
                                if (ridx as i32) != -1 {
                                    let mid = rd32(
                                        lf_checker_rt::relocated(MID_TAB)
                                            .wrapping_add(idx.wrapping_mul(4)),
                                    );
                                    let dst_i = rd32(
                                        mid.wrapping_add(MID_OFF).wrapping_add(ridx.wrapping_mul(4)),
                                    );
                                    if (dst_i as i32) != -1 {
                                        edx_var = rd32(
                                            lf_checker_rt::relocated(DST_TAB)
                                                .wrapping_add(dst_i.wrapping_mul(4)),
                                        );
                                    }
                                }
                                let r40: u32 = lf_checker_rt::callee_thiscall!(
                                    CAL_BUILD_D,
                                    u32,
                                    ridx,
                                    row.wrapping_add(0x94),
                                    0,
                                    edx_var,
                                    blk_a.as_mut_ptr() as u32,
                                    blk_b.as_mut_ptr() as u32
                                );
                                let h2: u32 = lf_checker_rt::callee_thiscall!(
                                    CAL_HANDLE_D,
                                    u32,
                                    this_v,
                                    r40
                                );
                                wr32(row.wrapping_add(0x90), h2);
                                if h2 != 0 {
                                    let mut f = row.wrapping_add(LANE_FLOATS);
                                    let mut b = row.wrapping_add(LANE_BYTES);
                                    let mut k = INNER_N;
                                    while k != 0 {
                                        if rd8(b) != 0 {
                                            let _: u32 = lf_checker_rt::callee_thiscall!(
                                                CAL_LANE,
                                                u32,
                                                this_v,
                                                h2,
                                                b,
                                                rd32(f)
                                            );
                                        }
                                        f = f.wrapping_add(INNER_FSTEP);
                                        b = b.wrapping_add(INNER_BSTEP);
                                        k -= 1;
                                    }
                                }
                            } else {
                                let _: u32 = lf_checker_rt::callee_thiscall!(
                                    CAL_REFRESH,
                                    u32,
                                    this_v,
                                    h,
                                    blk_a.as_mut_ptr() as u32,
                                    blk_b.as_mut_ptr() as u32
                                );
                            }
                        } else {
                            let h = rd32(row.wrapping_add(0x90));
                            if h != 0 {
                                let _: u32 = lf_checker_rt::callee_thiscall!(
                                    CAL_RELEASE,
                                    u32,
                                    this_v,
                                    h
                                );
                                wr32(row.wrapping_add(0x90), 0);
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
            if (i as i32) >= (count as i32) {
                break;
            }
        }
        count
    }
});
