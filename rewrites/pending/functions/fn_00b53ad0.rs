// original: 0x00b53ad0 dual_embedded_ctor
/// Constructor: builds two embedded sub-objects of the same shape in
/// sequence (callee constructs, then vtable stamp, zeroed word and cleared
/// flag byte), zeroes two header words, constructs a third sub-object,
/// and returns `this`.
export!(thiscall, rw_00b53ad0(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(0xEAF2DC);
        *(this.add(0x14) as *mut u32) = 0;
        *this.add(0x20) = 0;
        callee_thiscall!(1, u32, (this as u32).wrapping_add(0x24));
        *(this.add(0x24) as *mut u32) = relocated(0xEAF2DC);
        *(this.add(0x38) as *mut u32) = 0;
        *this.add(0x44) = 0;
        *(this.add(0x48) as *mut u32) = 0;
        *(this.add(0x4C) as *mut u32) = 0;
        callee_thiscall!(2, u32, (this as u32).wrapping_add(0x58));
        this as u32
    }
});
