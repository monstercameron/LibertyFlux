// original: 0x0089cf80 audio_rebase_keys (proposed)

/// Rebases the keys of grouped record arrays in place, returning 0.
///
/// Runs the setup helper (callee 1, cdecl/3, twice, answers ignored) with
/// (`a4`, `LO_GLOB`, `SZ_GLOB`) and (`SZ+a4`, `a2`, `a3`). Then, unless `a1`
/// is 0 (in which case the second setup answer is returned), walks `a1`
/// 10-byte headers at `a0` (record base at `+0`, 16-bit count at `+8`):
/// for each of the count 14-byte records, a key `v` at the record start is
/// rebased UNSIGNED: inside [`LO`, `LO`+`SZ`) it becomes `v-LO`, otherwise
/// `v-a2+SZ`; then `a4` is added and the key stored back. Counts and the
/// range test are UNSIGNED (`jbe`/`jb`/`jae`). The outer counter reuses
/// incoming `a1`'s stack slot (ending at 0), so the stack check is off.
///
/// Original: 0x0089cf80 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_0089cf80(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const LO_GLOB: u32 = 0x0115f834;
        const SZ_GLOB: u32 = 0x0115f838;
        const HEAD_STRIDE: u32 = 10;
        const BASE_OFF: u32 = 0;
        const COUNT_OFF: u32 = 8;
        const REC_STRIDE: u32 = 14;
        const KEY_OFF: u32 = 0;
        const SETUP: u32 = 1;
        let lo = lf_checker_rt::global::<u32>(LO_GLOB).read_unaligned();
        let sz = lf_checker_rt::global::<u32>(SZ_GLOB).read_unaligned();
        lf_checker_rt::callee_cdecl!(SETUP, u32, a4, lo, sz);
        let r2: u32 = lf_checker_rt::callee_cdecl!(SETUP, u32, sz.wrapping_add(a4), a2, a3);
        if a1 == 0 {
            return r2;
        }
        for i in 0..a1 {
            let head = a0.wrapping_add(i.wrapping_mul(HEAD_STRIDE));
            let base = (head + BASE_OFF) as *const u32;
            let base = base.read_unaligned();
            let count = ((head + COUNT_OFF) as *const u16).read_unaligned() as u32;
            for j in 0..count {
                let kp = base.wrapping_add(j.wrapping_mul(REC_STRIDE)) as *mut u32;
                let v = kp.read_unaligned();
                let mut v = if v.wrapping_sub(lo) < sz { v.wrapping_sub(lo) } else { v.wrapping_sub(a2).wrapping_add(sz) };
                v = v.wrapping_add(a4);
                kp.write_unaligned(v);
            }
        }
        0
    }
});
