// original: 0x00a351d0 vehicle_node_rebuild (proposed)

/// Rebuild the node linked at `+0x10` and merge flag bits into the result.
///
/// A null link ends the call (the contract fixes entry EAX to 0, which the
/// original preserves on this path). Otherwise, when the flag byte at
/// `+0x18` has bit 0, the triple builder (id 1, cdecl/3) runs on
/// `(link, obj[0x14], obj + 0x20)`; else the pair builder (id 2, cdecl/2)
/// runs on `(link, obj + 0x20)`. The link is cleared, and a non-null build
/// result gets bits `0x1fe` of `obj[0x18] ^ result[0x20]` folded into its
/// `+0x20` word. Cdecl/1, returns EAX (0 on the first two paths, the
/// folded mask on the third).
lf_checker_rt::export!(cdecl, rw_00a351d0(obj: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x10;
        const AUX: u32 = 0x14;
        const FLAGS: u32 = 0x18;
        const EXTRA: u32 = 0x20;
        const MASK: u32 = 0x1FE;
        const TRIPLE: u32 = 1;
        const PAIR: u32 = 2;
        let link = core::ptr::read_unaligned((obj + LINK) as *const u32);
        if link == 0 {
            return 0;
        }
        let flags = core::ptr::read((obj + FLAGS) as *const u8);
        let out: u32 = if flags & 1 != 0 {
            let aux = core::ptr::read_unaligned((obj + AUX) as *const u32);
            lf_checker_rt::callee_cdecl!(TRIPLE, u32, link, aux, obj.wrapping_add(EXTRA))
        } else {
            lf_checker_rt::callee_cdecl!(PAIR, u32, link, obj.wrapping_add(EXTRA))
        };
        core::ptr::write_unaligned((obj + LINK) as *mut u32, 0);
        if out == 0 {
            return 0;
        }
        let mask = (core::ptr::read_unaligned((obj + FLAGS) as *const u32)
            ^ core::ptr::read_unaligned((out + EXTRA) as *const u32))
            & MASK;
        let f = core::ptr::read_unaligned((out + EXTRA) as *const u32);
        core::ptr::write_unaligned((out + EXTRA) as *mut u32, f ^ mask);
        mask
    }
});
