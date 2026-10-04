// original: 0x00874ba0 rage::crmtNodeFilter::vf1
/// Tears down a filter node: detaches its child if it has one, clears the
/// child slot, then runs the two base-class teardowns and clears the
/// parameter block.
export!(thiscall, rw_00874ba0(node: u32) -> () {
    unsafe {
        let child = ((node + 0x20) as *const u32).read();
        if child != 0 {
            let slot = ((child as *const u32).read() + 8) as *const u32;
            let detach: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            detach(child);
        }
        ((node + 0x20) as *mut u32).write(0);
        callee_thiscall!(2, u32, node);
        callee_thiscall!(3, u32, node);
        let base = node as *mut u32;
        base.add(3).write(0);
        base.add(4).write(0);
        if base.add(2).read() != 0 {
            base.add(2).write(0);
        }
    }
});
