// original: 0x00d447c0 position_query_a (proposed)

/// Query three float slots from the position helper; return the first.
///
/// Reserves three scratch slots and calls the helper with the incoming `arg`
/// plus one pointer per slot (pushed slot by slot, so the helper's parameter
/// order is (arg, third, second, first)). Returns the first slot's float, bit
/// for bit, through the floating-point result channel. The helper is
/// cdecl/4 and is intercepted; its out-words are scripted by the checker.
///
/// Original: cdecl, one stack word, caller cleans up.
lf_checker_rt::export!(cdecl, rw_00d447c0(arg: u32) -> f32 {
    unsafe {
        const QUERY: u32 = 1;
        let mut first = 0u32;
        let mut second = 0u32;
        let mut third = 0u32;
        let p_first = core::ptr::addr_of_mut!(first) as u32;
        let p_second = core::ptr::addr_of_mut!(second) as u32;
        let p_third = core::ptr::addr_of_mut!(third) as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(QUERY, u32, arg, p_third, p_second, p_first);
        f32::from_bits(first)
    }
});
