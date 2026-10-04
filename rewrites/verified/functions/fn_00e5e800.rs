// original: 0x00e5e800 net_list_prepend_110ea60
/// Prepend the static node at 0x110EA60 to the list headed at 0x17AD1B8.
///
/// Stores the previous head into the node's link slot at 0x110EA6C,
/// installs the node as the new head, and returns the previous head,
/// matching the value the original leaves in EAX.
export!(cdecl, rw_00e5e800() -> u32 {
    unsafe {
        const HEAD: u32 = 0x17AD1B8;
        const LINK_SLOT: u32 = 0x110EA6C;
        const NODE: u32 = 0x110EA60;
        let old = *global::<u32>(HEAD);
        *global::<u32>(LINK_SLOT) = old;
        *global::<u32>(HEAD) = relocated(NODE);
        old
    }
});
