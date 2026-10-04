// original: 0x00b00b40 CViewport::~CViewport__deleting
/// Scalar deleting destructor: run the destructor and free the object when
/// the low flag bit requests it. Returns the object pointer.
export!(thiscall, rw_00b00b40(this: u32, flags: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this);
        if flags & 1 != 0 {
            callee_cdecl!(2, u32, this);
        }
        this
    }
});
