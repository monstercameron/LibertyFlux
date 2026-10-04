// original: 0x00a7eac0 CComplexJumpTaskInfo::vf6
/// Serialize the three `CComplexJumpTaskInfo` flag bytes.
///
/// Runs the base serializer, writes the bytes at `+0x18..+0x1a`, and
/// returns the last helper answer ORed with every earlier changed flag.
lf_checker_rt::export!(thiscall, rw_00a7eac0(this: u32, stream: u32) -> u32 {
    unsafe {
        let base = this as *const u8;
        let mut acc = lf_checker_rt::callee_thiscall!(1, u32, this, stream) as u8;
        let mut last = 0u32;
        for (i, off) in [0x18usize, 0x19, 0x1a].iter().enumerate() {
            let b = base.add(*off).read();
            let r = lf_checker_rt::callee_thiscall!(2, u32, stream, b as u32, 0);
            if i < 2 {
                acc |= r as u8;
            } else {
                last = r;
            }
        }
        last | acc as u32
    }
});
