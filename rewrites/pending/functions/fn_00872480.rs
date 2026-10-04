// original: 0x00872480 rage::crmtManager::~crmtManager__deleting
//! Attach a child to a motion manager: keep the child pointer at `this+4`
//! and hand it to slot 4 of its own function table. Returns that call's
//! answer. The second stack argument is ignored.
export!(thiscall, rw_00872480(this: *mut u8, child: u32, _unused: u32) -> u32 {
    unsafe {
        *(this.add(4) as *mut u32) = child;
        let vt = *(child as *const u32);
        let attach: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(4)) as *const u32) as usize);
        attach(child)
    }
});
