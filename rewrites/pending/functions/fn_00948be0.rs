// original: 0x00948be0 obj_activate_timed
/// Activate an object waiting in state 2: run the payload worker over its
/// buffer, stamp state 3, set its active flag and expiry time, and count
/// the activation globally. Null objects and objects in other states are
/// left alone. Returns the expiry stamp on the taken path.
export!(cdecl, rw_00948be0(obj: *mut u8) -> u32 {
    unsafe {
        if obj.is_null() || *obj.add(0x22A) != 2 {
            return 0;
        }
        callee_cdecl!(1, u32, obj.add(0x80) as u32);
        *obj.add(0x22A) = 3;
        let stamp = (*global::<u32>(0x011735B4)).wrapping_add(0xF4240);
        let flags = obj.add(0x118) as *mut u32;
        *flags |= 0x100000;
        *(obj.add(0x224) as *mut u32) = stamp;
        let counter = global::<u32>(0x01633468);
        *counter = (*counter).wrapping_add(1);
        stamp
    }
});
