// original: 0x00e0419b set_ptr_slot_28c
/// Stores the argument in the global slot at 0x17AC28C and returns it.
///
/// The slot holds an encoded pointer consumed by the decode-and-call
/// dispatcher; this setter just records the raw value.
export!(cdecl, rw_00e0419b(value: u32) -> u32 {
    unsafe {
        *global::<u32>(0x17AC28C) = value;
        value
    }
});
