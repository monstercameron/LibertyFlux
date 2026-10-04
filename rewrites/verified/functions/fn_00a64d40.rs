// original: 0x00a64d40 PedSetDecisionMakerIndex
/// Stores the given value into the object field at +0xD0.
export!(thiscall, rw_00a64d40(this: u32, value: u32) -> u32 {
    unsafe {
        *((this + 0xD0) as *mut u32) = value;
    }
    0
});
