// original: 0x00b68020 CHeli::vf103
/// Refresh the four words at +0x1EE0 from the worker's answer struct.
///
/// Notifies the first callee (thiscall/1), returns early for a null
/// argument or a -1 sentinel at `tab2[0x16C]`, runs the sizing callee
/// (thiscall/2 on `(this, 0x5B, scratch)`) and the worker (thiscall/4 on
/// `(this+0x10E0, scratch, arg, scratch, 1)`), then copies the answer's
/// dword, two floats and trailing dword to +0x1EE0 (thiscall, one stack
/// argument; no meaningful return value). Scratch-stack arguments are
/// skipped in the call comparison.
export!(thiscall, rw_00b68020(this: u32, arg: u32) -> u32 {
    unsafe {
        const MODEL_INDEX_OFF: u32 = 0x2e;
        const MODEL_TABLE: u32 = 0x01295cd8;
        let _: u32 = callee_thiscall!(1, u32, this, arg);
        if arg == 0 {
            return 0;
        }
        let model =
            ((this.wrapping_add(MODEL_INDEX_OFF)) as *const i16).read_unaligned() as i32 as u32;
        let row = (relocated(MODEL_TABLE).wrapping_add(model.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let tab2 = (row.wrapping_add(0xcc) as *const u32).read_unaligned();
        if (tab2.wrapping_add(0x16c) as *const u32).read_unaligned() == 0xffffffff {
            return 0;
        }
        let mut s1 = [0u32; 4];
        let _: u32 = callee_thiscall!(2, u32, this, 0x5b, (&mut s1 as *mut u32) as u32);
        let mut s2 = [0u32; 4];
        let mut s3 = [0u32; 4];
        let ans: u32 = callee_thiscall!(
            3,
            u32,
            this.wrapping_add(0x10e0),
            (&mut s2 as *mut u32) as u32,
            arg,
            (&mut s3 as *mut u32) as u32,
            1
        );
        let x = f32::from_bits((ans.wrapping_add(4) as *const u32).read_unaligned());
        let y = f32::from_bits((ans.wrapping_add(8) as *const u32).read_unaligned());
        let z = (ans as *const u32).read_unaligned();
        (this.wrapping_add(0x1ee0) as *mut u32).write_unaligned(z);
        (this.wrapping_add(0x1ee4) as *mut u32).write_unaligned(x.to_bits());
        (this.wrapping_add(0x1ee8) as *mut u32).write_unaligned(y.to_bits());
        let w = (ans.wrapping_add(0xc) as *const u32).read_unaligned();
        (this.wrapping_add(0x1eec) as *mut u32).write_unaligned(w);
        0
    }
});
