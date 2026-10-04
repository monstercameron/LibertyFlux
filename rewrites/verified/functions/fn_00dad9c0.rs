// original: 0x00dad9c0 CTaskComplexAvoidPlayerTargetting::~CTaskComplexAvoidPlayerTargetting
/// Install this class's vtable, release the referenced objects held at
/// +0x14/+0x18 (each when non-null) through the release helper, then
/// tail-call the base destructor and return its answer.
export!(thiscall, rw_00dad9c0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EF07E4;
        *(this as *mut u32) = relocated(VTABLE);
        if *((this as *const u32).byte_add(0x14)) != 0 {
            let _: u32 = callee_stdcall!(1, u32, this.wrapping_add(0x14));
        }
        if *((this as *const u32).byte_add(0x18)) != 0 {
            let _: u32 = callee_stdcall!(1, u32, this.wrapping_add(0x18));
        }
        callee_thiscall!(2, u32, this)
    }
});
