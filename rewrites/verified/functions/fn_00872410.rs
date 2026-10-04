// original: 0x00872410 refcount_release
//! Release one reference on a counted node: decrement the count word at
//! `this+4` and return it. When the count reaches zero the node is deleted,
//! either through the owner hook at `this+0x18` (callee 1) or, when there
//! is none, through slot 0 of its own function table with argument 1.
export!(thiscall, rw_00872410(this: *mut u8) -> u32 {
    unsafe {
        let slot = this.add(4) as *mut u16;
        *slot = slot.read().wrapping_add(0xFFFF);
        if slot.read() != 0 {
            return slot.read() as u32;
        }
        let owner = *(this.add(0x18) as *const u32);
        if owner != 0 {
            callee_stdcall!(1, u32, this as u32);
        } else {
            let vt = *(this as *const u32);
            let delete: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute((*(vt as *const u32)) as usize);
            delete(this as u32, 1);
        }
        *(this.add(4) as *const u16) as u32
    }
});
