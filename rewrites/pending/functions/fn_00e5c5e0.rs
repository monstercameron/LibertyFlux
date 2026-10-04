// original: 0x00e5c5e0 link_static_node
/// Link one static record into its list: store the current head
/// value in the record's link slot, then head the list at the
/// record. Returns the previous head value.
export!(cdecl, rw_00e5c5e0() -> u32 {
    unsafe {
        let head = global::<u32>(0x017AD1B8);
        let slot = global::<u32>(0x0110DECC);
        let old = head.read();
        slot.write(old);
        head.write(relocated(0x0110DEC0));
        old
    }
});
