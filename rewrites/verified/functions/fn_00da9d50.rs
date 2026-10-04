// original: 0x00da9d50 flee_probe_check
/// Check whether the candidate pair passes the probe gate.
///
/// The second object must be non-null, carry kind bits 0x80 and state 2.
/// Then the check passes at once when the first object's inner object is
/// active, otherwise a probe call must succeed and the looked-up value must
/// exceed zero. Returns 1 on pass, 0 on fail.
export!(cdecl, rw_00da9d50(a: u32, b: u32) -> u32 {
    unsafe {
        /// Kind word and the required kind bits inside it.
        const KIND: u32 = 0x28;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_WANT: u32 = 0x80;
        /// State word and its required value.
        const STATE: u32 = 0x1304;
        const STATE_WANT: u32 = 2;
        /// Inner-object pointer and its activity flag offset.
        const INNER: u32 = 0x6c;
        const ACTIVE: u32 = 0x0e;
        /// Probe base pointer and its adjustment.
        const PROBE_BASE: u32 = 0x224;
        const PROBE_ADJ: u32 = 0x2e0;
        /// Probe arguments and the looked-up value's offset.
        const PROBE_ARG: u32 = 0x10c;
        const VALUE: u32 = 0x28;
        if b == 0 {
            return 0;
        }
        if ((b.wrapping_add(KIND)) as *const u32).read_unaligned() & KIND_MASK != KIND_WANT {
            return 0;
        }
        if ((b.wrapping_add(STATE)) as *const u32).read_unaligned() != STATE_WANT {
            return 0;
        }
        let inner = ((a.wrapping_add(INNER)) as *const u32).read_unaligned();
        if inner != 0 && (*((inner.wrapping_add(ACTIVE)) as *const u8)) != 0 {
            return 1;
        }
        let probe = ((a.wrapping_add(PROBE_BASE)) as *const u32).read_unaligned().wrapping_add(PROBE_ADJ);
        let ok: u32 = callee_thiscall!(1, u32, probe, PROBE_ARG, 0);
        // The original tests only the low byte of the probe's answer.
        if ok as u8 == 0 {
            return 0;
        }
        let d: u32 = callee_thiscall!(2, u32, b);
        if f32::from_bits(((d.wrapping_add(VALUE)) as *const u32).read_unaligned()) > 0.0 {
            1
        } else {
            0
        }
    }
});
