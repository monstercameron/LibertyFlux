// original: 0x0087bef0 guarded_construct_forward (proposed)
/// Null-guarded entry to the shared composite constructor.
///
/// A null pointer returns untouched (leaving an unstable return-register
/// value, which the contract does not compare). Any other pointer runs the
/// shared constructor body: tag word, cleared state, node table, a cleared
/// sub-object with its own table, then the sub-object initialiser invoked
/// through its table slot with the sub-object as receiver.
export!(cdecl, rw_0087bef0(obj: u32) -> u32 {
    /// Node tag word stored at offset 4.
    const TAG: u32 = 0x00150000;
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
        let p = obj as *mut u32;
        p.add(1).write(TAG);
        p.add(2).write(0);
        p.add(3).write(0);
        p.add(4).write(0);
        p.add(5).write(0);
        p.add(6).write(0);
        p.write(relocated(NODE_TABLE));
        let q = (obj + SUB_OFF) as *mut u32;
        q.write(relocated(SUB_TABLE));
        q.add(1).write(0);
        q.add(2).write(0);
        q.add(3).write(0);
        let _: u32 = callee_thiscall!(1, u32, obj + SUB_OFF);
    }
    obj
});
