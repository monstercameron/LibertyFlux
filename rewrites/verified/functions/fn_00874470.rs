// original: 0x00874470 rage::crmtNodeAnimation::vf1
/// Tears down the animation-specific state of a motion node: runs the base
/// initialiser, clears the parameter block, and releases any held animation
/// (running its destructor when the reference count reaches zero).
export!(thiscall, rw_00874470(node: u32) -> () {
    unsafe {
        let base = node as *mut u32;
        callee_thiscall!(2, u32, node);
        base.add(3).write(0);
        base.add(4).write(0);
        if base.add(2).read() != 0 {
            base.add(2).write(0);
        }
        let held = base.add(13).read();
        if held != 0 {
            let count = ((held + 4) as *mut u16).read().wrapping_sub(1);
            ((held + 4) as *mut u16).write(count);
            if count == 0 {
                let slot = (held as *const u32).read() as *const u32;
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot.read() as usize);
                release(held, 1);
            }
        }
        base.add(13).write(0);
        ((node + 0x38) as *mut u8).write(0);
    }
});
