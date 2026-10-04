// original: 0x00BE93D0 table_emit_b (proposed)
/// Look up a target in the stride table and emit to it.
///
/// Identical to 0x00BE9360 except the emit goes through the neighbouring
/// callee. See that function for the argument and table description.
///
/// Original: 0x00BE93D0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00BE93D0(obj: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x08;
        const AUX: u32 = 0x0c;
        const BACK: u32 = 0xa4;
        const SUB: u32 = 0x04;
        const ROW: u32 = 0x40;
        const ROW_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f14;
        const STRIDE_GLOBAL: u32 = 0x0115D968;
        const TABLE_GLOBAL: u32 = 0x0115D988;
        const LOOKUP: u32 = 1;
        const EMIT: u32 = 2;
        let key = ((obj + KEY) as *const u32).read_unaligned();
        let h: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key);
        if h == 0 {
            return 0;
        }
        if ((h + BACK) as *const u32).read_unaligned() != key {
            return 0;
        }
        let aux = ((obj + AUX) as *const u32).read_unaligned();
        let sub = ((h + SUB) as *const u8).read() as u32;
        if sub == 0xff {
            let _: u32 = lf_checker_rt::callee_thiscall!(EMIT, u32, 0, aux);
            return 0;
        }
        let row = ((h + ROW) as *const u8).read() as u32;
        let stride = (lf_checker_rt::global::<u32>(STRIDE_GLOBAL)).read_unaligned();
        let base = (lf_checker_rt::global::<u32>(TABLE_GLOBAL)).read_unaligned();
        let cell = (base
            .wrapping_add(row.wrapping_mul(ROW_STRIDE))
            .wrapping_add(TABLE_BIAS) as *const u32)
            .read_unaligned();
        let target = stride.wrapping_mul(sub).wrapping_add(cell);
        let _: u32 = lf_checker_rt::callee_thiscall!(EMIT, u32, target, aux);
        0
    }
});

