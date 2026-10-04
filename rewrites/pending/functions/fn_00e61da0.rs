// original: 0x00e61da0 timer_table_init_64
// Table initialiser: runs the shared setter reached through the data-table
// slot once per entry over the 64-entry record table, clearing each entry's
// low flag bits and resetting its two link words, then hands its descriptor
// to the shared registrar and returns the registrar's answer.
lf_checker_rt::export!(cdecl, rw_00e61da0() -> u32 {
    const SETTER_SLOT: u32 = 0x00E731C4;
    const RECORD0: u32 = 0x01B48508;
    const COUNT: usize = 64;
    const STRIDE: u32 = 0x2C;
    const DESC: u32 = 0x00E70790;
    let setter: extern "stdcall" fn(u32) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::global::<u32>(SETTER_SLOT).read() as usize)
    };
    let mut rec = lf_checker_rt::relocated(RECORD0);
    for _ in 0..COUNT {
        let _ = setter(rec.wrapping_sub(0x28));
        unsafe {
            let flag = rec as *mut u8;
            flag.write(flag.read() & 0xFC);
            (rec.wrapping_sub(8) as *mut u32).write(0xFFFF_FFFF);
            (rec.wrapping_sub(4) as *mut u32).write(0);
        }
        rec = rec.wrapping_add(STRIDE);
    }
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(DESC))
});
