// original: 0x00e5e7e0 net_list_prepend_110ea80
/// Prepend the static node at 0x110EA80 to the list headed at 0x17AD1B8.
///
/// Stores the previous head into the node's link slot at 0x110EA8C,
/// installs the node as the new head, and returns the previous head,
/// matching the value the original leaves in EAX.
export!(cdecl, rw_00e5e7e0() -> u32 {
    unsafe {
        const HEAD: u32 = 0x17AD1B8;
        const LINK_SLOT: u32 = 0x110EA8C;
        const NODE: u32 = 0x110EA80;
        let old = *global::<u32>(HEAD);
        *global::<u32>(LINK_SLOT) = old;
        *global::<u32>(HEAD) = relocated(NODE);
        old
    }
});
