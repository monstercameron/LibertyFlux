// original: 0x00c67160 cutscene_setup_and_arm
/// Brings the object up (state 2, setup call, two fetch rounds) and arms it.
///
/// Each fetch round resolves a value through vtable slot 0xa0 (falling
/// back to [this+0x100], resolving through slot 0xe0 otherwise). A zero
/// first value runs the fallback callee 4; a nonzero second value sets
/// flag bit 1 at 0xf4 and runs arming callee 5, otherwise byte 0x2a9 is
/// cleared. Both ends store the same constant word and byte. Returns the
/// arming call's result, or 0 on the disarmed path.
export!(thiscall, rw_c67160(this: u32) -> u32 {
    /// Constant stored at 0x50 on both exit paths (1000.0 as float bits).
    const READY_WORD: u32 = 0x447a_0000;
    let vtable = unsafe { (this as *const u32).read() };
    unsafe {
        ((this as *mut u32).byte_add(0x314)).write(2);
    }
    let setup_slot = unsafe { (vtable as *const u32).byte_add(0x40).read() };
    let setup: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(setup_slot as usize) };
    setup(this);
    let fetch_slot = unsafe { (vtable as *const u32).byte_add(0xa0).read() };
    let fetch: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(fetch_slot as usize) };
    let first = if fetch(this) == 0 {
        unsafe { (this as *const u32).byte_add(0x100).read() }
    } else {
        let inner = fetch(this);
        let inner_table = unsafe { (inner as *const u32).read() };
        let inner_slot =
            unsafe { (inner_table as *const u32).byte_add(0xe0).read() };
        let resolve: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(inner_slot as usize) };
        resolve(inner)
    };
    if first == 0 {
        callee_thiscall!(4, u32, this);
    }
    let second = if fetch(this) == 0 {
        unsafe { (this as *const u32).byte_add(0x100).read() }
    } else {
        let inner = fetch(this);
        let inner_table = unsafe { (inner as *const u32).read() };
        let inner_slot =
            unsafe { (inner_table as *const u32).byte_add(0xe0).read() };
        let resolve: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(inner_slot as usize) };
        resolve(inner)
    };
    if second == 0 {
        unsafe {
            ((this as *mut u8).byte_add(0x2a9)).write(0);
            ((this as *mut u32).byte_add(0x50)).write(READY_WORD);
            ((this as *mut u8).byte_add(0x63)).write(0xff);
        }
        return 0;
    }
    unsafe {
        let flag = (this as *mut u8).byte_add(0xf4);
        flag.write(flag.read() | 2);
    }
    let armed = callee_thiscall!(5, u32, this, 0);
    unsafe {
        ((this as *mut u32).byte_add(0x50)).write(READY_WORD);
        ((this as *mut u8).byte_add(0x63)).write(0xff);
    }
    armed
});
