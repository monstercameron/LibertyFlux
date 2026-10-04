// original: 0x00a7eb90 CComplexNewUseCoverInfo::vf6
/// Serialize `CComplexNewUseCoverInfo`: one 7-bit field, six bytes.
///
/// Runs the base serializer, writes the `+0x18` field with 7 bits and
/// the bytes at `+0x1c..+0x21`, and returns the last helper answer
/// ORed with every earlier changed flag.
lf_checker_rt::export!(thiscall, rw_00a7eb90(this: u32, stream: u32) -> u32 {
    unsafe {
        let base = this as *const u8;
        let mut acc = lf_checker_rt::callee_thiscall!(1, u32, this, stream) as u8;
        let f0 = (base.add(0x18) as *const u32).read_unaligned();
        acc |= lf_checker_rt::callee_thiscall!(2, u32, stream, f0, 7, 0) as u8;
        let mut last = 0u32;
        for (i, off) in [0x1cusize, 0x1d, 0x1e, 0x1f, 0x20, 0x21].iter().enumerate() {
            let b = base.add(*off).read();
            let r = lf_checker_rt::callee_thiscall!(3, u32, stream, b as u32, 0);
            if i < 5 {
                acc |= r as u8;
            } else {
                last = r;
            }
        }
        last | acc as u32
    }
});
