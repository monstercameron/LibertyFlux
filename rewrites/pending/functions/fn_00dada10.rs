// original: 0x00dada10 CTaskComplexInvestigateDeadPed::~CTaskComplexInvestigateDeadPed
/// Install this class's vtable, release the referenced object held at
/// +0x14 (when non-null) through the release helper, then tail-call
/// the base destructor and return its answer.
export!(thiscall, rw_00dada10(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EF06DC;
        *(this as *mut u32) = relocated(VTABLE);
        if *((this as *const u32).byte_add(0x14)) != 0 {
            let _: u32 = callee_stdcall!(1, u32, this.wrapping_add(0x14));
        }
        callee_thiscall!(2, u32, this)
    }
});
