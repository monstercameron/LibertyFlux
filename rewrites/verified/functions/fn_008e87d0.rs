// original: 0x008e87d0 emit_min_scan
/// Scan a neighbourhood for the minimum edge distance, then probe it.
///
/// `arg0` receives -1 up front and is returned on every path. `arg1` points
/// at two floats that are converted to indexes 0..7 through a clamping
/// helper; four scale values fetched for those indexes combine with the two
/// floats into four edge distances whose running minimum is kept in the
/// incoming float slot. A 15-word probe call publishes a gate value; when
/// the gate does not exceed the minimum the function returns. Otherwise a
/// five-round nest runs: each round scans up to four bounded index ranges
/// (every range guarded, so wild helper answers only skip blocks) issuing
/// the same probe with a computed 16-bit index, then grows the minimum by a
/// fixed step and leaves early once it passes the gate. `this` and the
/// trailing eleven words are only forwarded to the probe. Returns `arg0`.
export!(thiscall, rw_008e87d0(
    this_: u32,
    arg0: *mut u8,
    arg1: *const u8,
    entry_level: f32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
    a9: u32,
    a10: u32,
    a11: u32,
    a12: u32,
    a13: u32,
) -> u32 {
    const STEP: f32 = 750.0;
    unsafe {
        (arg0 as *mut u32).write_unaligned(0xFFFF_FFFF);
        let first = (arg1 as *const f32).read_unaligned();
        let second =
            ((arg1 as u32).wrapping_add(4) as *const f32).read_unaligned();
        let mut gate: f32 = entry_level;
        let gate_word = &mut gate as *mut f32 as u32;
        let index_a = lf_checker_rt::callee_stdcall!(1, u32, first.to_bits());
        let index_b = lf_checker_rt::callee_stdcall!(4, u32, second.to_bits());
        let scale_a = lf_checker_rt::callee_stdcall!(2, f32, index_a);
        let mut best = first - scale_a;
        let scale_a_next =
            lf_checker_rt::callee_stdcall!(5, f32, index_a.wrapping_add(1));
        let candidate = scale_a_next - first;
        if !(candidate > best) {
            best = candidate;
        }
        let scale_b = lf_checker_rt::callee_stdcall!(6, f32, index_b);
        let candidate = second - scale_b;
        if !(candidate > best) {
            best = candidate;
        }
        let scale_b_next =
            lf_checker_rt::callee_stdcall!(7, f32, index_b.wrapping_add(1));
        let candidate = scale_b_next - second;
        if !(candidate > best) {
            best = candidate;
        }
        let combo = index_b
            .wrapping_mul(8)
            .wrapping_add(index_a)
            & 0xFFFF;
        lf_checker_rt::callee_thiscall!(
            3, u32, this_, arg0 as u32, combo, arg1 as u32, gate_word,
            a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13
        );
        if !(gate > best) {
            return arg0 as u32;
        }
        let mut round: i32 = 1;
        let mut low = (index_b as i32).wrapping_sub(1);
        let diff = (index_b as i32).wrapping_sub(index_a as i32);
        let mut base = index_a as i32;
        let mut top = (index_a as i32).wrapping_add(1);
        loop {
            if (base.wrapping_sub(1) as u32) <= 7 {
                // The range end uses the still-loaded upper bound; the base
                // is reloaded only for the index below.
                let end = diff.wrapping_add(top);
                let mut k = low;
                if k <= end {
                    loop {
                        if (k as u32) <= 7 {
                            let idx = base
                                .wrapping_sub(1)
                                .wrapping_add(k.wrapping_mul(8))
                                as u16 as u32;
                            lf_checker_rt::callee_thiscall!(
                                3, u32, this_, arg0 as u32, idx,
                                arg1 as u32, gate_word,
                                a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13
                            );
                        }
                        k = k.wrapping_add(1);
                        if k > end {
                            break;
                        }
                    }
                }
            }
            if (top as u32) <= 7 {
                let end = diff.wrapping_add(top);
                let mut k = low;
                if k <= end {
                    loop {
                        if (k as u32) <= 7 {
                            let idx = top.wrapping_add(k.wrapping_mul(8))
                                as u16 as u32;
                            lf_checker_rt::callee_thiscall!(
                                3, u32, this_, arg0 as u32, idx,
                                arg1 as u32, gate_word,
                                a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13
                            );
                        }
                        k = k.wrapping_add(1);
                        if k > end {
                            break;
                        }
                    }
                }
            }
            if (low as u32) <= 7 && base < top {
                let mut k = base;
                loop {
                    if (k as u32) <= 7 {
                        let idx = k.wrapping_add(low.wrapping_mul(8))
                            as u16 as u32;
                        lf_checker_rt::callee_thiscall!(
                            3, u32, this_, arg0 as u32, idx,
                            arg1 as u32, gate_word,
                            a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13
                        );
                    }
                    k = k.wrapping_add(1);
                    if k >= top {
                        break;
                    }
                }
            }
            let reach = diff.wrapping_add(top);
            if (reach as u32) <= 7 && base < top {
                let mut k = base;
                loop {
                    if (k as u32) <= 7 {
                        let idx = k.wrapping_add(reach.wrapping_mul(8))
                            as u16 as u32;
                        lf_checker_rt::callee_thiscall!(
                            3, u32, this_, arg0 as u32, idx,
                            arg1 as u32, gate_word,
                            a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13
                        );
                    }
                    k = k.wrapping_add(1);
                    if k >= top {
                        break;
                    }
                }
            }
            best = best + STEP;
            if best > gate {
                return arg0 as u32;
            }
            round = round.wrapping_add(1);
            low = low.wrapping_sub(1);
            base = base.wrapping_sub(1);
            top = top.wrapping_add(1);
            if round >= 5 {
                return arg0 as u32;
            }
        }
    }
});
