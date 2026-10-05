// original: 0x00a07ac0 object_probe_publish (proposed)
/// Publish a probe vector to the shared slots and run the probe sweep.
///
/// Copies the four words at `vec` into the shared probe globals, then calls
/// the sweep with the 5-word header [x, y, 0, 0, 0.1], the relocated probe
/// kind, a 4-word window aliasing the header at word 2, and the (0x100, 5)
/// sweep parameters. The two zero header words are stack garbage in the
/// original; the checker pins them to zero and so does this rewrite.
/// Always answers 0. Cdecl, one word.
lf_checker_rt::export!(cdecl, rw_00a07ac0(vec: u32) -> u32 {
    unsafe {
        const SWEEP: u32 = 0;
        const KIND: u32 = 0x00a07d40;
        const SHARED: u32 = 0x012bd0a0;
        const TENTH: u32 = 0x3dcc_cccd;
        let x = (vec as *const u32).read_unaligned();
        let y = ((vec + 4) as *const u32).read_unaligned();
        let z = ((vec + 8) as *const u32).read_unaligned();
        let w = ((vec + 12) as *const u32).read_unaligned();
        let g = lf_checker_rt::global::<u32>(SHARED);
        (g.add(0) as *mut u32).write_unaligned(x);
        (g.add(1) as *mut u32).write_unaligned(y);
        (g.add(2) as *mut u32).write_unaligned(z);
        (g.add(3) as *mut u32).write_unaligned(w);
        let frame = [x, y, 0u32, 0u32, TENTH, z];
        lf_checker_rt::callee_cdecl!(SWEEP, u32, frame.as_ptr() as u32,
                                     lf_checker_rt::relocated(KIND),
                                     frame.as_ptr().add(2) as u32, 0x100u32, 5u32);
        0
    }
});
