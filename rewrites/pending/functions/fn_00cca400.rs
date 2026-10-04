// original: 0x00cca400 CTaskComplexInjuredOnGround::~CTaskComplexInjuredOnGround
/// Injured-on-ground-task destructor: stamp the vtable, when the field at
/// +0x14 is set run the float notifier (thiscall/1, id 1) with -16.0f bits,
/// then tail into the base destructor (id 2), whose answer is returned.
export!(thiscall, rw_00cca400(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xED9E74;
        const LEVEL_BITS: u32 = 0xC100_0000;
        *(this as *mut u32) = relocated(VTABLE);
        if *((this.add(0x14)) as *const u32) != 0 {
            let _: u32 = callee_thiscall!(1, u32, this as u32, LEVEL_BITS);
        }
        callee_thiscall!(2, u32, this as u32)
    }
});
