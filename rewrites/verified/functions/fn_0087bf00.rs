// original: 0x0087bf00 init_with_addref (proposed)
/// Initialise a composite node and take one reference on its sub-object.
///
/// A null pointer returns untouched (leaving an unstable return-register
/// value, which the contract does not compare). Otherwise the node table
/// and a cleared sub-object are stamped, and the sub-object's reference
/// entry runs: it only bumps the count word the constructor just zeroed,
/// so the net effect is a count of one, written directly.
export!(cdecl, rw_0087bf00(obj: u32) -> u32 {
    /// Node behaviour table (file VA).
    const NODE_TABLE: u32 = 0x00FE84C4;
    /// Sub-object behaviour table (file VA).
    const SUB_TABLE: u32 = 0x00FE7FB4;
    /// Sub-object offset within the node.
    const SUB_OFF: u32 = 0x1c;
    if obj == 0 {
        return 0;
    }
    unsafe {
        (obj as *mut u32).write(relocated(NODE_TABLE));
        let q = (obj + SUB_OFF) as *mut u32;
        q.write(relocated(SUB_TABLE));
        // Zeroed by the constructor, then bumped to one by the reference
        // entry the original tail-jumps to; the store is the net effect.
        q.add(1).write(1);
        q.add(2).write(0);
        q.add(3).write(0);
    }
    relocated(SUB_TABLE)
});
