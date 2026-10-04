// original: 0x00cca040 CTaskComplexHitResponse::CTaskComplexHitResponse
/// Hit-response-task constructor: base-construct, store the parameter at
/// +0x14, stamp the vtable. Returns `this`.
export!(thiscall, rw_00cca040(this: *mut u8, a0: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xED9FD4;
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        *((this.add(0x14)) as *mut u32) = a0;
        *(this as *mut u32) = relocated(VTABLE);
        this as u32
    }
});
