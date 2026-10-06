// original: 0x009055a0 input_global_reset (proposed)
/// Reset the input subsystem's static state and re-register its callbacks.
///
/// Registers the three code addresses with the registrar callee (a thiscall
/// on the static registrar with `(data, v1, v2)`), runs the prober callee,
/// then zeroes the counter words, the serial word, the gauge words and the
/// latch words, copies the flag byte of table object `index` into the
/// static output byte, and stores the below-frame scratch float (zero under
/// the contract's stack fill) into the static float slot. Finally runs the
/// settle callee on `(0, 0)` and returns its answer. Cdecl, no arguments.
export!(cdecl, rw_009055a0() -> u32 {
    unsafe {
        /// Static registrar object (file VA).
        const REGISTRAR: u32 = 0x012BCD18;
        /// Registered addresses (file VAs).
        const REG_A: u32 = 0x00E84B38;
        const REG_B: u32 = 0x009057F0;
        const REG_C: u32 = 0x009083C0;
        /// Settle arguments (file VAs).
        const SET_A: u32 = 0x01193C5C;
        const SET_B: u32 = 0x00E84B40;
        const SET_C: u32 = 8;
        /// Index word and object table for the flag byte (file VAs).
        const INDEX: u32 = 0x0118EA08;
        const OTABLE: u32 = 0x0118E7F8;
        const FLAG_OFF: u32 = 0x58;
        /// Static words zeroed (file VAs).
        const CNT_A: u32 = 0x01034490;
        const CNT_B: u32 = 0x01034498;
        const CNT_C: u32 = 0x0118F4E0;
        const GAUGE_A: u32 = 0x0118F4C0;
        const GAUGE_B: u32 = 0x0118F4C4;
        const LATCH_A: u32 = 0x01191260;
        const LATCH_B: u32 = 0x01191264;
        const LATCH_C: u32 = 0x01191268;
        /// Static serial halfword, output byte and float slot (file VAs).
        const SERIAL: u32 = 0x0118F4DC;
        const OUT: u32 = 0x010344D8;
        const FSLOT: u32 = 0x0119126C;
        const REG_ID: u32 = 1;
        const PROBE_ID: u32 = 2;
        const SET_ID: u32 = 3;
        const SETTLE_ID: u32 = 4;
        let _: u32 = callee_thiscall!(
            REG_ID,
            u32,
            relocated(REGISTRAR),
            relocated(REG_A),
            relocated(REG_B),
            relocated(REG_C)
        );
        let _: u32 = callee_cdecl!(PROBE_ID, u32,);
        let ix = (global::<u32>(INDEX)).read_unaligned();
        let obj = ((relocated(OTABLE).wrapping_add(ix.wrapping_mul(4))) as *const u32)
            .read_unaligned();
        (global::<u32>(CNT_A)).write_unaligned(0);
        (global::<u32>(CNT_B)).write_unaligned(0);
        (global::<u8>(OUT)).write(((obj.wrapping_add(FLAG_OFF)) as *const u8).read());
        (global::<u32>(CNT_C)).write_unaligned(0);
        let _: u32 = callee_cdecl!(SET_ID, u32, relocated(SET_A), relocated(SET_B), SET_C);
        (global::<u16>(SERIAL)).write_unaligned(0);
        (global::<u32>(GAUGE_A)).write_unaligned(0);
        (global::<u32>(GAUGE_B)).write_unaligned(0);
        (global::<u32>(LATCH_A)).write_unaligned(0);
        (global::<u32>(LATCH_B)).write_unaligned(0);
        (global::<u32>(LATCH_C)).write_unaligned(0);
        // Below-frame scratch float: zero under the contract's stack fill.
        (global::<u32>(FSLOT)).write_unaligned(0);
        callee_cdecl!(SETTLE_ID, u32, 0u32, 0u32)
    }
});
