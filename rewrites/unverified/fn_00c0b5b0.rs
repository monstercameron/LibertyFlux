// original: 0x00C0B5B0 wanted_level_spawn_factor (proposed)

/// Spawn factor selected by wanted level and a streaming flag.
///
/// Takes no arguments and returns a float on the x87 stack. Calls the
/// streaming selector (intercepted callee 1, one stack word set to 0);
/// when its result is non-zero the first factor table applies, else the
/// second. Then reads the current handle (callee 2, no arguments) and
/// the wanted level for it (callee 3, handle in ECX). The level minus
/// two indexes the table as an UNSIGNED comparison against 4: levels 2
/// to 6 pick the table's five factor constants from the image, any
/// other level returns 1.0. The first table's slot 0 also yields 1.0,
/// and the second table's slot 0 reuses the first table's 0.9 factor;
/// both quirks are the original's jump tables, reproduced exactly.
///
/// Original: 0x00C0B5B0 (cdecl, no arguments; selector at 0x0093F1C0,
/// handle query at 0x0093F1E0, level query at 0x00A1DF20, tables in
/// code, factors in read-only data).
lf_checker_rt::export!(cdecl, rw_00c0b5b0() -> f32 {
    unsafe {
        /// Recorder id of the streaming selector call.
        const SELECT: u32 = 1;
        /// Recorder id of the current-handle query call.
        const CURRENT: u32 = 2;
        /// Recorder id of the wanted-level query call.
        const LEVEL: u32 = 3;
        /// Factor constants in read-only data, first table slots 1-4.
        const F09: u32 = 0x00FE_88BC;
        /// Second factor constant of the first table.
        const F08: u32 = 0x00FE_8898;
        /// Third factor constant of the first table.
        const F065: u32 = 0x00FE_8864;
        /// Fourth factor constant of the first table.
        const F05: u32 = 0x00FE_8830;
        /// Factor constants of the second table, slots 1-4.
        const G06: u32 = 0x00FE_8858;
        /// Second factor constant of the second table.
        const G04: u32 = 0x00FE_881C;
        /// Third factor constant of the second table.
        const G03: u32 = 0x00FE_87E8;
        /// Fourth factor constant of the second table.
        const G02: u32 = 0x00FE_87D0;

        #[inline(always)]
        unsafe fn factor(addr: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::relocated(addr) as *const u32).read()) }
        }

        let flag: u32 = lf_checker_rt::callee_cdecl!(SELECT, u32, 0);
        let handle: u32 = lf_checker_rt::callee_cdecl!(CURRENT, u32);
        let level: u32 = lf_checker_rt::callee_thiscall!(LEVEL, u32, handle);
        let slot = level.wrapping_sub(2);
        if flag != 0 {
            match slot {
                1 => factor(F09),
                2 => factor(F08),
                3 => factor(F065),
                4 => factor(F05),
                _ => 1.0,
            }
        } else {
            match slot {
                0 => factor(F09),
                1 => factor(G06),
                2 => factor(G04),
                3 => factor(G03),
                4 => factor(G02),
                _ => 1.0,
            }
        }
    }
});
