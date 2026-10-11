// original: 0x009BB0B0 CCamScriptInstruction_SetViewport::vf2

/// Apply the viewport rectangle held by this instruction.
///
/// The instruction key is at `+0x08`; four f32 values at `+0x0C` through
/// `+0x18` are forwarded unchanged, followed by the constant pair `0.0, 1.0`.
/// The camera manager lookup returns an object whose subobject at `+0x10` is
/// the receiver of the rectangle helper. Both helper calls are intercepted;
/// the second call's six stack words and result are compared exactly as bits.
/// A null lookup still produces the original receiver value `0x10`.
lf_checker_rt::export!(thiscall, rw_009bb0b0(this: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x0128_e400;
        const LOOKUP: u32 = 1;
        const APPLY_VIEWPORT: u32 = 2;
        const UNIT_F32: u32 = 0x3f80_0000;

        let key = (this.wrapping_add(8) as *const u32).read_unaligned();
        let camera = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            lf_checker_rt::relocated(MANAGER),
            key,
        );
        let receiver = camera.wrapping_add(0x10);
        let x0 = (this.wrapping_add(0x0c) as *const u32).read_unaligned();
        let y0 = (this.wrapping_add(0x10) as *const u32).read_unaligned();
        let x1 = (this.wrapping_add(0x14) as *const u32).read_unaligned();
        let y1 = (this.wrapping_add(0x18) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            APPLY_VIEWPORT,
            u32,
            receiver,
            x0,
            y0,
            x1,
            y1,
            0,
            UNIT_F32,
        )
    }
});
