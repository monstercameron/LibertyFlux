// original: 0x009fa480 playstat_multi_stat_publish
/// Publish a stat across the wide, narrow and byte-sink stages.
///
/// Runs the shared publish chain, forwards two wide datum pairs (skipping
/// the second when both halves are zero), emits one narrow pair, pushes
/// three byte slots through the byte sink, and forwards one trailing byte.
/// Any failed check aborts with 0, else returns 1.
export!(cdecl, rw_009fa480(obj: u32, stat: u32) -> u32 {
    unsafe {
        if callee_cdecl!(1, u32, obj, stat) & 0xFF == 0 {
            return 0;
        }
        let base = obj as *const u32;
        let fields = stat as *const u32;
        let bytes = stat as *const u8;
        const WIDE: u32 = 0x40;
        if callee_thiscall!(2, u32, obj, fields.add(0x38 / 4).read(), fields.add(0x3c / 4).read(), WIDE)
            & 0xFF
            == 0
        {
            return 0;
        }
        let lo = fields.add(0x40 / 4).read();
        let hi = fields.add(0x44 / 4).read();
        if lo | hi != 0 && callee_thiscall!(2, u32, obj, lo, hi, WIDE) & 0xFF == 0 {
            return 0;
        }
        let flag = bytes.add(0x49).read();
        if callee_thiscall!(3, u32, obj, 2) & 0xFF == 0 {
            return 0;
        }
        let total = base.add(3).read().wrapping_add(base.add(1).read());
        callee_cdecl!(4, u32, base.read(), u32::from(flag), 2, total);
        callee_thiscall!(5, u32, obj, 2);
        for off in [0x48u32, 0x4a, 0x4b] {
            if callee_thiscall!(6, u32, obj, stat.wrapping_add(off), 6) & 0xFF == 0 {
                return 0;
            }
        }
        u32::from(callee_thiscall!(7, u32, obj, u32::from(bytes.add(0x4c).read())) & 0xFF != 0)
    }
});
