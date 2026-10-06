// original: 0x00C4D890 task_params_emit (proposed)

/// Check a ped's task state and submit three clamped control parameters.
///
/// `obj` is the ped/task object, `arg1` an opaque value forwarded to one
/// callee. The function bails out (returning nothing) unless every gate
/// holds: global `0x11F7060` is not 1, global `0x12088B4` equals the
/// constant at `0xF1C040`, global `0x1037720` is not 0x12, bit 5 of the
/// word at `obj+0x24` is set, and either the low nibble of the byte at
/// `obj+0x1E2` is below 2 or the word at `obj+0x1BC` is zero. It then asks
/// callee 1 for the range of `[obj+0x20]+0x30` and bails when that exceeds
/// 400.0.
///
/// Past the gates it resolves a handle (callee 2, tag `0xEC9D20`), opens a
/// parameter block through the object at `0x1394D60` (callees 3 and 5),
/// forwards `arg1` (callee 4), and fetches a 3-float position through the
/// virtual slot at `+0xEC` of `[obj]`. Three floats are derived, each in
/// the original's SSE order and clamped against 1.0: the position length
/// times 0.2; a table value (`[0x1174790]*0x210` into `0x15E89E4`, minus
/// 2.0, times 1/7, additionally floored at 0.0 when below 1.0); and
/// `[obj+0xB24]` times 0.25. Each is submitted with its tag (`0xEC9D2C`,
/// `0xEC9D34`, `0xEC9D3C`, callee 7), the block is closed (callee 8), and a
/// final 8-argument call (callee 9) carries the handle, `obj`, two ones,
/// a zero and the three floats.
///
/// All `comiss` selections use ordered-above (`ja`) semantics, reproduced
/// with plain `>` (false for NaN, -0.0 equal to +0.0). The range answer
/// arrives on the x87 stack; the rewrite reads it as an `f32` return.
/// Original: 0x00C4D890 (stdcall, two stack arguments, no return value).
lf_checker_rt::export!(stdcall, rw_00c4d890(obj: u32, arg1: u32) -> u32 {
    unsafe {
        const G_ACTIVE: u32 = 0x11F7060;
        const G_TOKEN: u32 = 0x12088B4;
        const C_TOKEN: u32 = 0xF1C040;
        const G_MODE: u32 = 0x1037720;
        const MODE_SKIP: u32 = 0x12;
        const OBJ_VTABLE: u32 = 0x00;
        const VSLOT_POS: u32 = 0xEC;
        const OBJ_RANGE_IN: u32 = 0x20;
        const RANGE_BIAS: u32 = 0x30;
        const OBJ_FLAGS: u32 = 0x24;
        const FLAG_BIT: u32 = 5;
        const OBJ_STATE: u32 = 0x1E2;
        const OBJ_ALT: u32 = 0x1BC;
        const OBJ_METER: u32 = 0xB24;
        const C_LIMIT: u32 = 0xFE8C20; // 400.0
        const C_ONE: u32 = 0xFE88E8; // 1.0
        const C_LEN_K: u32 = 0xFE87D0; // 0.2
        const C_TAB_SUB: u32 = 0xFE8A24; // 2.0
        const C_TAB_K: u32 = 0xE83170; // 1/7
        const C_METER_K: u32 = 0xFE87E4; // 0.25
        const G_TAB_IDX: u32 = 0x1174790;
        const TAB_BASE: u32 = 0x15E89E4;
        const TAB_STRIDE: u32 = 0x210;
        const SHARED_OBJ: u32 = 0x1394D60;
        const TAG_HANDLE: u32 = 0xEC9D20;
        const TAG_LEN: u32 = 0xEC9D2C;
        const TAG_TAB: u32 = 0xEC9D34;
        const TAG_METER: u32 = 0xEC9D3C;
        const C_RANGE_OF: u32 = 1;
        const C_GET_HANDLE: u32 = 2;
        const C_OPEN: u32 = 3;
        const C_FORWARD: u32 = 4;
        const C_BEGIN: u32 = 5;
        const C_SUBMIT: u32 = 7;
        const C_CLOSE: u32 = 8;
        const C_FINISH: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn cf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        if g32(G_ACTIVE) == 1 {
            return 0;
        }
        if g32(G_TOKEN) != g32(C_TOKEN) {
            return 0;
        }
        if g32(G_MODE) == MODE_SKIP {
            return 0;
        }
        if ((rd32(obj.wrapping_add(OBJ_FLAGS)) >> FLAG_BIT) & 1) == 0 {
            return 0;
        }
        let state = ((obj.wrapping_add(OBJ_STATE)) as *const u8).read() & 0x0f;
        if state >= 2 && rd32(obj.wrapping_add(OBJ_ALT)) != 0 {
            return 0;
        }
        let range_in = rd32(obj.wrapping_add(OBJ_RANGE_IN)).wrapping_add(RANGE_BIAS);
        let range: f32 = lf_checker_rt::callee_cdecl!(C_RANGE_OF, f32, range_in);
        if range > cf(C_LIMIT) {
            return 0;
        }
        let tag0 = lf_checker_rt::relocated(TAG_HANDLE);
        let handle: u32 = lf_checker_rt::callee_cdecl!(C_GET_HANDLE, u32, tag0, 0);
        let shared = lf_checker_rt::relocated(SHARED_OBJ);
        let blk: u32 = lf_checker_rt::callee_thiscall!(C_OPEN, u32, shared, handle, 0, 0);
        if blk == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(C_FORWARD, u32, blk, arg1);
        let _: u32 = lf_checker_rt::callee_thiscall!(C_BEGIN, u32, shared, blk, obj, 0);
        let vtable = rd32(obj.wrapping_add(OBJ_VTABLE));
        let getpos = rd32(vtable.wrapping_add(VSLOT_POS));
        let scratch: u32 = 0;
        let pos_fn: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(getpos as usize);
        let pos = pos_fn(obj, core::ptr::addr_of!(scratch) as u32);
        let one = cf(C_ONE);
        let px = rdf(pos);
        let py = rdf(pos.wrapping_add(4));
        let pz = rdf(pos.wrapping_add(8));
        let xx = mul(px, px);
        let yy = mul(py, py);
        let zz = mul(pz, pz);
        let len = mul(add(add(xx, yy), zz).sqrt(), cf(C_LEN_K));
        let f_len = if one > len { len } else { one };
        let _: u32 =
            lf_checker_rt::callee_thiscall!(C_SUBMIT, u32, blk, lf_checker_rt::relocated(TAG_LEN), f_len.to_bits());
        let idx = g32(G_TAB_IDX);
        let tab = lf_checker_rt::relocated(TAB_BASE)
            .wrapping_add(idx.wrapping_mul(TAB_STRIDE));
        let tv = mul(sub(rdf(tab), cf(C_TAB_SUB)), cf(C_TAB_K));
        let f_tab = if one > tv {
            if tv > 0.0 {
                tv
            } else {
                0.0
            }
        } else {
            one
        };
        let _: u32 =
            lf_checker_rt::callee_thiscall!(C_SUBMIT, u32, blk, lf_checker_rt::relocated(TAG_TAB), f_tab.to_bits());
        let f_meter = mul(rdf(obj.wrapping_add(OBJ_METER)), cf(C_METER_K));
        let f_meter = if one > f_meter { f_meter } else { one };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            C_SUBMIT,
            u32,
            blk,
            lf_checker_rt::relocated(TAG_METER),
            f_meter.to_bits()
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(C_CLOSE, u32, blk);
        let fl = f_len.to_bits();
        let _: u32 = lf_checker_rt::callee_cdecl!(
            C_FINISH, u32, handle, obj, 1, 1, 0, fl, f_tab.to_bits(), f_meter.to_bits()
        );
        0
    }
});
