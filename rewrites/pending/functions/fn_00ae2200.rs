// original: 0x00ae2200 ui_construct_shared_callback
/// Construct a shared callback object from its parts.
///
/// Installs the base vtable, mixes the old tag word with the shared counter
/// (then bumps the counter), stores the id, installs the final vtable,
/// copies the handler pointer and the twelve style words from the template,
/// and copies the flag byte. Returns the object.
export!(thiscall, rw_00ae2200(this: *mut u8, id: u32, handler: *const u32, template: *const u32, flags: *const u8) -> u32 {
    unsafe {
        const VT_BASE: u32 = 0x00E7E048;
        const VT_FINAL: u32 = 0x00EA77B8;
        const COUNTER: u32 = 0x010327A0;
        let t = this;
        let tag = t.add(4) as *mut u32;
        let old = *tag;
        *(t as *mut u32) = relocated(VT_BASE);
        let ctr = global::<u32>(COUNTER);
        *tag ^= (old ^ *ctr) & 0x3fff;
        *ctr = (*ctr).wrapping_add(1);
        *(t.add(8) as *mut u32) = id;
        *(t as *mut u32) = relocated(VT_FINAL);
        *(t.add(0x0c) as *mut u32) = *handler;
        let copy = |dst: u32, src: u32| {
            *(t.add(dst as usize) as *mut u32) = *((template as *const u8).add(src as usize) as *const u32);
        };
        copy(0x10, 0x00);
        copy(0x14, 0x04);
        copy(0x18, 0x08);
        copy(0x20, 0x10);
        copy(0x24, 0x14);
        copy(0x28, 0x18);
        copy(0x30, 0x20);
        copy(0x34, 0x24);
        copy(0x38, 0x28);
        copy(0x40, 0x30);
        copy(0x44, 0x34);
        copy(0x48, 0x38);
        *t.add(0x50) = *flags;
        t as u32
    }
});
