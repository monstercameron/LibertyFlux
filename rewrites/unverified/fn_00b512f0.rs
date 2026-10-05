// original: 0x00b512f0 assign_ped_to_slot (proposed)

/// Reset an event-list header and attach a ped record to it.
///
/// The header is reset (callee 1, thiscall on `this`), the ped record `a`
/// is stored at `+0x0C` and registered (callee 2, stdcall with the slot
/// address; the call leaves no meaningful register behind). Then `b` and
/// `c` become the head and tag, generation and state are cleared, and the
/// ped's vehicle (at `a + 0x224`) gets the low word of `b` as its model id
/// (`+0x2EC`) with a zero sub-model byte (`+0x2EE`). Returns the head word
/// with its low half replaced by `b`'s, as the original's partial register
/// update does.
///
/// Original: 0x00b512f0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00b512f0(this: u32, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x00;
        const TAG: u32 = 0x04;
        const GENERATION: u32 = 0x08;
        const PAYLOAD: u32 = 0x0c;
        const STATE: u32 = 0x10;
        const RESET: u32 = 1;
        const REGISTER: u32 = 2;
        const VEHICLE: u32 = 0x224;
        const MODEL_ID: u32 = 0x2ec;
        const SUBMODEL: u32 = 0x2ee;
        lf_checker_rt::callee_thiscall!(RESET, u32, this);
        let slot = this + PAYLOAD;
        (slot as *mut u32).write_unaligned(a);
        lf_checker_rt::callee_stdcall!(REGISTER, u32, slot);
        ((this + HEAD) as *mut u32).write_unaligned(b);
        ((this + TAG) as *mut u32).write_unaligned(c);
        ((this + STATE) as *mut u32).write_unaligned(0);
        ((this + GENERATION) as *mut u32).write_unaligned(0);
        let inner = (slot as *const u32).read_unaligned();
        let veh = ((inner + VEHICLE) as *const u32).read_unaligned();
        ((veh + MODEL_ID) as *mut u16).write_unaligned(b as u16);
        ((veh + SUBMODEL) as *mut u8).write(0);
        (a & 0xffff_0000) | (b & 0x0000_ffff)
    }
});
