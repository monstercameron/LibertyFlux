// original: 0x009e95c0 ped_kind_is_simple
/// True when the kind argument is 1 or `0x14`. (cdecl; low byte.)
lf_checker_rt::export!(cdecl, rw_009e95c0(kind: u32) -> u32 {
    unsafe {
        if kind == 1 || kind == 0x14 { 1 } else { 0 }
    }
});
