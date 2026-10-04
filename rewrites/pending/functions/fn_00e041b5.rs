// original: 0x00e041b5 set_ptr_slot_294
/// Stores the argument in the global slot at 0x17AC294 and returns it.
export!(cdecl, rw_00e041b5(value: u32) -> u32 {
    unsafe {
        *global::<u32>(0x17AC294) = value;
        value
    }
});
