// original: 0x006957c0 rage::crAnimToleranceSimple::vf1
/// Tolerance lookup: pick a threshold float by (kind, lane).
///
/// Kinds 0 and 5 read the near threshold (slot 4), kinds 1 and 6 the far
/// threshold (slot 8); both fall back to the shared epsilon when the lane
/// is zero. Every other kind reads the default threshold (slot C).
export!(thiscall, rs80_6957c0(this: *const u8, kind: u32, lane: u32, _u: u32) -> f32 {
    unsafe {
        let bits = match kind as u8 {
            0 | 5 => {
                if (lane as u16) == 0 {
                    *global::<u32>(0x00FE865C)
                } else {
                    *((this).add(4) as *const u32)
                }
            }
            1 | 6 => {
                if (lane as u16) == 0 {
                    *global::<u32>(0x00FE865C)
                } else {
                    *((this).add(8) as *const u32)
                }
            }
            _ => *((this).add(0x0C) as *const u32),
        };
        f32::from_bits(bits)
    }
});
