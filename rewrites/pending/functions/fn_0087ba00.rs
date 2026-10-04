// original: 0x0087ba00 mirror_node_construct (proposed)
/// Construct a mirror node at the given address (thiscall constructor).
///
/// Tag word, cleared state words, attached mirror-node behaviour table.
/// Returns `this`.
export!(thiscall, rw_0087ba00(this: u32) -> u32 {
    /// Node tag word stored at offset 4.
    const TAG: u32 = 0x00080000;
    /// Mirror-node behaviour table (file VA).
    const MIRROR_TABLE: u32 = 0x00FE844C;
    unsafe {
        let p = this as *mut u32;
        p.add(1).write(TAG);
        p.add(2).write(0);
        p.add(3).write(0);
        p.add(4).write(0);
        p.add(5).write(0);
        p.add(6).write(0);
        p.add(7).write(0);
        p.write(relocated(MIRROR_TABLE));
        p.add(8).write(0);
    }
    this
});
