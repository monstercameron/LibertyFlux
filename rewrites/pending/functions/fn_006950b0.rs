// original: 0x006950b0 channel_store_propagate
/// Store a channel value, optionally propagating it to linked channels.
///
/// Always stores `fbits` at `index` in the value array. When `flag` is
/// nonzero, walks the node graph reachable from the index's node (child at
/// +C, sibling at +8, parent at +10) and stores the same value at each
/// visited node's channel index (+14). Returns nothing meaningful.
export!(thiscall, rs80_6950b0(this: *const u8, index: u32, fbits: u32, flag: u32) -> u32 {
    unsafe {
        let arr = *((this).add(0x0C) as *const u32) as *mut u32;
        *arr.add(index as usize) = fbits;
        if (flag as u8) == 0 {
            return 0; // unchecked: the original leaks entry-EAX on this path
        }
        let node_base = *(((*((this).add(0x14) as *const u32)) as *const u8).add(0) as *const u32);
        let start = node_base.wrapping_add(index.wrapping_mul(0xE0));
        let mut cur = start;
        loop {
            if cur == 0 {
                break;
            }
            let down = *((cur as *const u8).add(0x0C) as *const u32);
            if down != 0 {
                cur = down;
            } else if cur == start {
                cur = 0;
            } else {
                let sib = *((cur as *const u8).add(8) as *const u32);
                if sib != 0 {
                    cur = sib;
                } else {
                    let mut up = *((cur as *const u8).add(0x10) as *const u32);
                    if up == 0 {
                        cur = 0;
                    } else {
                        loop {
                            if *((up as *const u8).add(8) as *const u32) != 0 {
                                break;
                            }
                            if up == start {
                                up = 0;
                                break;
                            }
                            up = *((up as *const u8).add(0x10) as *const u32);
                            if up == 0 {
                                break;
                            }
                        }
                        if up == 0 || up == start {
                            cur = 0;
                        } else {
                            cur = *((up as *const u8).add(8) as *const u32);
                        }
                    }
                }
            }
            if cur == 0 {
                break;
            }
            let ci = *((cur as *const u8).add(0x14) as *const u16) as usize;
            *arr.add(ci) = fbits;
        }
        0 // unchecked: see contract (return is entry-dependent residue)
    }
});
