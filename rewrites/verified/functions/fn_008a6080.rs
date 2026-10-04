// original: 0x008a6080 rage::audMathOperationSound::~audMathOperationSound__deleting
/// Scalar deleting destructor: run the destructor, maybe unregister.
///
/// Calls the destructor helper, then when the low bit of the flags argument
/// is set and the object pointer is non-null, forwards (object, index byte
/// at +0x40) to the table owner. Returns the object pointer.
export!(thiscall, rw_008a6080(this: *mut u8, flags: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        if (flags & 1) != 0 && !(this as *const u8).is_null() {
            let index = *this.add(0x40) as u32;
            let _: u32 = callee_thiscall!(2, u32, relocated(0x115d8a0), this as u32, index);
        }
        this as u32
    }
});
