// original: 0x00874170 oneshot_init_guard
/// One-shot initialisation guard: calls the subsystem init routine the first
/// time it runs and records that it has run, so later calls do nothing. The
/// original leaves an unspecified value in eax; this rewrite returns nothing.
export!(cdecl, rw_00874170() -> () {
    unsafe {
        let flag = global::<u8>(0x01BB68D5);
        if flag.read() == 0 {
            callee_cdecl!(1, u32,);
            flag.write(1);
        }
    }
});
