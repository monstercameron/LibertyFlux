// original: 0x00a95a80 fiStreamingDevice_deleting_dtor
/// Scalar deleting destructor of the streaming-device object.
///
/// Restamps the object's function table, then frees the object through the
/// heap when the low bit of the destructor flags is set. Returns the object
/// pointer in all cases.
export!(thiscall, rw_00a95a80(this: u32, flags: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xE7F81C);
        if flags & 1 != 0 {
            callee_cdecl!(1, u32, this);
        }
        this
    }
});
