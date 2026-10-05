// original: 0x00c03070 stream_init_67 (proposed)

/// Initialise a streaming record (tag `0x67`) and publish three arguments.
///
/// `this` points to the record. Writes the tag and the shared streaming
/// global, then calls the field callee with pointers to the first three
/// incoming arguments (the callee reads the pointed-to words; the rewrite
/// passes pointers to local copies holding the same values, and the
/// contract skips the addresses while snapshotting the contents), and
/// stores the float `f4` at `+0x14`. Returns the callee's answer (what the
/// original leaves in `eax`).
///
/// Original: 0x00c03070 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00c03070(this: u32, a1: u32, a2: u32, a3: u32, f4: u32) -> u32 {
    unsafe {
        const TAG: u8 = 0x67;
        const GLOBAL_STREAM: u32 = 0x011735a4;
        (this as *mut u8).write(TAG);
        let g = (lf_checker_rt::relocated(GLOBAL_STREAM) as *const u32).read_unaligned();
        (this.wrapping_add(4) as *mut u32).write_unaligned(g);
        let c1 = a1;
        let c2 = a2;
        let c3 = a3;
        let ans = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            this,
            &c1 as *const u32 as u32,
            &c2 as *const u32 as u32,
            &c3 as *const u32 as u32
        );
        (this.wrapping_add(0x14) as *mut u32).write_unaligned(f4);
        ans
    }
});
