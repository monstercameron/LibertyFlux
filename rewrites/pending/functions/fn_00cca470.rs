// original: 0x00cca470 CTaskComplexRevive::~CTaskComplexRevive
/// Revive-task destructor: stamp the vtable, release both handles at
/// +0x14/+0x18 through the handle helper (thiscall/1, id 1), clearing each,
/// then tail into the base destructor (id 2), whose answer is returned.
export!(thiscall, rw_00cca470(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEDA0DC;
        *(this as *mut u32) = relocated(VTABLE);
        let first = *((this.add(0x14)) as *const u32);
        if first != 0 {
            let field = (this.add(0x14)) as u32;
            let _: u32 = callee_thiscall!(1, u32, first, field);
            *((this.add(0x14)) as *mut u32) = 0;
        }
        let second = *((this.add(0x18)) as *const u32);
        if second != 0 {
            let field = (this.add(0x18)) as u32;
            let _: u32 = callee_thiscall!(1, u32, second, field);
            *((this.add(0x18)) as *mut u32) = 0;
        }
        callee_thiscall!(2, u32, this as u32)
    }
});
