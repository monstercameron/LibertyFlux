// original: 0x00874580 rage::crmtNodeAnimation::vf4
/// Forwards the node's blend state to its target: when an animation is
/// attached, calls the target's blend slot with the animation, the two blend
/// weights and the mode flag; otherwise calls the target's idle slot.
export!(thiscall, rw_00874580(node: u32, target: u32) -> () {
    unsafe {
        let held = ((node + 0x34) as *const u32).read();
        if held != 0 {
            let mode = ((node + 0x38) as *const u8).read() as u32;
            let weight_hi = ((node + 0x24) as *const u32).read();
            let weight_lo = ((node + 0x20) as *const u32).read();
            let slot = ((target as *const u32).read() + 0x20) as *const u32;
            let blend: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            blend(target, held, weight_lo, weight_hi, mode);
        } else {
            let slot = ((target as *const u32).read() + 0x48) as *const u32;
            let idle: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            idle(target);
        }
    }
});
