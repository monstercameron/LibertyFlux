// original: 0x00cca110 CTaskComplexRevive::CTaskComplexRevive
/// Revive-task constructor: base-construct, stamp the vtable, store the two
/// parameters at +0x14/+0x18, and run the handle helper (thiscall/1, id 2)
/// once per non-null parameter with (arg, &field). Returns `this`.
export!(thiscall, rw_00cca110(this: *mut u8, a0: u32, a1: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEDA0DC;
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x14)) as *mut u32) = a0;
        *((this.add(0x18)) as *mut u32) = a1;
        if a0 != 0 {
            let field = (this.add(0x14)) as u32;
            let _: u32 = callee_thiscall!(2, u32, a0, field);
        }
        if a1 != 0 {
            let field = (this.add(0x18)) as u32;
            let _: u32 = callee_thiscall!(2, u32, a1, field);
        }
        this as u32
    }
});
