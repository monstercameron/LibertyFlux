// original: 0x008a9320 rage::audEffect::~audEffect__deleting
/// Scalar deleting destructor: release the inner object, maybe free self.
///
/// Installs the class vtable, releases the inner object at +0x8 through its
/// slot-0 method when non-null, frees this object when the flag argument's
/// low bit is set, and returns the object pointer.
export!(thiscall, rw_008a9320(this: *mut u8, flags: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xe7bc64);
        let inner = *(this.add(8) as *const u32);
        if inner != 0 {
            let vt = *(inner as *const u32);
            let tgt = *(vt as *const u32);
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            release(inner, 1);
        }
        if (flags & 1) != 0 {
            let _: u32 = callee_cdecl!(2, u32, this as u32);
        }
        this as u32
    }
});
