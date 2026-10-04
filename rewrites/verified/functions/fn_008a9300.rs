// original: 0x008a9300 rage::audEffect::audEffect_2
/// Constructor: set the vtable, release the previous inner object.
///
/// Installs this class's vtable, then when the pointer at +0x8 is non-null,
/// calls its slot-0 method with 1 (release). Logically void; the contract
/// does not compare EAX (entry garbage on the null path).
export!(thiscall, rw_008a9300(this: *mut u8) -> u32 {
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
        0
    }
});
