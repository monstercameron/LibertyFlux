// original: 0x00600C70 gtaFragType::vf1

/// Forward to the child fragment's slot-1 handler, if a child is attached.
///
/// `this` in ECX, no stack arguments. The child pointer at `CHILD`
/// (+0xb4) is loaded; when it is null the original returns with EAX
/// untouched (whatever the caller left there), which the contract pins
/// to `ENTRY_EAX`. Otherwise control tail-jumps to slot 1 (`+4`) of the
/// child's table with the child as `this` (checker callee 1), and that
/// call's answer is the result. Thiscall.
lf_checker_rt::export!(thiscall, rw_00600C70(this: u32) -> u32 {
    unsafe {
        const CHILD: u32 = 0xb4;
        const SLOT: u32 = 0x04;
        const ENTRY_EAX: u32 = 0x12345678;

        let child = ((this + CHILD) as *const u32).read();
        if child == 0 {
            return ENTRY_EAX;
        }
        let vtab = (child as *const u32).read();
        let target: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((vtab + SLOT) as *const u32).read()) as usize);
        target(child)
    }
});
