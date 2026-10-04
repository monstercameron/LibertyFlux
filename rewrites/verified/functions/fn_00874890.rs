// original: 0x00874890 locked_node_relink
/// Re-attaches a target node while holding the node's lock: looks the target
/// up by its key, detaches the target's old link, then links the target into
/// the found entry and bumps the entry's reference count.
export!(thiscall, rw_00874890(node: u32, target: u32) -> () {
    unsafe {
        if target == 0 {
            return;
        }
        let cs = node + 8;
        if ((cs) as *const u32).read() != 0 {
            let slot = global::<u32>(0x00E731CC);
            let enter: extern "stdcall" fn(u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            enter(cs);
        }
        let key = ((target + 6) as *const u16).read() as u32;
        let found = callee_thiscall!(1, u32, node, key);
        if found != 0 {
            let slot = (target as *const u32).read() as *const u32;
            let detach: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            detach(target, 0);
            let link = ((found + 4) as *const u32).read();
            let next = ((link + 0xC) as *const u32).read();
            (target as *mut u32).write(next);
            let count = ((link + 6) as *mut u16).read().wrapping_add(1);
            ((link + 6) as *mut u16).write(count);
            ((link + 0xC) as *mut u32).write(target);
        }
        if ((cs) as *const u32).read() != 0 {
            let slot = global::<u32>(0x00E731C8);
            let leave: extern "stdcall" fn(u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            leave(cs);
        }
    }
});
