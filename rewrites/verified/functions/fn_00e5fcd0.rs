// original: 0x00e5fcd0 list_prepend_1110010
/// Prepend a static node onto the shared global list.
///
/// Stores the current list head into the node link word, then points the
/// head at this node. Returns the previous head value.
export!(cdecl, rw_00e5fcd0() -> u32 {
    unsafe {
        const HEAD: u32 = 0x17acd24;
        const SLOT: u32 = 0x1110010;
        let old = global::<u32>(HEAD).read();
        global::<u32>(SLOT + 4).write(old);
        global::<u32>(HEAD).write(relocated(SLOT));
        old
    }
});
