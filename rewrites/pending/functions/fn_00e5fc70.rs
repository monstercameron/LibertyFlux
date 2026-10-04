// original: 0x00e5fc70 list_prepend_110ff50
/// Prepend a static node onto the shared global list.
///
/// Stores the current list head into the node link word, then points the
/// head at this node. Returns the previous head value.
export!(cdecl, rw_00e5fc70() -> u32 {
    unsafe {
        const HEAD: u32 = 0x17acd24;
        const SLOT: u32 = 0x110ff50;
        let old = global::<u32>(HEAD).read();
        global::<u32>(SLOT + 4).write(old);
        global::<u32>(HEAD).write(relocated(SLOT));
        old
    }
});
