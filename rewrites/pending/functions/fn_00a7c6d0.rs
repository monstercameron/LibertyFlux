// original: 0x00a7c6d0 chain_contains_target
// Chain membership test: is `target` reachable from `this` via +0x118?
//
// Every visited node must carry flag bits 2 and 3 (+0x13c & 0x0C); the
// first node that fails the test ends the walk with 0. Returns 1 when
// the walk reaches `target`, 0 on a flag failure or a null link.
export!(thiscall, rw_s13_00a7c6d0(this: *const u8, target: u32) -> u8 {
    unsafe {
        let mut node = this;
        loop {
            let flags = *node.add(0x13C);
            if flags & 0x04 == 0 || flags & 0x08 == 0 {
                return 0;
            }
            let next = *(node.add(0x118) as *const u32);
            if next == target {
                return 1;
            }
            if next == 0 {
                return 0;
            }
            node = next as *const u8;
        }
    }
});
