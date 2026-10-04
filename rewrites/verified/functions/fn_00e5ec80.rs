// original: 0x00e5ec80 net_list_prepend_110eb78
/// Prepend the static node at 0x110EB78 to the list headed at 0x17AD1B8.
///
/// Stores the previous head into the node's link slot at 0x110EB84,
/// installs the node as the new head, and returns the previous head,
/// matching the value the original leaves in EAX.
export!(cdecl, rw_00e5ec80() -> u32 {
    unsafe {
        const HEAD: u32 = 0x17AD1B8;
        const LINK_SLOT: u32 = 0x110EB84;
        const NODE: u32 = 0x110EB78;
        let old = *global::<u32>(HEAD);
        *global::<u32>(LINK_SLOT) = old;
        *global::<u32>(HEAD) = relocated(NODE);
        old
    }
});
