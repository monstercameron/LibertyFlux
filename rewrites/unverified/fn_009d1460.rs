// original: 0x009D1460 stream_ctx_teardown (proposed)
//
/// Tears down a streaming context's sub-objects in order.
///
/// Runs the child teardown (callee 1 on `this`, return ignored), then resets
/// four member vectors (callee 2 with `0` on `this + 0x58`, `+0x2c`, `+0x20`,
/// `+0x14` in that order). Returns nothing meaningful (`eax` is untouched
/// incoming-register passthrough). Thiscall, no arguments.
lf_checker_rt::export!(thiscall, rw_009D1460(this: u32) -> u32 {
    unsafe {
        const CHILD: u32 = 1;
        const RESET: u32 = 2;
        let _: u32 = lf_checker_rt::callee_thiscall!(CHILD, u32, this);
        for off in [0x58u32, 0x2c, 0x20, 0x14] {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                RESET, u32, this.wrapping_add(off), 0);
        }
        0
    }
});
