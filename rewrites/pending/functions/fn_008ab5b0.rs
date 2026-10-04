// original: 0x008AB5B0 audio_follower_select
/// Produce the follower output: silent when disabled, the held value once
/// after a retrigger, otherwise refresh both mode bytes through the probe
/// routine and follow the selected one of four rate inputs.
export!(thiscall, rw_008AB5B0(obj: *mut u8) -> f32 {
    unsafe {
        if *obj == 0 {
            return 0.0;
        }
        if *obj.add(1) != 0 {
            let held = *(obj.add(0x1C) as *const f32);
            *obj.add(1) = 0;
            return held;
        }
        let probe = |bits: u32| callee_cdecl!(1, u32, bits) as u8;
        let field = |o: usize| *(obj.add(o) as *const u32);
        if probe(field(0x14)) != 0 {
            let mode = obj.add(0x20);
            *mode = (*mode == 0) as u8;
        }
        if probe(field(0x18)) != 0 {
            let mode = obj.add(0x21);
            *mode = (*mode == 0) as u8;
        }
        let pick = if *obj.add(0x21) != 0 {
            if *obj.add(0x20) != 0 { 0x10 } else { 0x0C }
        } else if *obj.add(0x20) != 0 {
            8
        } else {
            4
        };
        callee_thiscall!(2, f32, (obj as u32).wrapping_add(0x24), field(pick))
    }
});
