// original: 0x00874bf0 rage::crmtNodeFilter::vf4
/// Forwards the filter's child to its target when it has one, replacing the
/// call's argument with the child; does nothing when there is no child.
export!(thiscall, rw_00874bf0(node: u32, target: u32) -> () {
    unsafe {
        let child = ((node + 0x20) as *const u32).read();
        if child != 0 {
            let slot = ((target as *const u32).read() + 0x3C) as *const u32;
            let forward: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            forward(target, child);
        }
    }
});
