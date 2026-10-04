// original: 0x00dada70 CTaskComplexAvoidPlayerTargetting::~CTaskComplexAvoidPlayerTargetting__deleting
/// Scalar deleting destructor: run the class destructor, then free this
/// object through the allocator global when bit 0 of the flags word is set.
/// Returns the object pointer in both cases.
export!(thiscall, rw_00dada70(this: u32, flags: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this);
        if flags & 1 != 0 {
            let alloc = *global::<u32>(0x0167E2A0);
            let _: u32 = callee_thiscall!(2, u32, alloc, this);
        }
        this
    }
});
