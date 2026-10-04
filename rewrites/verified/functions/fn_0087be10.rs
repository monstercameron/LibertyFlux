// original: 0x0087be10 composite_teardown (proposed)
/// Two-phase teardown of a composite node and its sub-object.
///
/// First the node table is re-attached and the sub-object's owner hook runs
/// if an owner link is present, then the base teardown runs and the state
/// words are cleared; the same sequence repeats for the sub-object with its
/// own table and terminal marker before the final base teardown. Returns
/// the last teardown answer.
export!(thiscall, rw_0087be10(this: u32) -> u32 {
    /// Node table re-attached at teardown start (file VA).
    const NODE_TABLE: u32 = 0x00FE84C4;
    /// Sub-object table used during teardown (file VA).
    const SUB_TABLE: u32 = 0x00FE7FB4;
    /// Terminal marker stamped on the sub-object (file VA).
    const TERMINAL: u32 = 0x00E86AFC;
    /// Final node table (file VA).
    const FINAL_TABLE: u32 = 0x00FE7F28;
    /// Sub-object offset within the node.
    const SUB_OFF: u32 = 0x1c;
    unsafe {
        let sub = this + SUB_OFF;
        (this as *mut u32).write(relocated(NODE_TABLE));
        let owner = ((sub + 8) as *const u32).read();
        if owner != 0 {
            let _: u32 = callee_thiscall!(1, u32, owner, sub);
        }
        let _: u32 = callee_thiscall!(2, u32, this);
        ((this + 0x0c) as *mut u32).write(0);
        ((this + 0x10) as *mut u32).write(0);
        if ((this + 0x08) as *const u32).read() != 0 {
            ((this + 0x08) as *mut u32).write(0);
        }
        let owner2 = ((sub + 8) as *const u32).read();
        (sub as *mut u32).write(relocated(SUB_TABLE));
        if owner2 != 0 {
            let _: u32 = callee_thiscall!(1, u32, owner2, sub);
        }
        (sub as *mut u32).write(relocated(TERMINAL));
        (this as *mut u32).write(relocated(FINAL_TABLE));
        let answer: u32 = callee_thiscall!(2, u32, this);
        ((this + 0x0c) as *mut u32).write(0);
        ((this + 0x10) as *mut u32).write(0);
        if ((this + 0x08) as *const u32).read() != 0 {
            ((this + 0x08) as *mut u32).write(0);
        }
        answer
    }
});
