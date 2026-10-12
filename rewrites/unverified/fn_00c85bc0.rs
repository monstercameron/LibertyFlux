// original: 0x00C85BC0 physics_shape_list_construct

/// Construct a shape-list object from the supplied parent list. The base
/// constructor runs first, the receiver receives its vtable, and a list helper
/// is called before walking the receiver's array at `+0x210`. The unsigned
/// count at `+0x214` controls entries spaced `0x6C` bytes apart; each non-null
/// entry is initialized with the parent pointer. The receiver is returned.
lf_checker_rt::export!(thiscall, rw_00c85bc0(this: u32, parent_list: u32) -> u32 {
    const LIST_POINTER_OFFSET: u32 = 0x210;
    const LIST_COUNT_OFFSET: u32 = 0x214;
    const LIST_ELEMENT_STRIDE: u32 = 0x6C;
    const SHAPE_LIST_VTABLE: u32 = 0x00ED_63BC;
    const CONSTRUCT_BASE: u32 = 1;
    const INITIALIZE_LIST: u32 = 2;
    const INITIALIZE_ENTRY: u32 = 3;

    let _ = lf_checker_rt::callee_thiscall!(CONSTRUCT_BASE, u32, this, parent_list);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(SHAPE_LIST_VTABLE));
    }
    let list_storage = this.wrapping_add(LIST_POINTER_OFFSET);
    let _ = lf_checker_rt::callee_thiscall!(INITIALIZE_LIST, u32, parent_list, list_storage);
    let elements = unsafe { (list_storage as *const u32).read_unaligned() };
    let element_count = unsafe {
        (this.wrapping_add(LIST_COUNT_OFFSET) as *const u16).read_unaligned() as u32
    };
    for index in 0..element_count {
        let element = elements.wrapping_add(index.wrapping_mul(LIST_ELEMENT_STRIDE));
        if element != 0 {
            let _ = lf_checker_rt::callee_thiscall!(INITIALIZE_ENTRY, u32, element, parent_list);
        }
    }
    this
});
