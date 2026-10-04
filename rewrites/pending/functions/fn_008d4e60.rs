// original: 0x008D4E60 BuildTrigTriple
/// Builds a float triplet from the two helpers: the masked product of the
/// first two answers in slot 0, the second product in slot 1, and the plain
/// helper answer in slot 2. Returns the output pointer.
export!(cdecl, rw_008D4E60(out: u32, a: f32, b: f32) -> u32 {
    unsafe {
        let f1 = f32::from_bits(callee_cdecl!(0, u32, a.to_bits()));
        let f2 = f32::from_bits(callee_cdecl!(1, u32, b.to_bits()));
        let mask = *global::<u32>(0xFE8FA0);
        ((out) as *mut u32).write_unaligned((f2 * f1).to_bits() ^ mask);
        let f3 = f32::from_bits(callee_cdecl!(2, u32, b.to_bits()));
        ((out + 4) as *mut f32).write_unaligned(f3 * f1);
        let f4 = f32::from_bits(callee_cdecl!(3, u32, a.to_bits()));
        ((out + 8) as *mut f32).write_unaligned(f4);
        out
    }
});
