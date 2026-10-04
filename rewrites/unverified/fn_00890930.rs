// original: 0x00890930 
// 00890930 audSound chain free: walk the link chain to its end, then release
// the tail's pool entry.
export!(thiscall, rw_00890930(this: *mut u8) -> () {
    unsafe {
        let stride = *global::<u32>(0x115D964);
        let base = *global::<u32>(0x115D988);
        let mut node = this;
        loop {
            let sel = *node.add(5);
            if sel == 0xFF {
                break;
            }
            let row = (*node.add(0x40) as u32).wrapping_mul(0x6F40);
            let entry = *((row.wrapping_add(base).wrapping_add(0x6F10)) as *const u32);
            node = stride.wrapping_mul(sel as u32).wrapping_add(entry) as *mut u8;
        }
        let sel = *node.add(4);
        let arg = if sel == 0xFF {
            0xD8u32
        } else {
            let row = (*node.add(0x40) as u32).wrapping_mul(0x6F40);
            let entry = *((row.wrapping_add(base).wrapping_add(0x6F14)) as *const u32);
            (*global::<u32>(0x115D968))
                .wrapping_mul(sel as u32)
                .wrapping_add(entry)
                .wrapping_add(0xD8)
        };
        callee_cdecl!(1, u32, arg);
    }
});
