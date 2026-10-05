// original: 0x0099e010 audio_level_resolve
/// Resolve one audio level probe into a float/int output pair.
///
/// The object (passed in ECX) and four stack words drive the probe. A helper
/// call classifies the request into a flag byte and a status byte; when the
/// object names a driver block and the driver answers, a virtual hook result
/// selects two small table indexes (or a fixed pair when the flag byte is
/// set) whose rescaled samples seed two working slots. A global tuning value
/// optionally passes through a third rescale, and an argument byte selects an
/// affine remap of the level. The larger of the first slot and the level is
/// stored through the second argument, and the third argument receives the
/// truncated global factor or second slot depending on the sign of their
/// difference; a global mute byte forces the float output to -100.0. The
/// result is the third argument.
lf_rb80_rt::export!(thiscall, rw_0099e010(this: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    let rd32 = |addr: u32| unsafe { (addr as *const u32).read_unaligned() };
    let obj = this;
    let mut scratch = [0u8; 8];
    let r1: u32 = lf_rb80_rt::callee_thiscall!(1, u32, lf_rb80_rt::relocated(0x1165880),
        scratch.as_mut_ptr() as u32, 0);
    let flag = scratch[0];
    let bh = (r1 & 0xff) as u8;
    let bl = (a1 & 0xff) as u8;
    let mut c10 = -100.0f32;
    let mut c0c = 0.0f32;
    let v = rd32(obj.wrapping_add(8));
    if v != 0 {
        let p: u32 = lf_rb80_rt::callee_thiscall!(2, u32, v.wrapping_add(0x3c0));
        if p != 0 {
            let p2: u32 = lf_rb80_rt::callee_thiscall!(2, u32, v.wrapping_add(0x3c0));
            let vt = rd32(p2);
            let slot = rd32(vt.wrapping_add(0xc));
            let hook: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(slot as usize) };
            let r3 = hook(p2);
            let mut sv = rd32(r3.wrapping_add(0x14));
            if flag != 0 {
                sv = 0xf;
            }
            let hi = ((sv >> 2) & 3) as f32;
            let ecx_hi = if bl != 0 {
                lf_rb80_rt::relocated(0x12844f0)
            } else {
                lf_rb80_rt::relocated(0x128453c)
            };
            let o1: f32 = lf_rb80_rt::callee_thiscall!(4, f32, ecx_hi, hi.to_bits());
            c10 = o1;
            let lo = (sv & 3) as f32;
            let o2: f32 = lf_rb80_rt::callee_thiscall!(4, f32,
                lf_rb80_rt::relocated(0x1284468), lo.to_bits());
            c0c = o2;
        }
    }
    let g3 = rd32(lf_rb80_rt::relocated(0x1038ddc)) as i32 as f32;
    let x1g = f32::from_bits(rd32(lf_rb80_rt::relocated(0x1167fc0)));
    let mut x0 = if bl != 0 && bh == 0 {
        lf_rb80_rt::callee_thiscall!(4, f32, lf_rb80_rt::relocated(0x12844bc), x1g.to_bits())
    } else {
        -100.0f32
    };
    if (a4 & 0xff) != 0 {
        let c1 = f32::from_bits(rd32(lf_rb80_rt::relocated(0x1038de8)));
        let c0 = f32::from_bits(rd32(lf_rb80_rt::relocated(0x1038de4)));
        x0 = c1 * x1g + c0;
    }
    let x1 = c10;
    let top = if x1 > x0 { x1 } else { x0 };
    let d = x1 - x0;
    unsafe { (a2 as *mut u32).write_unaligned(top.to_bits()) };
    let pick = if !(d >= 0.0) { g3 } else { c0c };
    unsafe { (a3 as *mut u32).write_unaligned((pick as i32) as u32) };
    let mute = unsafe { (lf_rb80_rt::relocated(0x1284382) as *const u8).read_unaligned() };
    if mute == 0 {
        unsafe { (a2 as *mut u32).write_unaligned((-100.0f32).to_bits()) };
    }
    a3
});
