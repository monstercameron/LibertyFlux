// original: 0x00bebe30 gather_buffers_call_kernel
/// Fill two stack buffers through helpers, then call the kernel on them.
///
/// Calls helper one (thiscall on `this`, intercepted) to fill three words
/// at a stack buffer and helper two to fill four words at a second buffer,
/// then calls the kernel (thiscall, intercepted) with ECX = `arg + 0x10e0`
/// and the two buffer base addresses (second buffer first). The addresses
/// are the function's own frame, so the contract skips them and compares
/// the stub-written words through the kernel call's snapshots instead.
/// Returns the kernel's answer. Thiscall, one stack argument.
export!(thiscall, rw_00bebe30(this: u32, arg: u32) -> u32 {
    unsafe {
        const HELPER_ONE: u32 = 1;
        const HELPER_TWO: u32 = 2;
        const KERNEL: u32 = 3;
        const KERNEL_OBJ_OFF: u32 = 0x10e0;
        let mut b1 = [0u32; 3];
        let mut b2 = [0u32; 4];
        let _: u32 = callee_thiscall!(HELPER_ONE, u32, this, b1.as_mut_ptr() as u32);
        let _: u32 = callee_thiscall!(HELPER_TWO, u32, this, b2.as_mut_ptr() as u32);
        let r: u32 = callee_thiscall!(KERNEL, u32, arg.wrapping_add(KERNEL_OBJ_OFF),
            b2.as_mut_ptr() as u32,
            b1.as_mut_ptr() as u32);
        r
    }
});
