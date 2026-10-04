// original: 0x00a217b0 cam_gated_peak_capture (proposed)

/// Captures a negated peak through gated probes and a magnitude check.
///
/// `a0` points at a record with a linked state pointer at `+LINK_OFF` (a
/// record whose byte at `+LINK_FLAG_OFF` is set vetoes the run), `a1` and
/// the word at `a0 + BASE2_OFF` feed two gate probes (selectors 0x410 and
/// 0x412, each on its base + `GATE_BIAS`), `a2` selects the sampler
/// subject (null returns) and `a3` points at the published peak float.
/// The table index at `a0 + TAB_OFF` picks (via `(i+3)*3`, wrapping) one
/// table word for the classifier callee; its answer's word at `+CODE_OFF`
/// must have bit 4 set and bit 1 clear to arm the capture. The second gate
/// always runs once reached (its call is unconditional; only its answer is
/// tested afterwards). Then the sampler getter/converter pair runs on
/// `a2`: unless the converted magnitude strictly exceeds the stored peak's
/// magnitude the function returns, otherwise the pair runs once more and
/// the negated conversion overwrites the peak. Returns nothing.
///
/// Original: 0x00a217b0 (stdcall, four stack words).
lf_checker_rt::export!(stdcall, rw_00a217b0(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const C_GATE: u32 = 1;
        const C_CLASS: u32 = 2;
        const C_GET: u32 = 3;
        const C_CONV: u32 = 4;
        const LINK_OFF: u32 = 0x6c;
        const LINK_FLAG_OFF: u32 = 0xe;
        const BASE2_OFF: u32 = 0x224;
        const GATE_BIAS: u32 = 0x44;
        const SEL_A: u32 = 0x410;
        const SEL_B: u32 = 0x412;
        const TAB_OFF: u32 = 0x2b0;
        const CODE_OFF: u32 = 0x20;
        const ABS_MASK: u32 = 0x7fff_ffff;
        let link = ((a0 + LINK_OFF) as *const u32).read_unaligned();
        if link != 0 && ((link + LINK_FLAG_OFF) as *const u8).read() != 0 {
            return 0;
        }
        let g1 = lf_checker_rt::callee_thiscall!(C_GATE, u32, a1 + GATE_BIAS, SEL_A);
        if g1 == 0 {
            return 0;
        }
        let i = ((a0 + TAB_OFF) as *const u32).read_unaligned();
        let j = i.wrapping_add(3).wrapping_mul(3);
        let w = ((a0 + j * 4 + TAB_OFF) as *const u32).read_unaligned();
        let t = lf_checker_rt::callee_cdecl!(C_CLASS, u32, w);
        let mut armed = false;
        if t != 0 {
            let c = ((t + CODE_OFF) as *const u32).read_unaligned();
            armed = c & 0x10 != 0 && c & 0x02 == 0;
        }
        let b2 = ((a0 + BASE2_OFF) as *const u32).read_unaligned();
        let g2 = lf_checker_rt::callee_thiscall!(C_GATE, u32, b2 + GATE_BIAS, SEL_B);
        if !armed || g2 == 0 || a2 == 0 {
            return 0;
        }
        let t = lf_checker_rt::callee_thiscall!(C_GET, u32, a2);
        let v = lf_checker_rt::callee_cdecl!(CONV, u32, t);
        let vmag = f32::from_bits(((v as i32) as f32).to_bits() & ABS_MASK);
        let m = f32::from_bits((a3 as *const u32).read_unaligned());
        let mmag = f32::from_bits(m.to_bits() & ABS_MASK);
        if !(vmag > mmag) {
            return 0;
        }
        let t = lf_checker_rt::callee_thiscall!(C_GET, u32, a2);
        let v = lf_checker_rt::callee_cdecl!(CONV, u32, t);
        let f = (v as i32).wrapping_neg() as f32;
        (a3 as *mut u32).write_unaligned(f.to_bits());
        0
    }
});
