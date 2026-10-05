// original: 0x00A4D860 vehicle_spawn_copy (proposed)

/// Returns the object in the slot at `this + SLOT`, creating it when empty.
///
/// When the slot word is non-zero it is returned at once. Otherwise calls the
/// creating callee (cdecl, `(this, 1, 0, 1, arg, 0)`); a null answer returns
/// 0. The callee is inferred to store the new object into the slot (the
/// function returns that slot at the end, after finding it empty), which the
/// contract models as an out-param write of the returned pointer. With a
/// created object: when bit 4 of the flag byte at `this + FLAG` (0x0F20) is
/// set, asks the second callee (thiscall, object in `ecx`) and, on a non-zero
/// low byte, runs the third (cdecl, `(object, 1, 0, 0)`). Copies the tag byte
/// at `this + TAG` (0x63) onto the object and returns the slot word.
///
/// Original: 0x00A4D860 (thiscall, one stack word), three callees.
lf_checker_rt::export!(thiscall, rw_00A4D860(this: u32, arg: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x0F50;
        const FLAG: u32 = 0x0F20;
        const TAG: u32 = 0x63;
        const MARK: u8 = 0x10;
        const CREATE_CALLEE: u32 = 1;
        const ASK_CALLEE: u32 = 2;
        const RUN_CALLEE: u32 = 3;
        let slot = (this + SLOT) as *const u32;
        if slot.read_unaligned() != 0 {
            return slot.read_unaligned();
        }
        let obj: u32 =
            lf_checker_rt::callee_cdecl!(CREATE_CALLEE, u32, this, 1, 0, 1, arg, 0);
        if obj == 0 {
            return 0;
        }
        if ((this + FLAG) as *const u8).read() & MARK != 0 {
            let ans: u32 = lf_checker_rt::callee_thiscall!(ASK_CALLEE, u32, obj);
            if (ans & 0xFF) != 0 {
                lf_checker_rt::callee_cdecl!(RUN_CALLEE, u32, obj, 1, 0, 0);
            }
        }
        ((obj + TAG) as *mut u8).write(((this + TAG) as *const u8).read());
        slot.read_unaligned()
    }
});
