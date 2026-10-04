// original: 0x008744c0 rage::crmtNodeAnimation::vf2
/// Samples the node's animation state and notifies its listener chain: asks
/// the sampler to classify the current sample, then walks the listeners
/// while the node is active, sending each either an activation or a release
/// message.
export!(thiscall, rw_008744c0(node: u32, arg: u32) -> () {
    unsafe {
        let index = ((arg + 0x24) as *const u32).read();
        let sample = ((arg.wrapping_add(index.wrapping_mul(4))) as *const u32).read();
        let mut outbuf = [0u32; 2];
        callee_thiscall!(1, u32, node.wrapping_add(0x1C), sample, outbuf.as_mut_ptr() as u32);
        if (outbuf.as_ptr() as *const u8).read() == 0 {
            return;
        }
        let tag = ((node + 0x1C) as *const u32).read() & 3;
        let active = if tag != 0 {
            (tag == 3) as u32
        } else {
            let held = ((node + 0x34) as *const u32).read();
            if held == 0 {
                0
            } else {
                ((((held + 6) as *const u8).read() & 1) != 0) as u32
            }
        };
        let chain = ((node + 0x14) as *const u32).read();
        if chain == 0 {
            return;
        }
        let mut scratch = [0u32; 2];
        let mut msg = [0u32; 2];
        if active != 0 {
            msg[0] = 0x40005;
            msg[1] = 0;
        } else {
            msg[0] = 0x40006;
            msg[1] = scratch.as_mut_ptr() as u32;
        }
        let mut cur = chain;
        while cur != 0 {
            let vtable = (cur as *const u32).read();
            let next = ((cur + 0xC) as *const u32).read();
            let slot = (vtable + 0xC) as *const u32;
            let step: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            step(cur, msg.as_ptr() as u32);
            cur = next;
        }
    }
});
