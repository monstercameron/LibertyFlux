// original: 0x005d5be0 CHtmlElementNode::CHtmlElementNode
/// Construct an element node: stamp the base vtable, clear the header,
/// build the member at `+0x14`, then stamp this vtable, clear the string
/// slot and store the argument at `+0xdc`. Returns the object.
export!(thiscall, rw_005d5be0(this_ptr: u32, arg: u32) -> u32 {
    unsafe {
        (this_ptr as *mut u32).write(relocated(0x00FE0B94));
        ((this_ptr + 8) as *mut u32).write(0);
        ((this_ptr + 0x0C) as *mut u32).write(0);
        ((this_ptr + 0x10) as *mut u32).write(0);
    }
    let _: u32 = callee_thiscall!(0, u32, this_ptr.wrapping_add(0x14));
    unsafe {
        (this_ptr as *mut u32).write(relocated(0x00FE0B9C));
        ((this_ptr + 0xE4) as *mut u32).write(0);
        ((this_ptr + 0xE0) as *mut u32).write(0);
        ((this_ptr + 4) as *mut u32).write(0);
        ((this_ptr + 0xDC) as *mut u32).write(arg);
    }
    this_ptr
});
