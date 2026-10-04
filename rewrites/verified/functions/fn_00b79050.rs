// original: 0x00b79050 CTaskComplex::~CTaskComplex__deleting
/// Deleting destructor for `CTaskComplex`.
///
/// Runs the destructor, then frees the object through the global allocator
/// when bit 0 of the flags argument is set. Returns the object pointer.
export!(thiscall, rw_00b79050(this: u32, flags: u32) -> u32 {
    let _: u32 = callee_thiscall!(1, u32, this);
    if flags & 1 != 0 {
        let alloc = unsafe { *global::<u32>(0x0167E2A0) };
        let _: u32 = callee_thiscall!(2, u32, alloc, this);
    }
    this
});
