// original: 0x00874c80 composite_node_teardown
/// Tears down a composite node: detaches both children, clears their slots,
/// runs the pair teardown twice, re-tags for the base class and runs the
/// base helper on the embedded sub-object when its gate word is set.
export!(thiscall, rw_00874c80(node: u32) -> () {
    unsafe {
        let base = node as *mut u32;
        base.write(relocated(0x00FE810C));
        let right = ((node + 0x198) as *const u32).read();
        if right != 0 {
            let slot = ((right as *const u32).read() + 8) as *const u32;
            let detach: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            detach(right);
        }
        ((node + 0x198) as *mut u32).write(0);
        let left = ((node + 0x194) as *const u32).read();
        if left != 0 {
            let slot = ((left as *const u32).read() + 8) as *const u32;
            let detach: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            detach(left);
        }
        ((node + 0x194) as *mut u32).write(0);
        callee_thiscall!(2, u32, node);
        callee_thiscall!(2, u32, node);
        base.write(relocated(0x00FE7FC8));
        let sub = node + 4;
        if ((node + 0xC) as *const u32).read() != 0 {
            callee_stdcall!(3, u32, sub);
        }
        (sub as *mut u32).write(relocated(0x00FE7FB4));
        if ((node + 0xC) as *const u32).read() != 0 {
            callee_stdcall!(3, u32, sub);
        }
        (sub as *mut u32).write(relocated(0x00E86AFC));
    }
});
