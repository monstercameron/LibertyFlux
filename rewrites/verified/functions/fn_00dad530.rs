// original: 0x00dad530 CTaskComplexGangHasslePed::vf19
/// Copy sixteen bytes (one dword, two single-precision bit patterns, one
/// dword) from the nested structure reached through the argument into
/// +0x20..+0x2F of this object, then allocate and construct a fresh
/// sub-task through the allocator global and return it, or null when
/// allocation fails.
export!(thiscall, rw_00dad530(this: u32, arg: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EB391C;
        let inner = *((arg as *const u32).byte_add(0x20));
        let src = (inner as *const u32).byte_add(0x30);
        let dst = (this as *mut u32).byte_add(0x20);
        *dst.add(0) = *src.add(0);
        *dst.add(1) = *src.add(1);
        *dst.add(2) = *src.add(2);
        *dst.add(3) = *src.add(3);
        let alloc = *global::<u32>(0x0167E2A0);
        let obj: u32 = callee_thiscall!(1, u32, alloc);
        if obj == 0 {
            return 0;
        }
        let _: u32 = callee_thiscall!(2, u32, obj);
        *(obj as *mut u32) = relocated(VTABLE);
        *((obj as *mut u32).byte_add(0x14)) = 0;
        *((obj as *mut u8).byte_add(0x18)) = 0;
        *((obj as *mut u32).byte_add(0x1C)) = 0;
        obj
    }
});
