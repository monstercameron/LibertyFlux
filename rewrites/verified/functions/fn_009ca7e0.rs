// original: 0x009CA7E0 timed_element_sweep_update (proposed)

/// Sweep the timed-element table, updating every element whose gate is NaN.
///
/// `time` is the current tick as a float. The global index selects an object
/// from the object table; the object holds a signed element count at
/// `+COUNT_OFF` and element slots of `ELEM_STRIDE` bytes starting at
/// `+ELEM_BASE`. The function returns the object pointer, or zero when the
/// table slot is null.
///
/// For each element the gate callee (id 1, thiscall, `0x8C`) is asked for a
/// score. A non-NaN score skips the element; a NaN score runs the update:
/// three float sums of the two callee-returned triples are formed (callees
/// 2 and 3), two more gate samples are taken (`0x8D`, `0x8E`), an out-buffer
/// triple is collected (callee 2), two classifier probes run (callee 4), and
/// the sixteen-word update call (callee 5) fires with the frame buffers, the
/// gate samples, the scaled sample and constant flags that record which
/// classifier branch was taken (`0x184` clean, `0x180`/`0x80` flagged).
///
/// Frame facts the contract relies on: the loop index slot is never
/// initialized (the pre-loop zero store lands 8 bytes past it), so the first
/// iteration reads the initial fill, defined as 0 by the contract
/// (`stack_fill`); the element-offset slot doubles as the second gate
/// sample's home later in the iteration; the three scale slots are
/// re-initialized every iteration. Callee out-buffers live in the frame and
/// are compared by snapshot, never by address.
///
/// Float order is the original's, pinned through `black_box` helpers; the
/// NaN gate is a bit test, matching `ucomiss` against `+0.0` exactly.
///
/// Original: 0x009CA7E0 (cdecl, one float stack word).
lf_checker_rt::export!(cdecl, rw_009CA7E0(time: f32) -> u32 {
    unsafe {
        const IDX_GLOB: u32 = 0x1294730;
        const TABLE_GLOB: u32 = 0x1293E58;
        const COUNT_OFF: u32 = 0x103F4;
        const ELEM_BASE: u32 = 0xF930;
        const ELEM_STRIDE: u32 = 0x24;
        const FLAG_GLOB: u32 = 0x103EED4;
        const SCALE_ADDR: u32 = 0xFE8830;
        const PROBE_IMM_A: u32 = 0xE952EC;
        const PROBE_IMM_B: u32 = 0xE952FC;
        const GATE_ID: u32 = 0x8C;
        const SAMPLE_ID_A: u32 = 0x8D;
        const SAMPLE_ID_B: u32 = 0x8E;
        const OUT_KIND: u32 = 0x8F;
        const OUT_SIDE: u32 = 0x8B;
        const FLAG_CLEAN: u32 = 0x184;
        const FLAG_FIRST_ZERO: u32 = 0x180;
        const FLAG_FIRST_NONZERO: u32 = 0x80;
        const CAL_GATE: u32 = 1;
        const CAL_OUT: u32 = 2;
        const CAL_PAIR: u32 = 3;
        const CAL_PROBE: u32 = 4;
        const CAL_UPDATE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn is_nan_bits(b: u32) -> bool {
            b & 0x7F80_0000 == 0x7F80_0000 && b & 0x007F_FFFF != 0
        }

        let arg_bits = time.to_bits();
        let idx = rd32(lf_checker_rt::relocated(IDX_GLOB));
        let obj = rd32(lf_checker_rt::relocated(TABLE_GLOB).wrapping_add(idx.wrapping_mul(4)));
        if obj == 0 {
            return 0;
        }
        let count = rd32(obj.wrapping_add(COUNT_OFF));
        if (count as i32) <= 0 {
            return obj;
        }
        let flag_word = rd32(lf_checker_rt::relocated(FLAG_GLOB));
        let scale = f32::from_bits(rd32(lf_checker_rt::relocated(SCALE_ADDR)));
        let mut index = 0u32;
        let mut esi = ELEM_BASE;
        loop {
            let elem = rd32(obj.wrapping_add(esi));
            let gate: f32 = lf_checker_rt::callee_thiscall!(CAL_GATE, f32, elem, GATE_ID, arg_bits);
            if is_nan_bits(gate.to_bits()) {
                let mut out_main = [0u32; 2];
                let pair_a = lf_checker_rt::callee_thiscall!(
                    CAL_OUT,
                    u32,
                    elem,
                    out_main.as_mut_ptr() as u32,
                    0,
                    arg_bits
                );
                let mut pair_ptr_slot = [0u32; 1];
                let pair_b = lf_checker_rt::callee_cdecl!(
                    CAL_PAIR,
                    u32,
                    pair_ptr_slot.as_mut_ptr() as u32,
                    idx
                );
                let s0 = add(
                    f32::from_bits(rd32(pair_a)),
                    f32::from_bits(rd32(pair_b)),
                );
                let s1 = add(
                    f32::from_bits(rd32(pair_a.wrapping_add(4))),
                    f32::from_bits(rd32(pair_b.wrapping_add(4))),
                );
                let s2 = add(
                    f32::from_bits(rd32(pair_a.wrapping_add(8))),
                    f32::from_bits(rd32(pair_b.wrapping_add(8))),
                );
                let mut sums = [s0.to_bits(), s1.to_bits(), s2.to_bits()];
                let mut out_side = [0u32; 2];
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_OUT,
                    u32,
                    elem,
                    out_side.as_mut_ptr() as u32,
                    OUT_KIND,
                    arg_bits
                );
                let mut out_aux = [0u32; 2];
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_OUT,
                    u32,
                    elem,
                    out_aux.as_mut_ptr() as u32,
                    OUT_SIDE,
                    arg_bits
                );
                let sample_a: f32 =
                    lf_checker_rt::callee_thiscall!(CAL_GATE, f32, elem, SAMPLE_ID_A, arg_bits);
                let sample_b: f32 =
                    lf_checker_rt::callee_thiscall!(CAL_GATE, f32, elem, SAMPLE_ID_B, arg_bits);
                let mut carried = out_side[1];
                let probe_base = obj.wrapping_add(4).wrapping_add(esi);
                let first_zero = lf_checker_rt::callee_cdecl!(CAL_PROBE, u32, PROBE_IMM_A, probe_base, 3) == 0;
                let second_zero = lf_checker_rt::callee_cdecl!(CAL_PROBE, u32, PROBE_IMM_B, probe_base, 3) == 0;
                let scaled = mul(sample_b, scale);
                if second_zero {
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        CAL_UPDATE,
                        u32,
                        0,
                        2,
                        FLAG_CLEAN,
                        out_side.as_mut_ptr() as u32,
                        &mut carried as *mut u32 as u32,
                        sums.as_mut_ptr() as u32,
                        out_aux.as_mut_ptr() as u32,
                        gate.to_bits(),
                        0,
                        flag_word,
                        sample_a.to_bits(),
                        scaled.to_bits(),
                        sample_b.to_bits(),
                        0xFFFF_FFFF,
                        0xFFFF_FFFF,
                        obj.wrapping_add(esi)
                    );
                } else {
                    let flag = if first_zero { FLAG_FIRST_ZERO } else { FLAG_FIRST_NONZERO };
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        CAL_UPDATE,
                        u32,
                        0,
                        2,
                        flag,
                        out_side.as_mut_ptr() as u32,
                        &mut carried as *mut u32 as u32,
                        sums.as_mut_ptr() as u32,
                        out_aux.as_mut_ptr() as u32,
                        gate.to_bits(),
                        0,
                        flag_word,
                        sample_a.to_bits(),
                        scaled.to_bits(),
                        sample_b.to_bits(),
                        0xFFFF_FFFF,
                        0xFFFF_FFFF,
                        0
                    );
                }
            }
            index = index.wrapping_add(1);
            esi = esi.wrapping_add(ELEM_STRIDE);
            if (index as i32) >= (count as i32) {
                break;
            }
        }
        obj
    }
});
