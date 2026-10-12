// original: 0x006428F0 rage::ptxDomain::vf2

/// Copy the three consecutive origin components from offsets `0xf0`, `0xf4` and `0xf8` into the caller's three-word result buffer. Preserve the raw 32-bit values and return the output pointer.
lf_checker_rt::export!(thiscall, rw_006428f0(this: *mut u8, out: *mut u32) -> *mut u32 {
    unsafe {
        const DEFAULT_ORIGIN: usize = 0xF0;
        let source = this.add(DEFAULT_ORIGIN) as *const u32;
        let x = source.read_unaligned();
        let y = source.add(1).read_unaligned();
        let z = source.add(2).read_unaligned();
        out.write_unaligned(x);
        out.add(1).write_unaligned(y);
        out.add(2).write_unaligned(z);
    }
    out
});
