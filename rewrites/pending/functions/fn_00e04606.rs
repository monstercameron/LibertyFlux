// original: 0x00e04606 attach_object_buffer
/// Attaches a 0x1000-byte buffer to a freshly validated object.
///
/// Resolves and validates the object through two lookup callees, locates
/// its registration slot (base+0x20 is slot 0, base+0x40 is slot 1), then
/// binds the slot's buffer (allocating it on first use) or falls back to
/// the object's inline area when allocation fails. Returns 1 on success,
/// 0 when validation fails or a conflicting flag is present.
export!(cdecl, rw_00e04606(obj: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x17AC2AC;
        const COUNT: u32 = 0x17AC3D4;
        const SIZE: u32 = 0x1000;
        let key = callee_cdecl!(1, u32, obj);
        if callee_cdecl!(2, u32, key) == 0 {
            return 0;
        }
        let base = callee_cdecl!(3, u32,);
        let slot: usize = if obj == base.wrapping_add(0x20) {
            0
        } else if obj == callee_cdecl!(3, u32,).wrapping_add(0x40) {
            1
        } else {
            return 0;
        };
        *global::<u32>(COUNT) = (*global::<u32>(COUNT)).wrapping_add(1);
        if *((obj + 0xC) as *const u32) & 0x10C != 0 {
            return 0;
        }
        let entry = (global::<[u32; 2]>(SLOTS) as *mut u32).add(slot);
        let buf = if *entry == 0 {
            let fresh = callee_cdecl!(4, u32, SIZE);
            *entry = fresh;
            fresh
        } else {
            *entry
        };
        if buf == 0 {
            let inline = obj.wrapping_add(0x14);
            *((obj + 8) as *mut u32) = inline;
            *(obj as *mut u32) = inline;
            *((obj + 0x18) as *mut u32) = 2;
            *((obj + 4) as *mut u32) = 2;
        } else {
            *((obj + 8) as *mut u32) = buf;
            *(obj as *mut u32) = buf;
            *((obj + 0x18) as *mut u32) = SIZE;
            *((obj + 4) as *mut u32) = SIZE;
        }
        *((obj + 0xC) as *mut u32) |= 0x1102;
        1
    }
});
