// original: 0x00ad0160 Wheel_RepairTyre
/// Field initialiser: write a constant pair, then hand off to shared logic.
///
/// Takes the object pointer in ECX. Writes the same single-precision
/// constant into the two adjacent words late in the object, then transfers
/// control to the shared continuation with the same object pointer,
/// returning whatever that call answers.
export!(thiscall, rw_00ad0160(this: u32) -> u32 {
    unsafe {
        *(this.wrapping_add(0x15C) as *mut u32) = 0x447A0000;
        *(this.wrapping_add(0x160) as *mut u32) = 0x447A0000;
        callee_thiscall!(1, u32, this)
    }
});
