// original: 0x00967140 affinity_covers_low_cpus
/// Report whether the process affinity mask covers CPUs 0-3.
///
/// Calls `GetCurrentProcess` (callee 1, IAT) for the pseudo-handle, then
/// `GetProcessAffinityMask` (callee 2, IAT) with two out-pointers to its
/// own scratch, and returns 1 in `al` when the process mask is at least
/// 0xF, else 0. Upper `eax` keeps the second call's `BOOL`, hence the
/// `al` return channel. The out-pointer arguments address stack scratch
/// whose layout differs per side, so the contract skips them; the mask
/// itself is observed through the return value.
///
/// Original: 0x00967140 (cdecl, no stack words; both callees stdcall).

export!(cdecl, rw_00967140() -> u32 {
    unsafe {
        const FULL_LOW_NIBBLE: u32 = 0xF;
        let proc = callee_stdcall!(1, u32,);
        let mut mask: u32 = 0;
        let mut sys: u32 = 0;
        callee_stdcall!(
            2,
            u32,
            proc,
            &mut mask as *mut u32 as u32,
            &mut sys as *mut u32 as u32
        );
        u32::from(mask >= FULL_LOW_NIBBLE)
    }
});
