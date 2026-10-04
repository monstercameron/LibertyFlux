// original: 0x00e5d1f0 clear_flag_and_register
/// Clear the low bit of the status flag, then register a fixed code pointer.
///
/// ANDs the flag byte at `0x018E0104` with `0xFE`, then passes `0x00E6E800`
/// to the registrar (`cdecl/1`, stubbed). Returns the registrar's answer.
export!(cdecl, rw_00e5d1f0() -> u32 {
    unsafe {
        *global::<u8>(0x18E0104) &= 0xFE;
        callee_cdecl!(2, u32, relocated(0xE6E800))
    }
});
