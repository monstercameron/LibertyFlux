// original: 0x00A4C780 vehicle_release_f50 (proposed)

/// Releases the slot at `this + SLOT` and clears it, adjusting flag bits first.
///
/// When the slot word is zero, returns at once. Otherwise: unless the flag
/// field (`[this + FLAGS]` (0x28) masked with `FIELD`, 0x7C00) already equals
/// `SKIP` (0xC00), and only when the argument byte is non-zero, clears
/// `CLEAR` (0x7400) and sets `SET` (0x800) in the flags. Then releases the old
/// slot value through the callee (`this` = old value, argument = slot
/// address) and zeroes the slot. Returns nothing defined (`eax` is the callee
/// answer on the release path and untouched incoming state otherwise, so the
/// contract compares no return channel).
///
/// Original: 0x00A4C780 (thiscall, one byte-ish stack word), one callee.
lf_checker_rt::export!(thiscall, rw_00A4C780(this: u32, arg: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x0F50;
        const FLAGS: u32 = 0x28;
        const FIELD: u32 = 0x7C00;
        const SKIP: u32 = 0x0C00;
        const CLEAR: u32 = 0x7400;
        const SET: u32 = 0x0800;
        const RELEASE_CALLEE: u32 = 1;
        let slot = (this + SLOT) as *mut u32;
        if slot.read_unaligned() == 0 {
            return 0;
        }
        let f = (this + FLAGS) as *mut u32;
        if f.read_unaligned() & FIELD != SKIP && (arg & 0xFF) != 0 {
            f.write_unaligned(f.read_unaligned() & !CLEAR | SET);
        }
        let old = slot.read_unaligned();
        lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, old, slot as u32);
        slot.write_unaligned(0);
        0
    }
});
