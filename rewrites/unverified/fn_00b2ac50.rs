// original: 0x00b2ac50 obj_mode_setup

/// Sets an object's mode words from a probe result or a sense byte.
///
/// Calls the probe callee with the object. When it answers 0, calls the
/// default callee with the object, writes the default mode words and returns
/// the default callee's answer. Otherwise writes the active mode words, then
/// reads a signed sense byte 0x1F0B past `a0`, scales it by 15
/// (shift-by-4 minus itself, exact for a byte) and writes the scaled value
/// plus 0x28 and plus 0x14 as the two halves of the first mode word.
/// Returns the scaled value plus 0x14. Cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_00b2ac50(a0: u32, obj: u32) -> u32 {
    unsafe {
        const MODE_WORD: u32 = 0x20;
        const KIND_WORD: u32 = 0x26;
        const FLAG_BYTE: u32 = 0x28;
        const SENSE_OFF: u32 = 0x1F0B;
        const DFLT_MODE: u32 = 0x00C8_00C8;
        const DFLT_KIND: u16 = 0x0A04;
        const ACT_MODE: u32 = 0x0140_0028;
        const ACT_KIND: u16 = 0x4607;
        const PROBE: u32 = 0;
        const DEFAULT: u32 = 1;
        let r = lf_checker_rt::callee_thiscall!(PROBE, u32, obj);
        if r == 0 {
            let r1 = lf_checker_rt::callee_thiscall!(DEFAULT, u32, obj);
            ((obj + KIND_WORD) as *mut u16).write_unaligned(DFLT_KIND);
            ((obj + FLAG_BYTE) as *mut u8).write(0);
            ((obj + MODE_WORD) as *mut u32).write_unaligned(DFLT_MODE);
            r1
        } else {
            ((obj + MODE_WORD) as *mut u32).write_unaligned(ACT_MODE);
            ((obj + KIND_WORD) as *mut u16).write_unaligned(ACT_KIND);
            let c = (((a0 + SENSE_OFF) as *const u8).read() as i8) as i16;
            let t = c.wrapping_mul(15);
            let u = t as u16 as u32;
            ((obj + MODE_WORD) as *mut u16).write_unaligned(u.wrapping_add(0x28) as u16);
            ((obj + MODE_WORD + 2) as *mut u16).write_unaligned(u.wrapping_add(0x14) as u16);
            u.wrapping_add(0x14)
        }
    }
});
