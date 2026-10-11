// original: 0x00694DD0 component_node_ctor_00694DD0 (proposed)

/// Initialize the three-word component header at `this`.
///
/// The first word becomes the component's relocated vtable pointer and the
/// next two words are cleared. The function returns the original object
/// pointer. The constructor takes its object in ECX and has no stack args.
lf_checker_rt::export!(thiscall, rw_00694DD0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00FE397C;
        const SECOND_WORD: u32 = 4;
        const THIRD_WORD: u32 = 8;
        let object = this as *mut u8;
        (object as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (object.add(SECOND_WORD as usize) as *mut u32).write_unaligned(0);
        (object.add(THIRD_WORD as usize) as *mut u32).write_unaligned(0);
        this
    }
});
