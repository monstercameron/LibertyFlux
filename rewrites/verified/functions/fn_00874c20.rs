// original: 0x00874c20 filter_replace_child
/// Replaces a filter node's child: attaches the new child if there is one,
/// detaches the old child if there was one, then stores the new child.
export!(thiscall, rw_00874c20(node: u32, new_child: u32) -> () {
    unsafe {
        if new_child != 0 {
            let slot = ((new_child as *const u32).read() + 4) as *const u32;
            let attach: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            attach(new_child);
        }
        let old = ((node + 0x20) as *const u32).read();
        if old != 0 {
            let slot = ((old as *const u32).read() + 8) as *const u32;
            let detach: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            detach(old);
        }
        ((node + 0x20) as *mut u32).write(new_child);
    }
});
