// original: 0x00e5c300 node_register_dd48
/// Pushes the fixed node at 0x0110dd48 onto the intrusive list headed
/// at 0x017ad16c: the old head is saved in the node's next slot (+0x10)
/// and the head is updated. Returns the previous head.
export!(cdecl, rw_00e5c300() -> u32 {
    unsafe {
        const HEAD: u32 = 0x017AD16C;
        const NODE: u32 = 0x0110DD48;
        const LINK_SLOT: u32 = 0x0110DD58;
        let head = global::<u32>(HEAD);
        let old = *head;
        *global::<u32>(LINK_SLOT) = old;
        *head = relocated(NODE);
        old
    }
});
