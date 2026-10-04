// original: 0x00A99DB0 volume_update_pass (stage 1: entry through the all-zeros early exit)
/// Stage-1 rewrite of 0x00A99DB0 (original: 0x00A99DB0 volume_update_pass):
/// entry through the all-zeros early exit only. Later stages extend the
/// corpus past this gate and replace the unreachable below.
export!(thiscall, rw_00a99db0_stage1(this: u32, arg0: u32, _arg1: u32) -> u32 {
    unsafe {
        // Callee 1 takes (out, [arg0+0x68], [this+0x20], arg0): the pushes
        // run first-to-last, so the last push (the frame pointer) is the
        // callee's first parameter. It fills the output vector at out+0/4/8.
        // The buffer is deliberately uninitialized so a stub that failed to
        // write would surface as the checker's nonzero stack fill.
        let mut out = core::mem::MaybeUninit::<[u32; 8]>::uninit();
        let outp = out.as_mut_ptr() as u32;
        let ans: u32 = callee_cdecl!(
            1,
            u32,
            outp,
            *(arg0.wrapping_add(0x68) as *const u32),
            *(this.wrapping_add(0x20) as *const u32),
            arg0
        );
        let words = outp as *const u32;
        let x = f32::from_bits(core::ptr::read(words.add(0)));
        let y = f32::from_bits(core::ptr::read(words.add(1)));
        let z = f32::from_bits(core::ptr::read(words.add(2)));
        // ucomiss+lahf "all equal to +0.0" gate (-0.0 counts as zero).
        if x == 0.0 && y == 0.0 && z == 0.0 {
            // Each gate compare runs lahf, so the exit eax is the callee
            // answer with AH replaced by the last compare's flags. On this
            // path every compare is equal, hence AH is always 0x42
            // (ZF=1; PF=CF=0; SF=OF=AF=0 as ucomiss leaves them; bit 1 set).
            (ans & 0xFFFF00FF) | 0x4200
        } else {
            unreachable!("stage 2: nonzero entry vector continues into the main body")
        }
    }
});
