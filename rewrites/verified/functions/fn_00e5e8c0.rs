// original: 0x00e5e8c0 net_list_prepend_110eb00
/// Prepend the static node at 0x110EB00 to the list headed at 0x17ACD24.
///
/// Stores the previous head into the node's link slot at 0x110EB04,
/// installs the node as the new head, and returns the previous head,
/// matching the value the original leaves in EAX.
export!(cdecl, rw_00e5e8c0() -> u32 {
    unsafe {
        const HEAD: u32 = 0x17ACD24;
        const LINK_SLOT: u32 = 0x110EB04;
        const NODE: u32 = 0x110EB00;
        let old = *global::<u32>(HEAD);
        *global::<u32>(LINK_SLOT) = old;
        *global::<u32>(HEAD) = relocated(NODE);
        old
    }
});
