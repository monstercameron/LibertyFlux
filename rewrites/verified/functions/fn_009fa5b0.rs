// original: 0x009fa5b0 CPlayStatBase::~CPlayStatBase__deleting
/// Scalar deleting destructor for the play-stat base object.
///
/// Runs the base destructor, then frees the object through the global
/// operator delete when bit 0 of the placement flag is set. Always returns
/// the object pointer.
export!(thiscall, rw_009fa5b0(this_ptr: u32, flag: u32) -> u32 {
    unsafe {
        const DELETE_IF_SET: u32 = 1;
        callee_thiscall!(1, u32, this_ptr);
        if flag & DELETE_IF_SET != 0 {
            callee_cdecl!(2, u32, this_ptr);
        }
        this_ptr
    }
});
