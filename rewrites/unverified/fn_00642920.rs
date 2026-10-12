// original: 0x00642920 rage::ptxDomain::vf3

/// Copy the three consecutive origin components from offsets `0xe0`, `0xe4` and `0xe8` into the caller's three-word result buffer. Preserve the raw 32-bit values and return the output pointer.
lf_checker_rt::export!(thiscall, rw_00642920(this: *mut u8, out: *mut u32) -> *mut u32 {
    unsafe {
        const ORIGIN: usize = 0xE0;
        let source = this.add(ORIGIN) as *const u32;
        let x = source.read_unaligned();
        let y = source.add(1).read_unaligned();
        let z = source.add(2).read_unaligned();
        out.write_unaligned(x);
        out.add(1).write_unaligned(y);
        out.add(2).write_unaligned(z);
    }
    out
});
