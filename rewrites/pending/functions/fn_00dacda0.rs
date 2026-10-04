// original: 0x00dacda0 CTaskComplexGangHasslePed::vf1
/// Clone helper: allocate a fresh object through the allocator global (null
/// when allocation fails), otherwise construct the copy from the member at
/// +0x14 and return the constructor's answer.
export!(thiscall, rw_00dacda0(this: u32) -> u32 {
    unsafe {
        let alloc = *global::<u32>(0x0167E2A0);
        let obj: u32 = callee_thiscall!(1, u32, alloc);
        if obj == 0 {
            return 0;
        }
        let member = *((this as *const u32).byte_add(0x14));
        callee_thiscall!(2, u32, obj, member)
    }
});
