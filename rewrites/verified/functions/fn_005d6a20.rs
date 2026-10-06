// original: 0x005d6a20 html_element_node_construct (proposed)

/// Construct an HTML element node in place.
///
/// Stamps the base vtable, zeroes the child-list head (`+0x08` pointer,
/// `+0x0c` array, `+0x10` count/capacity word), runs the string-table
/// initialiser callee on the embedded member at `+0x14`, then fills the
/// element fields: the caller-supplied tag id at `+0xdc`, null link slots at
/// `+0xe0`/`+0xe4`, unset sibling indexes (`-1`) at `+0xe8`/`+0xec`, node
/// kind `3` at `+0x04`, and finally the element vtable. Returns the object.
///
/// Original: 0x005d6a20 (thiscall, one stack word: the tag id).
lf_checker_rt::export!(thiscall, rw_005d6a20(this: u32, tag: u32) -> u32 {
    unsafe {
        const BASE_VTABLE: u32 = 0x00FE0B94;
        const ELEM_VTABLE: u32 = 0x00FE0B58;
        const MEMBER_OFF: u32 = 0x14;
        const TAG_OFF: u32 = 0xdc;
        const KIND_TEXT: u32 = 3;
        const NO_INDEX: u32 = 0xffff_ffff;
        const INIT_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(this, lf_checker_rt::relocated(BASE_VTABLE));
        wr32(this + 0x08, 0);
        wr32(this + 0x0c, 0);
        wr32(this + 0x10, 0);
        lf_checker_rt::callee_thiscall!(INIT_CALLEE, u32, this + MEMBER_OFF);
        wr32(this + 0xe4, 0);
        wr32(this + 0xe0, 0);
        wr32(this + TAG_OFF, tag);
        wr32(this, lf_checker_rt::relocated(ELEM_VTABLE));
        wr32(this + 0xe8, NO_INDEX);
        wr32(this + 0xec, NO_INDEX);
        wr32(this + 0x04, KIND_TEXT);
        this
    }
});
