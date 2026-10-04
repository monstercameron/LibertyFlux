// original: 0x00cc74b0 subobject_zero_b
/// Zero the nine scattered state words of the second sub-object.
/// Returns the object pointer.
export!(thiscall, rw_00cc74b0(this: *mut u8) -> u32 {
    unsafe {
        for off in [0usize, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x38] {
            *(this.add(off) as *mut u32) = 0;
        }
        this as u32
    }
});
