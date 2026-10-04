// original: 0x00e041a8 set_ptr_slot_290
/// Stores the argument in the global slot at 0x17AC290 and returns it.
export!(cdecl, rw_00e041a8(value: u32) -> u32 {
    unsafe {
        *global::<u32>(0x17AC290) = value;
        value
    }
});
