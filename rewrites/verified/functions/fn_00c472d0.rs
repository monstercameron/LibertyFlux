// original: 0x00c472d0 CCamScripted::vf0 (symbols)
/// Deleting destructor: destroy, then free through the game allocator.
///
/// Runs the destructor (callee 1); when the low bit of the disposition
/// argument is set, frees `this` via the allocator object held in the
/// global (callee 2). Returns `this`.
///
/// Original: thiscall, one stack word, callee cleanup (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c472d0(this: u32, disp: u32) -> u32 {
    const DESTRUCTOR: u32 = 1;
    const OPERATOR_DELETE: u32 = 2;
    const ALLOCATOR_GLOBAL: u32 = 0x012fb1a0;
    const FREE_FLAG: u32 = 1;
    unsafe {
        lf_checker_rt::callee_thiscall!(DESTRUCTOR, u32, this);
        if disp & FREE_FLAG != 0 {
            let alloc = lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL).read_unaligned();
            lf_checker_rt::callee_thiscall!(OPERATOR_DELETE, u32, alloc, this);
        }
    }
    this
});
