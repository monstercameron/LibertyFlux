// original: 0x00ade100 render_phase_light_context_select (proposed)

/// Select the active light context slots for a render phase.
///
/// `this` is the phase object. When the flag byte at `this+0x8e8` has
/// bit 0x10 set, the three context slots are taken from the object
/// (`this+0x94c/0x950`, each falling back to the global default when
/// zero; `this+0x948` selects between two sub-slots at `+0x8a0/0x890`
/// of the pointed-to record depending on a global mode flag, or the
/// default when null), and the refresh callee (id 1) runs. Otherwise
/// all three slots take the global default. The chosen values are
/// stored to the context globals, and a fifth global is refreshed
/// from one of two sources depending on bit 0x40 of the flag byte.
/// The fourth context slot takes the refresh callee's return value on
/// the object path (the call clobbers eax) and the default otherwise.
///
/// Edge cases: each object pointer is independently nullable; the
/// sub-slot choice reads the mode global only when the third pointer
/// is non-null.
///
/// Original: thiscall, no stack arguments, one direct callee (id 1,
/// thiscall, no stack arguments, object is the selected second slot),
/// five globals read, five written, returns nothing.
lf_checker_rt::export!(thiscall, rw_00ade100(this: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x8e8;
        const SLOT0: u32 = 0x94c;
        const SLOT1: u32 = 0x950;
        const SLOT2: u32 = 0x948;
        const SUB_A: u32 = 0x8a0;
        const SUB_B: u32 = 0x890;
        const USE_OBJECT: u8 = 0x10;
        const ALT_SOURCE: u8 = 0x40;
        const G_DEFAULT: u32 = 0x017ED954;
        const G_MODE: u32 = 0x01550DF4;
        const G_CTX0: u32 = 0x01550E90;
        const G_CTX1: u32 = 0x01550E94;
        const G_CTX2: u32 = 0x01550E98;
        const G_CTX3: u32 = 0x01550E9C;
        const G_OUT_A: u32 = 0x0166DA1C;
        const G_OUT_B: u32 = 0x0154E174;
        let flags = ((this + FLAGS) as *const u8).read();
        let default = (lf_checker_rt::global::<u32>(G_DEFAULT) as *const u32).read_unaligned();
        let fourth: u32;
        if flags & USE_OBJECT != 0 {
            let s0 = ((this + SLOT0) as *const u32).read_unaligned();
            let first = if s0 != 0 { s0 } else { default };
            (lf_checker_rt::global::<u32>(G_CTX0) as *mut u32).write_unaligned(first);
            let s1 = ((this + SLOT1) as *const u32).read_unaligned();
            let second = if s1 != 0 { s1 } else { default };
            (lf_checker_rt::global::<u32>(G_CTX1) as *mut u32).write_unaligned(second);
            let s2 = ((this + SLOT2) as *const u32).read_unaligned();
            if s2 != 0 {
                let mode = (lf_checker_rt::global::<u32>(G_MODE) as *const u32).read_unaligned();
                let off = if mode != 0 { SUB_A } else { SUB_B };
                let third = ((s2 + off) as *const u32).read_unaligned();
                (lf_checker_rt::global::<u32>(G_CTX2) as *mut u32).write_unaligned(third);
            } else {
                (lf_checker_rt::global::<u32>(G_CTX2) as *mut u32).write_unaligned(default);
            }
            // The refresh call clobbers eax, so the fourth slot takes the
            // callee's return value, not the third slot's value.
            fourth = lf_checker_rt::callee_thiscall!(1, u32, second);
        } else {
            (lf_checker_rt::global::<u32>(G_CTX0) as *mut u32).write_unaligned(default);
            (lf_checker_rt::global::<u32>(G_CTX1) as *mut u32).write_unaligned(default);
            (lf_checker_rt::global::<u32>(G_CTX2) as *mut u32).write_unaligned(default);
            fourth = default;
        }
        (lf_checker_rt::global::<u32>(G_CTX3) as *mut u32).write_unaligned(fourth);
        let out_a = (lf_checker_rt::global::<u32>(G_OUT_A) as *const u32).read_unaligned();
        let picked = if flags & ALT_SOURCE != 0 {
            (lf_checker_rt::global::<u32>(G_OUT_B) as *const u32).read_unaligned()
        } else {
            out_a
        };
        (lf_checker_rt::global::<u32>(G_OUT_A) as *mut u32).write_unaligned(picked);
    }
    0
});
