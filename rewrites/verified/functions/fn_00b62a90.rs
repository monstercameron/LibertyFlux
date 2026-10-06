// original: 0x00B62A90 veh_convert_or_copy3
/// Copy 3 dwords, or convert through a scratch buffer when `a2 != -1`.
///
/// Copies `[a1+0x30..0x38]` to `[a0..a0+8]`. Returns `a0` at once when
/// `a2 == -1`. Otherwise calls the converter (stubbed, thiscall/4) with
/// `(a1, frame_scratch, a2, 0)`; the stub fills 4 words at scratch+0x30 (as the
/// real callee fills its out-block). When the answer's low byte is non-zero,
/// copies those 4 words over `[a0..a0+12]`. Returns `a0`. The original forwards
/// its ignored entry ECX to the converter; the rewrite passes 0 (unobserved).
/// Thiscall-shaped stdcall: entry ECX is forwarded to the converter (the
/// nested converter reads the words at +0x18 and +0x24 through it), so the
/// rewrite binds ECX explicitly; the stack shape is unchanged.
/// The scratch-pointer argument is skipped; ECX words +0x18/+0x24 are snapshotted.
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot};
export!(thiscall, rw_00b62a90(ecx: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        for (s, d) in [(0x30u32, 0u32), (0x34, 4), (0x38, 8)] {
            ((a0 + d) as *mut u32).write_unaligned(((a1 + s) as *const u32).read_unaligned());
        }
        if (a2 as i32) == -1 {
            return a0;
        }
        let mut scratch = [0u32; 16];
        let r: u32 = callee_thiscall!(1, u32, ecx, a1, scratch.as_mut_ptr() as u32, a2, 0);
        if (r & 0xFF) == 0 {
            return a0;
        }
        for i in 0..4usize {
            ((a0 + (i as u32) * 4) as *mut u32).write_unaligned(scratch[12 + i]);
        }
        a0
    }
});
