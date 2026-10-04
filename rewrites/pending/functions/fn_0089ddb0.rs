// original: 0x0089ddb0 rage::audWrapperSound::audWrapperSound
/// `rage::audWrapperSound` constructor: base-construct and install vtable.
/// thiscall/0, returns `this`.
export!(thiscall, rw_0089ddb0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xE7A184;
        let base: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        base(this);
        *(this as *mut u32) = relocated(VTABLE);
        this
    }
});
