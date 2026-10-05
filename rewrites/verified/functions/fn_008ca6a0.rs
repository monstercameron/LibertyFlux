// original: 0x008CA6A0 stream_channel_open (proposed)

/// Opens streaming channel `mode` on the object in ECX: stores the mode at
/// `+0x08`, clears `+0x04` and `+0x00`. For modes 1, 2 and 3 pushes the
/// mode's name global and resolves it through the lookup callee (callee 0,
/// thiscall with one stack argument); when lookup succeeds, registers the
/// result through the register callee (callee 1, thiscall with one argument)
/// and, when registration succeeds, starts the channel through the start
/// callee (callee 2, thiscall with fourteen constant arguments). Any other
/// mode, a failed lookup or a failed registration ends the call quietly.
///
/// Thiscall with one stack argument; pops the argument (the callee pops 4 bytes); no result.
lf_checker_rt::export!(thiscall, rw_008CA6A0(this: u32, mode: u32) -> u32 {
    unsafe {
        /// Lookup callee id.
        const LOOKUP: u32 = 0;
        /// Register callee id.
        const REGISTER: u32 = 1;
        /// Start callee id.
        const START: u32 = 2;
        /// Lookup callee's object.
        const LOOKUP_THIS: u32 = 0x116BFF0;
        /// Register/start callee's object.
        const CHAN_THIS: u32 = 0x1033130;
        /// Name globals per mode 1, 2, 3.
        const NAME1: u32 = 0x1032114;
        const NAME2: u32 = 0x1032310;
        const NAME3: u32 = 0x1032314;
        ((this + 8) as *mut u32).write_unaligned(mode);
        ((this + 4) as *mut u8).write(0);
        (this as *mut u32).write_unaligned(0);
        let name_va = match mode {
            1 => NAME1,
            2 => NAME2,
            3 => NAME3,
            _ => return 0,
        };
        let name = (lf_checker_rt::relocated(name_va) as *const u32).read();
        let found: u32 =
            lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(LOOKUP_THIS), name);
        if found == 0 {
            return 0;
        }
        let chan = lf_checker_rt::relocated(CHAN_THIS);
        let reg: u32 = lf_checker_rt::callee_thiscall!(REGISTER, u32, chan, found);
        if reg & 0xFF == 0 {
            return 0;
        }
        let _s: u32 = lf_checker_rt::callee_thiscall!(
            START, u32, chan, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0xFFFF_FFFF
        );
        0
    }
});
