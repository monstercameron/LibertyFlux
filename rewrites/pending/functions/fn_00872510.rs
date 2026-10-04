// original: 0x00872510 rage::crmtNodeBlendN::vf9
//! Count the siblings in a blend node's child list: follow `this+0x1C`,
//! then `+0x10` links, and return how many nodes were visited.
export!(thiscall, rw_00872510(this: *mut u8) -> u32 {
    unsafe {
        let mut node = *(this.add(0x1C) as *const u32);
        let mut n = 0u32;
        while node != 0 {
            node = *((node.wrapping_add(0x10)) as *const u32);
            n = n.wrapping_add(1);
        }
        n
    }
});
