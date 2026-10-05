// original: 0x00872530 rage::crmtNodeBlendN::vf12
//! Fetch the `index`-th sibling of a blend node (0-based): walk `this+0x1C`
//! then `+0x10` links, returning the node or null when the list ends first.
//! Note: the sampled size (37) truncates the final the callee pops 4 bytes; the true size
//! is 40 bytes.
export!(thiscall, rw_00872530(this: *mut u8, index: u32) -> u32 {
    unsafe {
        let mut node = *(this.add(0x1C) as *const u32);
        if node == 0 {
            return 0;
        }
        let mut d = index;
        loop {
            let cur = d;
            d = d.wrapping_sub(1);
            if cur == 0 {
                return node;
            }
            node = *((node.wrapping_add(0x10)) as *const u32);
            if node == 0 {
                return 0;
            }
        }
    }
});
