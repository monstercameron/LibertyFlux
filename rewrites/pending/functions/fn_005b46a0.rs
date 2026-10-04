// original: 0x005B46A0 aspect_band_lookup
/// Picks the display band for the current aspect ratio, fetches the paired
/// configuration blocks for that band, and publishes them to the two outputs.
/// Returns the second word of the second block.
export!(cdecl, rw_005B46A0(out0: *mut u32, out1: *mut u32) -> u32 {
    unsafe {
        let x: f32 = callee_thiscall!(1, f32, relocated(0x0118D7F0), 1);
        let t1 = *global::<f32>(0x00FE899C);
        let t2 = *global::<f32>(0x00FE8980);
        let t3 = *global::<f32>(0x00FE895C);
        let t4 = *global::<f32>(0x00FE8928);
        let t5 = *global::<f32>(0x00FE88E8);
        let (a, b) = if x > t1 {
            (1u32, 2u32)
        } else if x > t2 {
            (0xB5, 0xB6)
        } else if x > t3 {
            (0xB7, 0xB8)
        } else if x > t4 {
            (0x78, 0x79)
        } else if x > t5 {
            (0xB9, 0xBA)
        } else {
            (1u32, 2u32)
        };
        let mut buf0 = [0u32; 2];
        let mut buf1 = [0u32; 2];
        let p0 = callee_cdecl!(2, u32, buf0.as_mut_ptr() as u32, a) as *const u32;
        out0.add(0).write(p0.add(0).read());
        out0.add(1).write(p0.add(1).read());
        let p1 = callee_cdecl!(2, u32, buf1.as_mut_ptr() as u32, b) as *const u32;
        out1.add(0).write(p1.add(0).read());
        out1.add(1).write(p1.add(1).read());
        p1.add(1).read()
    }
});
