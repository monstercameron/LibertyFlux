// original: 0x00a2cc30 ped_slot_release

/// Release one reference on a table slot selected by the argument.
/// The table header at the global `G_TAB` gives a base pointer (`+0`),
/// a tag (`+4`) and a stride (`+0xC`). When the byte at
/// `arg + tag` has bit 0x80 set the original takes its assertion path:
/// it decrements the word at address 4, faulting exactly like here. Else
/// the slot `base + arg * stride` loses one reference at `+4`.
/// Original: 0x00a2cc30 (cdecl, one stack word, no return value).
lf_checker_rt::export!(cdecl, rw_00a2cc30(arg: u32) -> u32 {
    unsafe {
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
        const G_TAB: u32 = 0x16dd5d0;
        const TAG_BIT: u8 = 0x80;
        let tab = rd32(lf_checker_rt::relocated(G_TAB));
        let tag = rd32(tab.wrapping_add(4));
        let probe = (arg.wrapping_add(tag) as *const u8).read();
        if (probe & TAG_BIT) != 0 {
            let p = 4 as *mut u32;
            p.write_unaligned(p.read_unaligned().wrapping_sub(1));
        } else {
            let base = rd32(tab);
            let stride = rd32(tab.wrapping_add(0x0c));
            let slot = base.wrapping_add((arg as i32).wrapping_mul(stride as i32) as u32);
            let p = slot.wrapping_add(4) as *mut u32;
            p.write_unaligned(p.read_unaligned().wrapping_sub(1));
        }
        0
    }
});
