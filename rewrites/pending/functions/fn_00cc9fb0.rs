// original: 0x00cc9fb0 CTaskComplexDie::CTaskComplexDie
/// Die-task constructor: base-construct, store the 7 parameters, stamp the
/// vtable, clear the state words, and register the owner handle.
///
/// Layout: +0x14/+0x18/+0x1C take integer args 1..3, +0x20/+0x24 take float
/// args 4..5 as bit patterns, +0x28 takes arg 6's low byte, +0x2C takes arg
/// 0. Words at +0x30/+0x40/+0x44/+0x48/+0x54 and bytes at +0x29/+0x50 are
/// cleared. When arg 0 is non-null the handle helper (thiscall/1, id 2)
/// runs with (arg0, &field_2C). Returns `this`.
export!(thiscall, rw_00cc9fb0(this: *mut u8, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xED9F24;
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        *((this.add(0x14)) as *mut u32) = a1;
        *((this.add(0x18)) as *mut u32) = a2;
        *((this.add(0x1C)) as *mut u32) = a3;
        *((this.add(0x20)) as *mut u32) = a4;
        *((this.add(0x24)) as *mut u32) = a5;
        *this.add(0x28) = a6 as u8;
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x2C)) as *mut u32) = a0;
        *this.add(0x29) = 0;
        *((this.add(0x30)) as *mut u32) = 0;
        *((this.add(0x40)) as *mut u32) = 0;
        *((this.add(0x44)) as *mut u32) = 0;
        *((this.add(0x48)) as *mut u32) = 0;
        *this.add(0x50) = 0;
        *((this.add(0x54)) as *mut u32) = 0;
        if a0 != 0 {
            let field = (this.add(0x2C)) as u32;
            let _: u32 = callee_thiscall!(2, u32, a0, field);
        }
        this as u32
    }
});
