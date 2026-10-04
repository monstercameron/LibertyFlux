// original: 0x00e5eca0 net_list_prepend_110eb98
/// Prepend the static node at 0x110EB98 to the list headed at 0x17AD1B8.
///
/// Stores the previous head into the node's link slot at 0x110EBA4,
/// installs the node as the new head, and returns the previous head,
/// matching the value the original leaves in EAX.
export!(cdecl, rw_00e5eca0() -> u32 {
    unsafe {
        const HEAD: u32 = 0x17AD1B8;
        const LINK_SLOT: u32 = 0x110EBA4;
        const NODE: u32 = 0x110EB98;
        let old = *global::<u32>(HEAD);
        *global::<u32>(LINK_SLOT) = old;
        *global::<u32>(HEAD) = relocated(NODE);
        old
    }
});
