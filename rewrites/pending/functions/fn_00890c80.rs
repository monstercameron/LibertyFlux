// original: 0x00890c80 audio_mark_used
/// Mark this object's tag node used, bumping its owner's counter when new.
///
/// Resolves the tag node through the second row table. When this object has a
/// non-null owner at +0x74 and the node's flag byte at +0xe8 lacks bit 5, the
/// owner counter helper (stubbed by the checker) runs. Either way bit 5 is
/// then set. Returns the helper's answer, or the row-table base the original
/// leaves in EAX when the helper does not run.
export!(thiscall, rw_00890c80(this: u32) -> u32 {
    unsafe {
        let tag = ((this + 4) as *const u8).read();
        let node = if tag == 0xff {
            0
        } else {
            let stride = *global::<u32>(0x0115D968);
            let variant = ((this + 0x40) as *const u8).read() as u32;
            let base = *global::<u32>(0x0115D988);
            let row = ((base
                .wrapping_add(variant.wrapping_mul(0x6f40))
                .wrapping_add(0x6f14)) as *const u32)
                .read();
            stride.wrapping_mul(tag as u32).wrapping_add(row)
        };
        let owner = ((this + 0x74) as *const u32).read();
        let mut eax = *global::<u32>(0x0115D988);
        if owner != 0 {
            if ((((node + 0xe8) as *const u8).read()) & 0x20) == 0 {
                eax = callee_thiscall!(1, u32, owner);
            }
        }
        let flagp = (node + 0xe8) as *mut u8;
        flagp.write(flagp.read() | 0x20);
        eax
    }
});
