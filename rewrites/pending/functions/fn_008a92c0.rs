// original: 0x008a92c0 rage::audEffect::audEffect
/// `rage::audEffect` constructor: install the vtable and zero the fields.
///
/// Writes the (relocated) vtable pointer at `this+0`, zeroes the scalar and
/// pointer fields the original touches, sets the word at `this+0x70` to 1,
/// and returns `this`.
export!(thiscall, rw_008a92c0(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E7BC64;
        *(this as *mut u32) = relocated(VTABLE);
        *(this.add(0x04) as *mut u32) = 0;
        *(this.add(0x08) as *mut u32) = 0;
        *(this.add(0x20) as *mut u32) = 0;
        *(this.add(0x24) as *mut u32) = 0;
        *(this.add(0x28) as *mut u32) = 0;
        *(this.add(0x70) as *mut u16) = 1;
        this as u32
    }
});
