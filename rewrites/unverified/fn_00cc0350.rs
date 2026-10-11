// original: 0x00CC0350 is_route_control_code
/// Return one in AL when `value` is one of the four route-control codes
/// 0x35, 0x36, 0x37 or 0x39; otherwise return zero. The argument is cdecl.
lf_checker_rt::export!(cdecl, rw_00cc0350(value: u32) -> u32 {
    u32::from(matches!(value, 0x35 | 0x36 | 0x37 | 0x39))
});
