// original: 0x00694e90 rage::crFrameFilterMover::vf3
/// Query one filter parameter, or fall back to the mode flag.
///
/// `sel` chooses which parameter slot to read (5 -> slot C, 6 -> slot 10);
/// any other value, or a lane id that does not match the object, returns
/// the mode byte instead. On success the value is also written through
/// `out`, and the return is whether it exceeds the shared threshold.
export!(thiscall, rs80_694e90(this: *const u8, sel: u32, lane: u32, out: *mut u32) -> u32 {
    unsafe {
        const SLOT_A: u32 = 5;
        let sub = (sel as u8).wrapping_sub(SLOT_A as u8);
        let lane_ok = (lane as u16) == *((this).add(0x14) as *const u16);
        let bits = if sub == 0 && lane_ok {
            Some(*((this).add(0x0C) as *const u32))
        } else if sub == 1 && lane_ok {
            Some(*((this).add(0x10) as *const u32))
        } else {
            None
        };
        match bits {
            Some(b) => {
                *out = b;
                let thresh = f32::from_bits(*global::<u32>(0x00FE8628));
                ((f32::from_bits(b) > thresh) as u8) as u32
            }
            None => *((this).add(0x16)) as u32,
        }
    }
});
