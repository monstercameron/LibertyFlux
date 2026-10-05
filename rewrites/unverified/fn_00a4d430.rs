// original: 0x00A4D430 vehicle_set_slot4 (proposed)

/// Releases the old word at `this + SLOT` and installs the argument in it.
///
/// When the slot is non-zero it is released through the first callee (`this`
/// = old value, argument = slot address) and zeroed; then the argument is
/// stored and the slot is registered through the second callee (`this` = the
/// new value, argument = slot address). Returns the second callee's answer.
///
/// Original: 0x00A4D430 (thiscall, one stack word), two callees in order.
lf_checker_rt::export!(thiscall, rw_00A4D430(this: u32, val: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x04;
        const RELEASE_CALLEE: u32 = 1;
        const REGISTER_CALLEE: u32 = 2;
        let slot = (this + SLOT) as *mut u32;
        let old = slot.read_unaligned();
        if old != 0 {
            lf_checker_rt::callee_thiscall!(
                RELEASE_CALLEE,
                u32,
                old,
                slot as u32
            );
            slot.write_unaligned(0);
        }
        slot.write_unaligned(val);
        lf_checker_rt::callee_thiscall!(REGISTER_CALLEE, u32, val, slot as u32)
    }
});
