// original: 0x00a12240 driver_presence_publish (proposed)
/// Publish driver-presence flags derived from globals and object state.
///
/// Fetches the status word, reads the mode global and the driver object,
/// and decides presence: false when the mode is -1 or 0, or when there is
/// no driver. A status bit chooses whether the strobe flag follows presence
/// or stays cleared; two presence flags are then cleared and, when present,
/// set from two probe calls on the driver. Returns presence as 0/1, or the
/// second probe's answer when the probes ran. Cdecl, no arguments.
export!(cdecl, rw_00a12240() -> u32 {
    unsafe {
        const FETCH: u32 = 1;
        const PROBE_A: u32 = 2;
        const PROBE_B: u32 = 3;
        const STATUS_OFF: u32 = 0x24;
        const STROBE_BIT: u32 = 27;
        const MODE: u32 = 0x0103b110;
        const DRIVER: u32 = 0x012b41a4;
        const FLAG_BASE: u32 = 0x012bd0f2;
        let st = callee_cdecl!(FETCH, u32, 0);
        let mode = *global::<u32>(MODE);
        let drv = *global::<u32>(DRIVER);
        let strobe = (((st + STATUS_OFF) as *const u32).read_unaligned() >> STROBE_BIT) & 1;
        let mut present: u32 = 1;
        if mode == 0xffff_ffff || mode == 0 {
            present = 0;
        }
        if drv == 0 {
            present = 0;
        }
        if strobe != 0 {
            *global::<u8>(FLAG_BASE + 11) = 1;
            if present == 0 {
                *global::<u8>(FLAG_BASE + 11) = 0;
            }
        } else {
            *global::<u8>(FLAG_BASE + 11) = 0;
        }
        *global::<u8>(FLAG_BASE) = 0;
        *global::<u8>(FLAG_BASE + 1) = 0;
        if present != 0 {
            let a = callee_thiscall!(PROBE_A, u32, drv);
            *global::<u8>(FLAG_BASE + 1) = (a & 0xff) as u8;
            let b = callee_thiscall!(PROBE_B, u32, drv);
            *global::<u8>(FLAG_BASE) = (b & 0xff) as u8;
            return b;
        }
        present
    }
});
