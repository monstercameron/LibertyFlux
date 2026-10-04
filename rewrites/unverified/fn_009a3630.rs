// original: 0x009a3630 shared_stub_deleting_dtor
/// Deleting destructor of the radio audio entity.
///
/// Stamps the two table addresses, runs the teardown through callee 1,
/// frees the object through callee 2 when the low bit of the flag word is
/// set, and returns the object pointer.
export!(thiscall, rw_009a3630(this: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE_SUB: u32 = 0xe8e23c;
        const VTABLE_MAIN: u32 = 0xe83134;
        *((this.wrapping_add(8)) as *mut u32) = relocated(VTABLE_SUB);
        *(this as *mut u32) = relocated(VTABLE_MAIN);
        callee_thiscall!(1, u32, this);
        if (flags & 1) != 0 {
            callee_cdecl!(2, u32, this);
        }
        this
    }
});
