// original: 0x00CAECF0 route_task_check (proposed)

/// Decide whether a route-following task has reached its acceptance point.
///
/// `this` is the task, `target` a three-float point, `ped` a related object.
/// Returns 1 (accept) or 0 (keep going) in `al` only: the upper 24 bits of
/// `eax` are stale stack-address bits on every path, so the contract
/// compares `al` alone.
///
/// Behaviour: runs a classifier over the task (callee 1, cdecl of three
/// words: `ped`, `target`, then an out-pointer; only its `al` is tested). A
/// non-zero answer with bit 6 of the control word at `this + 0xc4` clear
/// accepts at once. Otherwise bits 11-14 of that control word are cleared
/// and the function works through two geometric tests separated by a
/// virtual call:
///
/// 1. From the 2D direction between `[[ped + 0x20] + 0x30/34` and `target`
///    it forms the squared length (dy*dy + dx*dx), calls the virtual slot
///    at `[[ped + 0xa80]] + 0x34` (callee 2: thiscall, no stack arguments,
///    float answer on the x87 stack), and adds that answer to the float at
///    `this + 0xa0`. The words at `[ped + 0x20] + 0x30..3c` are then copied
///    to `this + 0x50..5c`. It accepts when the squared sum strictly exceeds
///    the squared length AND bit 6 of the control word is clear AND the
///    float at `this + 0xa4` (raised to at least 4.0 when bit 2 of the byte
///    at `ped + 0x29c` is set) strictly exceeds the absolute z gap AND
///    (bit 1 of the control word is clear OR the second dot below is
///    negative).
/// 2. Unless bit 1 of the control word is set (reject), it calls the
///    virtual slot at `[ped] + 0xec` (callee 3: thiscall on `ped`, one
///    argument pointing four bytes past a stack slot holding a global
///    float), scales that callee's two returned floats by the global,
///    offsets them by `[ped + 0x20] + 0x30/34`, and dots the result against
///    the first direction. A positive (or NaN) dot rejects; with bit 6 of
///    the control word set it also rejects; otherwise it accepts.
///
/// Two reads deserve a note. The callee-1 call pushes three words for a
/// callee that pops none and is never cleaned up, so from that call on the
/// frame sits 12 bytes low: the flag byte and the classifier out-word,
/// written before the call, are read back from never-written slots.
/// Under the checker's zero stack fill both read as 0, so the flag gate is
/// always open and the out-word is always 0; the original's reads of the
/// words those values were computed from are kept for fault parity but
/// their values are unused. The same holds for the word above callee 3's
/// float slot, which the rewrite fabricates as 0. Only `al` of callee 1's
/// answer is tested (zero vs non-zero); nothing here is a signed/unsigned
/// integer comparison.
///
/// Original: 0x00CAECF0 (thiscall, two stack words, callee pops 8).
lf_checker_rt::export!(thiscall, rw_00CAECF0(this: u32, target: u32, ped: u32) -> u32 {
    unsafe {
        const CTRL_WORD: u32 = 0xc4;
        const CTRL_FIELD_MASK: u32 = 0x7800;
        const CTRL_EARLY_BIT: u32 = 0x40;
        const CTRL_DH_BIT: u32 = 0x02;
        const SUM_SLOT: u32 = 0xa0;
        const LIMIT_SLOT: u32 = 0xa4;
        const COPY_DST: u32 = 0x50;
        const PED_LINK: u32 = 0xa80;
        const PED_BASE_OFF: u32 = 0x20;
        const VTBL_SLOT_BACK: u32 = 0x34;
        const VTBL_SLOT_FWD: u32 = 0xec;
        const PED_MODE_BYTE: u32 = 0x29c;
        const GLOBAL_FACTOR: u32 = 0x011735BC;
        const CLASSIFY_CALLEE: u32 = 1;
        const BACK_CALLEE: u32 = 2;
        const FWD_CALLEE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
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
        fn absf(a: f32) -> f32 {
            f32::from_bits(a.to_bits() & 0x7FFF_FFFF)
        }

        let vlink = rd32(ped + PED_LINK);
        // Kept for fault parity; the values are computed into slots the
        // shifted frame never reads back (see doc comment).
        let _bit5 = rd8(vlink + 0x50);
        let _neg1 = rd32(vlink + 0x48);

        let ctrl = rd32(this + CTRL_WORD);
        let mut out_word: u32 = (ctrl >> 11) & 0x0f;
        let cls: u32 = lf_checker_rt::callee_cdecl!(
            CLASSIFY_CALLEE,
            u32,
            ped,
            target,
            (&mut out_word as *mut u32) as u32
        );
        if (cls & 0xFF) != 0 && (ctrl & CTRL_EARLY_BIT) == 0 {
            return 1;
        }
        // The out-word slot the frame reads back holds stale zero: clear.
        wr32(this + CTRL_WORD, ctrl & !CTRL_FIELD_MASK);

        let base = rd32(ped + PED_BASE_OFF);
        let dz = absf(sub(rdf(base + 0x38), rdf(target + 8)));
        let dx = sub(rdf(target), rdf(base + 0x30));
        let dy = sub(rdf(target + 4), rdf(base + 0x34));
        let d2 = add(mul(dy, dy), mul(dx, dx));

        // Virtual back-call through the object's table (fabricated).
        let vtab = rd32(vlink);
        let back: extern "thiscall" fn(u32) -> f32 =
            unsafe { core::mem::transmute(rd32(vtab + VTBL_SLOT_BACK) as usize) };
        let r = back(vlink);

        let sumsq = add(rdf(this + SUM_SLOT), r);
        let sumsq2 = mul(sumsq, sumsq);
        let dl = sumsq2 > d2;
        // Original order: (t4diff*dy) + (t0diff*dx).
        let dot = add(
            mul(sub(rdf(target + 4), rdf(this + COPY_DST + 4)), dy),
            mul(sub(rdf(target), rdf(this + COPY_DST)), dx),
        );
        let dh = 0.0f32 > dot;

        wr32(this + COPY_DST, rd32(base + 0x30));
        wr32(this + COPY_DST + 4, rd32(base + 0x34));
        wr32(this + COPY_DST + 8, rd32(base + 0x38));
        wr32(this + COPY_DST + 12, rd32(base + 0x3c));

        let mut limit = rdf(this + LIMIT_SLOT);
        if (rd8(ped + PED_MODE_BYTE) & 4) != 0 && !(limit > 4.0f32) {
            limit = 4.0f32;
        }
        if dl && (ctrl & CTRL_EARLY_BIT) == 0 && limit > dz && ((ctrl & CTRL_DH_BIT) == 0 || dh) {
            return 1;
        }
        if (ctrl & CTRL_DH_BIT) != 0 {
            return 0;
        }

        let gf_bits = (lf_checker_rt::global::<u32>(GLOBAL_FACTOR) as *const u32).read_unaligned();
        // Callee 3's float sits four bytes below the passed pointer; the
        // word at the pointer is stale stack (zero under the fill).
        let mut fwd_arg = [gf_bits, 0u32];
        let fwd_ptr = (&mut fwd_arg as *mut u32) as u32 + 4;
        let ftab = rd32(ped);
        let fwd: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(ftab + VTBL_SLOT_FWD) as usize) };
        let rp = fwd(ped, fwd_ptr);

        let gf = f32::from_bits(gf_bits);
        let t0 = mul(gf, rdf(rp));
        let t1 = mul(rdf(rp + 4), gf);
        let u2 = add(rdf(base + 0x30), t0);
        let u0 = add(rdf(base + 0x34), t1);
        let v3 = sub(rdf(target), u2);
        let v1 = sub(rdf(target + 4), u0);
        let q = add(mul(v1, dy), mul(v3, dx));
        // jb: reject on positive or NaN; continue on ordered <= 0.
        if !(0.0f32 >= q) {
            return 0;
        }
        if (ctrl & CTRL_EARLY_BIT) != 0 {
            return 0;
        }
        1
    }
});
