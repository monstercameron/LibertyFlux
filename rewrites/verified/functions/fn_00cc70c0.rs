// original: 0x00cc70c0 id_register_check
/// Register-or-check an id: map it through the id table, probe whether it is
/// already registered, and record it when new. Returns 1 when the id was
/// already known (or unmappable) and 0 when freshly recorded (low byte only).
export!(cdecl, rw_00cc70c0(id: u32) -> u32 {
    unsafe {
        let table = *global::<u32>(0x16DD63C);
        let idx: u32 = callee_thiscall!(1, u32, table, id);
        if idx == 0xFFFF_FFFF {
            return 1;
        }
        let set = *global::<u32>(0x10496E8);
        let known: u32 = callee_cdecl!(2, u32, idx, set);
        if known & 0xFF != 0 {
            return 1;
        }
        let _: u32 = callee_cdecl!(3, u32, idx, set, 8);
        0
    }
});
