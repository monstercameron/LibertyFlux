// original: 0x005F0EA0 CEventHandlerNY::vf20

/// Forward to the shared handler when both child slots are set.
///
/// When either word at `first + 0x18` or `first + 0x1c` is null, returns
/// `first` unchanged. Otherwise tail-calls the shared handler with the three
/// stack arguments. The original takes three stack arguments and cleans them
/// up itself.
lf_checker_rt::export!(stdcall, rw_005f0ea0(first: u32, second: u32, third: u32) -> u32 {
    unsafe {
        const LEFT_OFFSET: u32 = 0x18;
        const RIGHT_OFFSET: u32 = 0x1c;
        const HANDLER_CALLEE: u32 = 1;
        let left =
            (first.wrapping_add(LEFT_OFFSET) as *const u32).read_unaligned();
        if left == 0 {
            return first;
        }
        let right =
            (first.wrapping_add(RIGHT_OFFSET) as *const u32).read_unaligned();
        if right == 0 {
            return first;
        }
        lf_checker_rt::callee_stdcall!(HANDLER_CALLEE, u32, first, second, third)
    }
});
