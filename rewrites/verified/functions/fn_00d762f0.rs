// original: 0x00D762F0 gate_chain_status_check (proposed)

/// Return 1 when a three-gate status chain passes, else 0 in `al`.
///
/// Queries the global registry, requiring a non-null answer whose word at
/// `+0x38c` equals 9, then requires a non-null handle at `this + 0x44` and
/// a non-zero low byte from the per-handle check. Only `al` is set on
/// return: the upper 24 bits of `eax` are the last callee answer's upper
/// bytes (zero when the first query answered null). Thiscall: object in
/// `ecx`, no stack words.
use lf_checker_rt::{callee_thiscall, export, relocated};

const REGISTRY_QUERY: u32 = 1;
const HANDLE_CHECK: u32 = 2;

export!(thiscall, rw_00d762f0(this: u32) -> u32 {
    unsafe {
        const REGISTRY: u32 = 0x0103e498;
        const KIND_OFF: u32 = 0x38c;
        const KIND_WANT: u32 = 9;
        const HANDLE_OFF: u32 = 0x44;
        let reg = callee_thiscall!(REGISTRY_QUERY, u32, relocated(REGISTRY));
        if reg == 0 {
            return 0;
        }
        if ((reg + KIND_OFF) as *const u32).read_unaligned() != KIND_WANT {
            return reg & 0xffff_ff00;
        }
        let h = ((this + HANDLE_OFF) as *const u32).read_unaligned();
        if h == 0 {
            return reg & 0xffff_ff00;
        }
        let ans = callee_thiscall!(HANDLE_CHECK, u32, h);
        (ans & 0xffff_ff00) | u32::from(ans & 0xff != 0)
    }
});
