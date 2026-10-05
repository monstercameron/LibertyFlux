// original: 0x00c6d400 stream_system_init (proposed)

/// Initialise the streaming system: six subsystems, the dictionary,
/// the registry, then publish the root handle.
///
/// Six parameterless initialisers run in order, the animation
/// dictionary is created with (count 0x5DC, table 0xECBEA4), the
/// nineteen-word registry block (handler addresses and flags) is
/// installed, the root handle is derived from it and stored in its
/// global.
///
/// Original: cdecl with no arguments, nine call sites, writes one
/// global.
lf_checker_rt::export!(cdecl, rw_00c6d400() -> u32 {
    unsafe {
        const ROOT: u32 = 0x0104_96E8;
        const SYS_A: u32 = 1;
        const SYS_B: u32 = 2;
        const SYS_C: u32 = 3;
        const SYS_D: u32 = 4;
        const SYS_E: u32 = 5;
        const SYS_F: u32 = 6;
        const DICT_CREATE: u32 = 7;
        const INSTALL: u32 = 8;
        const DERIVE: u32 = 9;

        lf_checker_rt::callee_cdecl!(SYS_A, u32,);
        lf_checker_rt::callee_cdecl!(SYS_B, u32,);
        lf_checker_rt::callee_cdecl!(SYS_C, u32,);
        lf_checker_rt::callee_cdecl!(SYS_D, u32,);
        lf_checker_rt::callee_cdecl!(SYS_E, u32,);
        lf_checker_rt::callee_cdecl!(SYS_F, u32,);
        lf_checker_rt::callee_cdecl!(DICT_CREATE, u32, 0x5DC, lf_checker_rt::relocated(0x00ECBEA4));
        let h: u32 = lf_checker_rt::callee_stdcall!(
            INSTALL, u32, lf_checker_rt::relocated(0x00ECBEB8), lf_checker_rt::relocated(0x00ECBEB4), 0x5DC, lf_checker_rt::relocated(0x004BF5B0),
            0, lf_checker_rt::relocated(0x00C6D520), lf_checker_rt::relocated(0x00C6C760), lf_checker_rt::relocated(0x00C6D620),
            lf_checker_rt::relocated(0x00A7A530), lf_checker_rt::relocated(0x00C6D860), lf_checker_rt::relocated(0x00B54720),
            lf_checker_rt::relocated(0x00BE7A70), lf_checker_rt::relocated(0x00C6D3A0), lf_checker_rt::relocated(0x00401680),
            lf_checker_rt::relocated(0x00C6C7C0), lf_checker_rt::relocated(0x00C6C840), 0, 1, 1
        );
        let r: u32 = lf_checker_rt::callee_thiscall!(DERIVE, u32, h);
        unsafe { (lf_checker_rt::relocated(ROOT) as *mut u32).write_unaligned(r) };
        0
    }
});
