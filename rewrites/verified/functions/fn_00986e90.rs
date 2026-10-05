// original: 0x00986E90 audEmitter_scaled_param_sink (proposed)

/// Mode-selected scaled parameter sink: picks one float from a record,
/// optionally scales it, and hands it to a sink callee.
///
/// `record` points at a record holding three candidate floats at unaligned
/// offsets `PICK0_OFF`/`PICK1_OFF`/`PICK2_OFF` and a flag dword at `FLAG_OFF`.
/// The mode comes from the globals `MODE_ALT`/`MODE_MAIN`: the main value wins
/// unless it is -1 (exact equality), in which case the alt value is used.
/// `mode - 10 <= 7` (UNSIGNED) selects the first float, else
/// `mode - 0x13 <= 3` (UNSIGNED) selects the second, else the third.
/// When the record's low flag byte masked with `FLAG_MASK` equals `FLAG_WANT`,
/// a scale factor is computed from the float table `FACTOR_TABLE` (indexed by
/// the global `FACTOR_INDEX` times `0x210`) and the constants `FLOOR`,
/// `SPAN_CAP` and `SPAN_MUL`: values at or below the floor skip scaling;
/// otherwise the excess times the multiplier is clamped into `[0, cap]`,
/// shifted by the cap and multiplied by the picked float. The result (or the
/// raw pick) is passed by value to the sink callee (id 1). The unsigned
/// comparisons treat large modes (top bit set) as above the bound; the
/// `comiss` unordered (NaN) cases follow the original's taken branches.
/// Float operation order is the original's, pinned against commuting.
/// Original: stdcall, one stack word, callee pops 4, no return value.
lf_checker_rt::export!(stdcall, rw_00986E90(record: u32) -> u32 {
    const MODE_MAIN: u32 = 0x1295854;
    const MODE_ALT: u32 = 0x1295848;
    const PICK0_OFF: u32 = 0x12;
    const PICK1_OFF: u32 = 0x16;
    const PICK2_OFF: u32 = 0x1a;
    const FLAG_OFF: u32 = 5;
    const FLAG_MASK: u8 = 0x0c;
    const FLAG_WANT: u8 = 0x04;
    const FACTOR_INDEX: u32 = 0x1174790;
    const FACTOR_TABLE: u32 = 0x15e89e4;
    const FACTOR_STRIDE: u32 = 0x210;
    const FLOOR: u32 = 0xfe8b38;
    const SPAN_CAP: u32 = 0xfe88e8;
    const SPAN_MUL: u32 = 0xfe876c;
    const SINK: u32 = 1;

    #[inline(always)]
    fn sub(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) - core::hint::black_box(b)
    }
    #[inline(always)]
    fn mul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    #[inline(always)]
    fn add(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    #[inline(always)]
    unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
    }

    unsafe {
        let main = *lf_checker_rt::global::<u32>(MODE_MAIN);
        let mode = if main != 0xFFFFFFFF {
            main
        } else {
            *lf_checker_rt::global::<u32>(MODE_ALT)
        };
        let mut picked = if mode.wrapping_sub(10) <= 7 {
            rdf(record + PICK0_OFF)
        } else if mode.wrapping_sub(0x13) <= 3 {
            rdf(record + PICK1_OFF)
        } else {
            rdf(record + PICK2_OFF)
        };
        let flags = ((record + FLAG_OFF) as *const u32).read_unaligned();
        if (flags as u8) & FLAG_MASK == FLAG_WANT {
            let idx = *lf_checker_rt::global::<u32>(FACTOR_INDEX);
            let base = lf_checker_rt::relocated(FACTOR_TABLE);
            let raw = rdf(base.wrapping_add(idx.wrapping_mul(FACTOR_STRIDE)));
            let floor = rdf(lf_checker_rt::relocated(FLOOR));
            // `comiss raw, floor; jb`: skip when below or unordered.
            if raw >= floor {
                let cap = rdf(lf_checker_rt::relocated(SPAN_CAP));
                let excess = sub(raw, floor);
                let scaled = mul(excess, rdf(lf_checker_rt::relocated(SPAN_MUL)));
                // `comiss 0, scaled; ja`: keep 0 when 0 > scaled (ordered).
                // `comiss scaled, cap; jbe`: take scaled when below, equal
                // or unordered.
                let clamped = if 0.0 > scaled {
                    0.0
                } else if !(scaled > cap) {
                    scaled
                } else {
                    cap
                };
                picked = mul(add(clamped, cap), picked);
            }
        }
        let _: u32 =
            lf_checker_rt::callee_cdecl!(SINK, u32, picked.to_bits());
    }
    0
});
