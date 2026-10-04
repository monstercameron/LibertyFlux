// original: 0x00a71cf0 kind_is_4_5_6 (proposed)
/// True (1) when the task-kind code is 4, 5 or 6, else 0.
///
/// Takes one stack word (`cdecl`, plain `ret`). Pure integer compare,
/// no memory touched. Edge cases: 3 and 7 return 0, as do 0, -1 and large
/// values; only exactly 4/5/6 return 1.
lf_checker_rt::export!(cdecl, rw_00a71cf0(kind: u32) -> u8 {
    (kind == 4 || kind == 5 || kind == 6) as u8
});
