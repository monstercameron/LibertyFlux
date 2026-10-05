// original: 0x00a454e0 vehicle_contains_occupant
/// True when the callee's id matches one of the nine occupant slots.
///
/// Calls the id provider (callee 1, thiscall, no stack arguments) with
/// `this`; a zero answer means false at once. Otherwise compares the answer
/// with the dword at `this+0xF50` and then the eight at `this+0xF54..`,
/// returning 1 on the first match, 0 when none matches (thiscall, no stack
/// arguments). Only AL is compared.
export!(thiscall, rw_00a454e0(this: u32) -> u32 {
    unsafe {
        const CALLEE: u32 = 1;
        const FIRST_OFF: u32 = 0xf50;
        const REST_OFF: u32 = 0xf54;
        const REST_N: u32 = 8;
        let id: u32 = callee_thiscall!(CALLEE, u32, this);
        if id == 0 {
            return 0;
        }
        if (this.wrapping_add(FIRST_OFF) as *const u32).read_unaligned() == id {
            return 1;
        }
        let mut p = this.wrapping_add(REST_OFF);
        let mut i = 0u32;
        while i < REST_N {
            if (p as *const u32).read_unaligned() == id {
                return 1;
            }
            p = p.wrapping_add(4);
            i += 1;
        }
        0
    }
});
