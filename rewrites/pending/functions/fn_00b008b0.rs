// original: 0x00b008b0 set_slot_pointer
/// Store a value into the indexed slot of the object's table.
export!(thiscall, rw_00b008b0(this: u32, index: u32, value: u32) -> u32 {
    unsafe {
        let table = *(this as *const u32) as *mut u32;
        *table.add(index as usize) = value;
        value
    }
});
