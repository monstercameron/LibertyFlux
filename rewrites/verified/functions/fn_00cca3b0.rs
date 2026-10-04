// original: 0x00cca3b0 CTaskComplexDie::~CTaskComplexDie
/// Die-task destructor: stamp the vtable, release the owner handle at +0x2C
/// through the handle helper (thiscall/1, id 1) and clear it, release the
/// child at +0x30 through virtual slot 0 with argument 1 and clear it, then
/// tail into the base destructor (id 3), whose answer is returned.
export!(thiscall, rw_00cca3b0(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xED9F24;
        *(this as *mut u32) = relocated(VTABLE);
        let handle = *((this.add(0x2C)) as *const u32);
        if handle != 0 {
            let field = (this.add(0x2C)) as u32;
            let _: u32 = callee_thiscall!(1, u32, handle, field);
            *((this.add(0x2C)) as *mut u32) = 0;
        }
        let child = *((this.add(0x30)) as *const u32);
        if child != 0 {
            let vtable = *(child as *const u32);
            let slot = *(vtable as *const u32);
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            let _: u32 = release(child, 1);
            *((this.add(0x30)) as *mut u32) = 0;
        }
        callee_thiscall!(3, u32, this as u32)
    }
});
