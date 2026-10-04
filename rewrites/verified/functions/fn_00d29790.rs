// original: 0x00d29790 targeting_attach (proposed)
/// Attach target `p`: store 10.0 at `+0x20`, `p` at `+0x14`, the global rate
/// at `+0x24`, then copy four words from `q+0x30` (when `[p+0x20]` is
/// non-null) or from `p+0x10` to `+0`. Finally refresh through the first
/// callee and release the slot through the second, returning its answer.
///
/// Thiscall, one stack word (pointer).
lf_checker_rt::export!(thiscall, rw_00d29790(this: u32, p: u32) -> u32 {
    unsafe {
        const TEN: u32 = 0x41200000;
        const RATE_GLOB: u32 = 0x01054634;
        const REFRESH: u32 = 1;
        const RELEASE: u32 = 2;
        ((this + 0x20) as *mut u32).write_unaligned(TEN);
        ((this + 0x14) as *mut u32).write_unaligned(p);
        let rate = lf_checker_rt::global::<u32>(RATE_GLOB).read_unaligned();
        ((this + 0x24) as *mut u32).write_unaligned(rate);
        let q = ((p + 0x20) as *const u32).read_unaligned();
        let src = if q != 0 { q.wrapping_add(0x30) } else { p.wrapping_add(0x10) };
        for i in 0..4u32 {
            let w = ((src + i * 4) as *const u32).read_unaligned();
            ((this + i * 4) as *mut u32).write_unaligned(w);
        }
        lf_checker_rt::callee_thiscall!(REFRESH, u32, this);
        lf_checker_rt::callee_stdcall!(RELEASE, u32, this + 0x14)
    }
});
