// original: 0x00cca430 CTaskComplexMoveAboutInjured::~CTaskComplexMoveAboutInjured
/// Move-about-injured-task destructor: stamp the vtable, release the handle
/// at +0x14 through the handle helper (thiscall/1, id 1) and clear it, then
/// tail into the base destructor (id 2), whose answer is returned.
export!(thiscall, rw_00cca430(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEDA084;
        *(this as *mut u32) = relocated(VTABLE);
        let handle = *((this.add(0x14)) as *const u32);
        if handle != 0 {
            let field = (this.add(0x14)) as u32;
            let _: u32 = callee_thiscall!(1, u32, handle, field);
            *((this.add(0x14)) as *mut u32) = 0;
        }
        callee_thiscall!(2, u32, this as u32)
    }
});
