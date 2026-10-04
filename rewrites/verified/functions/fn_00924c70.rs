// original: 0x00924C70 input_item_create
/// Create an item for key `a0`: register, allocate, tag and publish it.
///
/// Key -1 does nothing and returns -1. Otherwise registers the key pair,
/// allocates a 0x10-byte object, tags it from global `0x010327A0` (masked
/// to 14 bits, counter bumped), fills the table pointers and the word from
/// `0x0119CFFC`, then publishes the object (or null when allocation failed).
export!(cdecl, rw_00924C70(a0: u32, a1: u32) -> u32 {
    unsafe {
        if a0 == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        callee_cdecl!(1, u32, a0, a1);
        let obj = callee_cdecl!(2, u32, 0x10, 0);
        if obj == 0 {
            callee_cdecl!(3, u32, 0);
            return 0;
        }
        let o = obj as *mut u32;
        let counter = global::<u32>(0x10327A0);
        let tag = *o.add(1) ^ *counter;
        *o.add(0) = relocated(0xE7E048);
        *o.add(1) ^= tag & 0x3FFF;
        *counter = (*counter).wrapping_add(1);
        *o.add(0) = relocated(0xE862C0);
        *o.add(2) = relocated(0x62BF30);
        *o.add(3) = *global::<u32>(0x119CFFC);
        callee_cdecl!(3, u32, obj);
        0
    }
});
