// original: 0x00b53f10 rage::atDNode<CAnimPlayer, CAtdVirtualBase>::vf0
/// Scalar deleting destructor: stamps the node vtable, destroys the
/// embedded player member, stamps the base vtable, and frees `this`
/// when bit 0 of the flag is set. Returns `this`.
export!(thiscall, rw_00b53f10(this: *mut u8, flag: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xEAF3A0);
        callee_thiscall!(1, u32, (this as u32).wrapping_add(4));
        *(this as *mut u32) = relocated(0xEAF398);
        if flag & 1 != 0 {
            callee_cdecl!(2, u32, this as u32);
        }
        this as u32
    }
});
