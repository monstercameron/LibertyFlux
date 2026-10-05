// original: 0x00ab35f0 stream_release_nodes (proposed)

/// Release the two node chains hanging off a streaming object.
///
/// Walks the chain at `+0x40`, unlinking each node through the unlink
/// callee on the sub-object at `+0x3C` and releasing it through the release
/// callee on the loader singleton; then does the same for the chain at
/// `+0x34` with the sub-object at `+0x30` and the second release callee.
/// No return value.
///
/// Callees: 1 = unlink (thiscall, one word), 2 = first release (thiscall,
/// one word), 3 = unlink (thiscall, one word), 4 = second release
/// (thiscall, one word).
///
/// Original: 0x00ab35f0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ab35f0(this: u32) -> u32 {
    unsafe {
        const UNLINK_A: u32 = 1;
        const RELEASE_A: u32 = 2;
        const UNLINK_B: u32 = 3;
        const RELEASE_B: u32 = 4;
        const SINGLETON: u32 = 0x013B_ABA0;
        const CHAIN_A: u32 = 0x40;
        const SUB_A: u32 = 0x3C;
        const CHAIN_B: u32 = 0x34;
        const SUB_B: u32 = 0x30;
        let loader = lf_checker_rt::relocated(SINGLETON);
        let mut node = ((this + CHAIN_A) as *const u32).read_unaligned();
        while node != 0 {
            let next = (node as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(UNLINK_A, u32, this.wrapping_add(SUB_A), node);
            lf_checker_rt::callee_thiscall!(RELEASE_A, u32, loader, node);
            node = next;
        }
        node = ((this + CHAIN_B) as *const u32).read_unaligned();
        while node != 0 {
            let next = (node as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(UNLINK_B, u32, this.wrapping_add(SUB_B), node);
            lf_checker_rt::callee_thiscall!(RELEASE_B, u32, loader, node);
            node = next;
        }
        0
    }
});
