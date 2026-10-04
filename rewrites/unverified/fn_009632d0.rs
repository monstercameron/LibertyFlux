// original: 0x009632d0 timing_delta_34
/// Store a timing value and the scaled delta from the live field.
///
/// Same shape as `rw_00963250` with the value at +0x30, the live field at
/// +0x34 and the scaled float at +0x38. Skip path returns the live field.
export!(cdecl, rw_009632d0(arg: u32) -> u32 {
    unsafe {
        const SCALE: f32 = f32::from_bits(0x3A83126F);
        const TWO32: f64 = 4294967296.0;
        let obj = *global::<u32>(0x11F6954);
        let slot = *global::<u32>(0x11F6F34);
        let a1 = callee_thiscall!(1, u32, obj, slot);
        *((a1 as *mut u32).add(0x30 / 4)) = arg;
        let a2 = callee_thiscall!(1, u32, obj, slot);
        let live = *((a2 as *const u32).add(0x34 / 4));
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
        *((a3 as *mut u32).add(0x38 / 4)) = f.to_bits();
        a3
    }
});
