// original: 0x0089ddb0 rage::audWrapperSound::audWrapperSound
/// `rage::audWrapperSound` constructor: base-construct and install vtable.
/// thiscall/0, returns `this`.
export!(thiscall, rw_0089ddb0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xE7A184;
        
        callee_thiscall!(1, u32, this);
        *(this as *mut u32) = relocated(VTABLE);
        this
    }
});
