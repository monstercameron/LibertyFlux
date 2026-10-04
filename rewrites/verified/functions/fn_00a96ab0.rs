// original: 0x00a96ab0 fade_setup
/// Fills a channel from its nine setup arguments and runs the first tick.
///
/// Stores the state word, span, link, bounds pair and flag bits packed from
/// three argument bytes, snapshots the target word through the sample
/// pointer when one is given, mirrors the bounds around 1.0 when the mirror
/// flag byte is set, then clamps, marks dirty and runs the progress update.
export!(thiscall, rw_00a96ab0(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
) -> u32 {
    unsafe {
        *(this as *mut u32) = a0;
        *((this + 8) as *mut u32) = a1;
        let half = *(global::<f32>(0xFE8830) as *const f32);
        let mut cl = (((((a8 & 1) << 1) | (a7 & 1)) << 2) as u8)
            | (*((this + 0x20) as *const u8) & 0xF2);
        cl |= (a2 as u8) & 1;
        *((this + 0x1C) as *mut u32) = a3;
        *((this + 0x0C) as *mut u32) = a5;
        *((this + 0x10) as *mut u32) = a6;
        *((this + 0x20) as *mut u8) = cl;
        if a4 != 0 {
            *((this + 0x18) as *mut u32) = *(a4 as *const u32);
        }
        if (a2 as u8) != 0 {
            let x = f32::from_bits(a5) - half;
            let y = f32::from_bits(a6) - half;
            *((this + 0x10) as *mut f32) = half - x;
            *((this + 0x0C) as *mut f32) = half - y;
        }
        let t = callee_thiscall!(1, u32, this);
        *((this + 0x20) as *mut u8) |= 2;
        *((this + 4) as *mut u32) = t;
        callee_thiscall!(2, u32, this);
        0
    }
});
