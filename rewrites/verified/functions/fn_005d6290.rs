// original: 0x005d6290 CHtmlTableNode::CHtmlTableNode
/// Construct a table node over the base built by the delegated
/// constructor, then relocate the five offset fields (in the original's
/// order `e8/ec/f0/f8/f4`) by adding the translator answer for each
/// non-zero one. Returns the object.
export!(thiscall, rw_005d6290(this_ptr: u32, ctx: u32) -> u32 {
    let _: u32 = callee_thiscall!(0, u32, this_ptr, ctx);
    unsafe { (this_ptr as *mut u32).write(relocated(0x00FE0B50)) };
    for off in [0xE8u32, 0xEC, 0xF0, 0xF8, 0xF4] {
        let v = unsafe { ((this_ptr + off) as *const u32).read() };
        if v != 0 {
            let d: u32 = callee_thiscall!(1, u32, ctx, v);
            unsafe { ((this_ptr + off) as *mut u32).write(v.wrapping_add(d)) };
        }
    }
    this_ptr
});
