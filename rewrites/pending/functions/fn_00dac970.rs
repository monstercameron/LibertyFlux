// original: 0x00dac970 CTaskComplexTrackEntity::~CTaskComplexTrackEntity
/// Install this class's vtable, release the referenced object held at +0x14
/// (when non-null) through the release helper without nulling the slot,
/// then tail-call the base destructor and return its answer.
export!(thiscall, rw_00dac970(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EF0454;
        const MEMBER: u32 = 0x14;
        *(this as *mut u32) = relocated(VTABLE);
        if *((this as *const u32).byte_add(MEMBER as usize)) != 0 {
            let _: u32 = callee_stdcall!(1, u32, this.wrapping_add(MEMBER));
        }
        callee_thiscall!(2, u32, this)
    }
});
