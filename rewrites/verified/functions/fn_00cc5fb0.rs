// original: 0x00cc5fb0 notify_forward_b
/// Gate on the enable flag and three readiness probes; map the id through the
/// id table and tail-forward it to handler B. No meaningful return value.
export!(cdecl, rw_00cc5fb0(id: u32) -> u32 {
    unsafe {
        if *global::<u8>(0x12B41C5) == 0 {
            return 0;
        }
        let r1: u32 = callee_cdecl!(1, u32,);
        if r1 & 0xFF != 0 {
            return 0;
        }
        let r2: u32 = callee_cdecl!(2, u32,);
        if r2 & 0xFF != 0 {
            return 0;
        }
        let r3: u32 = callee_cdecl!(3, u32,);
        if r3 & 0xFF != 0 {
            return 0;
        }
        if id == 0xFFFF_FFFF {
            return 0;
        }
        let table = *global::<u32>(0x16DD63C);
        let mapped: u32 = callee_thiscall!(4, u32, table, id);
        callee_cdecl!(5, u32, mapped)
    }
});
