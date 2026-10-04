// original: 0x008738f0 blend_target_swap
use lf_checker_rt::{export};

/// Swap the blend target: step the old links, store the new node.
///
/// Steps the incoming node and the current target at +0x120 through their
/// vtables when present, then stores the node as the new target. EAX is
/// untouched, so no return channel.
export!(thiscall, rw_008738f0(this_: u32, node: u32) -> () {
    unsafe {
        if node != 0 {
            let vt = *(node as *const u32);
            let step: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(*((vt.wrapping_add(4)) as *const u32));
            step(node);
        }
        let target = *((this_.wrapping_add(0x120)) as *const u32);
        if target != 0 {
            let vt = *(target as *const u32);
            let step: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(*((vt.wrapping_add(8)) as *const u32));
            step(target);
        }
        *((this_.wrapping_add(0x120)) as *mut u32) = node;
    }
});
