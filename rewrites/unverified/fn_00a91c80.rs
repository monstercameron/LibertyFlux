// original: 0x00a91c80 stream_open_entry_guarded (proposed)

/// Open an entry unless it is already open or fails its guard.
///
/// A nonzero head word at the entry (second argument) returns at once.
/// Else the entry is probed (callee 1, thiscall/2 on `entry+0x30` with the
/// first argument and the global threshold float); a zero low byte in the
/// answer, or a nonzero flag byte
/// at `entry+0x54`, returns that answer. Otherwise the entry is converted
/// (callee 2, cdecl/2 with the entry and 0x18), finalised (callee 3,
/// stdcall/1 with the converted value; ecx is stub garbage there, so it is
/// deliberately not compared) and committed (callee 4, cdecl/1 with 1).
///
/// Returns 0 for an already-open entry (entry eax is pinned to 0 by the
/// contract), the probe answer on the guard paths, else the commit answer.
/// Cdecl, two arguments.
lf_checker_rt::export!(cdecl, rw_00a91c80(a0: u32, entry: u32) -> u32 {
    unsafe {
        const CONVERT_ARG: u32 = 0x18;
        const COMMIT_ARG: u32 = 1;
        const PROBE_OFF: u32 = 0x30;
        const FLAG_OFF: u32 = 0x54;
        const FLOAT_GLOBAL: u32 = 0x012fb264;
        const SET_GLOBAL: u32 = 0x012fb258;
        if ((entry) as *const u32).read_unaligned() != 0 {
            return 0;
        }
        let threshold = lf_checker_rt::global::<u32>(FLOAT_GLOBAL).read_unaligned();
        let probe =
            lf_checker_rt::callee_thiscall!(1, u32, entry + PROBE_OFF, a0, threshold);
        if (probe & 0xff) == 0 {
            return probe;
        }
        if (((entry + FLAG_OFF) as *const u8).read()) != 0 {
            return probe;
        }
        let set = lf_checker_rt::global::<u32>(SET_GLOBAL).read_unaligned();
        let _ = set;
        let conv = lf_checker_rt::callee_cdecl!(2, u32, entry, CONVERT_ARG);
        lf_checker_rt::callee_stdcall!(3, u32, conv);
        lf_checker_rt::callee_cdecl!(4, u32, COMMIT_ARG)
    }
});
