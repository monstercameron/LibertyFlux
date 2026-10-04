// original: 0x008d4f20 vec_channels_lerp
/// Advances thirteen float channels of the object toward the matching source
/// channels by fraction `t`: `dst += (src - dst) * t`. Channels sit at
/// 0x50/0x54/0x58, 0x60/0x64/0x68, 0x80/0x84/0x88 and 0x90/0x94/0x98/0x9c.
/// The original adds `dst + step` on seven lanes and `step + dst` on the
/// other six (register allocation); the order decides dual-NaN payloads, so
/// each lane keeps its own order. Returns the source pointer.
export!(thiscall, rw_008d4f20(this_: u32, src: u32, t: f32) -> u32 {
    unsafe {
        // (lane, dst_first): per-lane add-operand order of the original.
        const LANES: [(u32, bool); 13] = [
            (0x50, true),
            (0x54, true),
            (0x58, false),
            (0x60, true),
            (0x64, true),
            (0x68, true),
            (0x80, true),
            (0x84, false),
            (0x88, true),
            (0x90, false),
            (0x94, false),
            (0x98, false),
            (0x9c, false),
        ];
        for (lane, dst_first) in LANES {
            let d = (this_ + lane) as *mut f32;
            let s = *((src + lane) as *const f32);
            let step = fmul(fsub(s, *d), t);
            *d = if dst_first { fadd(*d, step) } else { fadd(step, *d) };
        }
        src
    }
});
