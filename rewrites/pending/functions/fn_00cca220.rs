// original: 0x00cca220 complex_task_ctor_i2f5
/// Task constructor taking 2 integer and 5 float parameters: base-construct
/// (id 1), store the integers at +0x14/+0x18 and the float bit patterns at
/// +0x20/+0x24/+0x28/+0x2C/+0x30, stamp the vtable, clear +0x1C/+0x34.
/// Returns `this`. (No merged symbol; name proposed.)
export!(thiscall, rw_00cca220(this: *mut u8, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xED9E1C;
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        *((this.add(0x20)) as *mut u32) = a2;
        *((this.add(0x24)) as *mut u32) = a3;
        *((this.add(0x28)) as *mut u32) = a4;
        *((this.add(0x2C)) as *mut u32) = a5;
        *((this.add(0x14)) as *mut u32) = a0;
        *((this.add(0x18)) as *mut u32) = a1;
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x1C)) as *mut u32) = 0;
        *((this.add(0x30)) as *mut u32) = a6;
        *this.add(0x34) = 0;
        this as u32
    }
});
