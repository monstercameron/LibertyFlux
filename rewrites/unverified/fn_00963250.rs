// original: 0x00963250 timing_delta_40
/// Store a timing value and the scaled delta from the live field.
///
/// Resolves the timing record, stores the argument at +0x3C, and unless the
/// record's live field (+0x40) is -1, stores the live field minus the
/// argument, converted to float and scaled by 0.001, at +0x44. Returns the
/// helper's answer, or the live field itself (-1) on the skip path.
export!(cdecl, rw_00963250(arg: u32) -> u32 {
    unsafe {
        const SCALE: f32 = f32::from_bits(0x3A83126F);
        const TWO32: f64 = 4294967296.0;
        let obj = *global::<u32>(0x11F6954);
        let slot = *global::<u32>(0x11F6F34);
        let a1 = callee_thiscall!(1, u32, obj, slot);
        *((a1 as *mut u32).add(0x3C / 4)) = arg;
        let a2 = callee_thiscall!(1, u32, obj, slot);
        let live = *((a2 as *const u32).add(0x40 / 4));
        if live == 0xFFFFFFFF {
            return live;
        }
        let v = live.wrapping_sub(arg);
        let mut d = (v as i32) as f64;
        if (v as i32) < 0 {
            d += TWO32;
        }
        let f = (d as f32) * SCALE;
        let a3 = callee_thiscall!(1, u32, obj, slot);
        *((a3 as *mut u32).add(0x44 / 4)) = f.to_bits();
        a3
    }
});
