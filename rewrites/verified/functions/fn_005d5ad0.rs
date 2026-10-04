// original: 0x005d5ad0 CHtmlTableElementNode::CHtmlTableElementNode
/// Dispatch on the node kind at `+0x4` (jump table over 0-3, anything else
/// returns the kind unchanged): kinds 0-2 delegate to their own builders
/// with the extra argument, kind 3 delegates and then stamps this node's
/// vtable. Returns the builder answer on kinds 0-3.
export!(fastcall, rw_005d5ad0(this_ptr: u32, extra: u32) -> u32 {
    let kind = unsafe { ((this_ptr + 4) as *const u32).read() };
    match kind {
        0 => callee_thiscall!(0, u32, this_ptr, extra),
        1 => callee_thiscall!(1, u32, this_ptr, extra),
        2 => callee_thiscall!(2, u32, this_ptr, extra),
        3 => {
            let r: u32 = callee_thiscall!(0, u32, this_ptr, extra);
            unsafe { (this_ptr as *mut u32).write(relocated(0x00FE0B58)) };
            r
        }
        _ => kind,
    }
});
