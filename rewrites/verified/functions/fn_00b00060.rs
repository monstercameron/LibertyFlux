// original: 0x00b00060 update_zoom_level
/// Accumulate a scaled step into the shared level, pick the cap selected by
/// the mode query and clamp the level into [0, cap].
export!(cdecl, rw_00b00060() -> u32 {
    unsafe {
        let info = callee_cdecl!(1, u32,) as *const u8;
        let flag = *info.add(0x26c);
        let mut level = *global::<f32>(0x11735bc);
        if flag & 4 != 0 {
            level *= *global::<f32>(0xfe8750);
        } else {
            level *= *global::<f32>(0xfe870c);
        }
        let slot = global::<f32>(0x103ffec);
        level += *slot;
        *slot = level;
        let mode_obj = callee_cdecl!(2, u32,);
        let mode = callee_thiscall!(3, u32, mode_obj);
        let cap = if mode == 2 {
            *global::<f32>(0xfe88bc)
        } else if mode == 3 {
            *global::<f32>(0xfe8898)
        } else if (mode as i32) >= 4 {
            *global::<f32>(0xfe8874)
        } else {
            *global::<f32>(0xfe88e8)
        };
        let clamped = if cap > level { level } else { cap };
        if !(0.0 > clamped) {
            *slot = clamped;
        } else {
            *slot = 0.0;
        }
        mode
    }
});
