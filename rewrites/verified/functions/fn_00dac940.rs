// original: 0x00dac940 CTaskComplexGangHasslePed::~CTaskComplexGangHasslePed
/// Install this class's vtable, decrement the live-task counter, release the
/// referenced object held at +0x14 (when non-null) through the release helper
/// and null the slot, then tail-call the base destructor and return its answer.
export!(thiscall, rw_00dac940(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EF04AC;
        const MEMBER: u32 = 0x14;
        *(this as *mut u32) = relocated(VTABLE);
        let count = global::<u32>(0x017A6538);
        *count = (*count).wrapping_sub(1);
        let slot = (this as *mut u32).byte_add(MEMBER as usize);
        if *slot != 0 {
            let _: u32 = callee_stdcall!(1, u32, this.wrapping_add(MEMBER));
            *slot = 0;
        }
        callee_thiscall!(2, u32, this)
    }
});
