// original: 0x00a35220 vehicle_node_attach (proposed)

/// Attach a fresh node (from the allocator callee, id 1) to a record.
///
/// A null allocator answer ends the call with 0. Otherwise the node gets
/// `val` at `+0`, a global stamp word at `+0x1c`, and its flag word at
/// `+0x20` gets bit 0 set to "record head non-zero". When that bit ends up
/// set, six words are copied from the record (`+0x10`..`+0x28`, skipping
/// `+0x1c`) into the node (`+0x4`..`+0x18`). Cdecl/2, returns the node.
lf_checker_rt::export!(cdecl, rw_00a35220(val: u32, rec: u32) -> u32 {
    unsafe {
        const STAMP: u32 = 0x0117_35B4;
        const FLAG: u32 = 0x20;
        const ALLOC: u32 = 1;
        const SRC: [u32; 6] = [0x10, 0x14, 0x18, 0x20, 0x24, 0x28];
        let node: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32,);
        if node == 0 {
            return 0;
        }
        core::ptr::write_unaligned(node as *mut u32, val);
        let stamp = core::ptr::read(lf_checker_rt::global::<u32>(STAMP));
        core::ptr::write_unaligned((node + 0x1C) as *mut u32, stamp);
        let bit = (core::ptr::read_unaligned(rec as *const u32) != 0) as u32;
        let flags = core::ptr::read_unaligned((node + FLAG) as *const u32);
        let flags = flags ^ ((bit ^ flags) & 1);
        core::ptr::write_unaligned((node + FLAG) as *mut u32, flags);
        if flags & 1 != 0 {
            for (i, off) in SRC.iter().enumerate() {
                let v = core::ptr::read_unaligned((rec + off) as *const u32);
                core::ptr::write_unaligned((node + 4 + i as u32 * 4) as *mut u32, v);
            }
        }
        node
    }
});
