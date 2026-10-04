// original: 0x009d0720 fetch_locked_field_pair
/// Fetch two locked context fields into two caller out-pointers.
///
/// Reads the words at offsets 0x378 and 0x3c0 of fresh context pointers
/// and stores them through the two out-pointers; returns the second one.
/// The lock calls are made through addresses loaded from their import
/// slots, exactly like the original.
export!(cdecl, rw_009d0720(out0: u32, out1: u32) -> u32 {
    const LOCK_FLAG: u32 = 0x0103AD58;
    const LOCK_CS: u32 = 0x0129588C;
    const ENTER_SLOT: u32 = 0x00E731CC;
    const LEAVE_SLOT: u32 = 0x00E731C8;
    let flag = global::<u8>(LOCK_FLAG);
    let p: u32 = callee_cdecl!(3, u32,);
    if unsafe { flag.read() } != 0 {
        let addr = unsafe { global::<u32>(ENTER_SLOT).read() };
        let enter: extern "stdcall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(addr as usize) };
        enter(relocated(LOCK_CS));
    }
    let guarded = unsafe { flag.read() };
    let first = unsafe { (p.wrapping_add(0x378) as *const u32).read() };
    if guarded != 0 {
        let addr = unsafe { global::<u32>(LEAVE_SLOT).read() };
        let leave: extern "stdcall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(addr as usize) };
        leave(relocated(LOCK_CS));
    }
    unsafe { (out0 as *mut u32).write(first) };
    let q: u32 = callee_cdecl!(3, u32,);
    if unsafe { flag.read() } != 0 {
        let addr = unsafe { global::<u32>(ENTER_SLOT).read() };
        let enter: extern "stdcall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(addr as usize) };
        enter(relocated(LOCK_CS));
    }
    let guarded2 = unsafe { flag.read() };
    let second = unsafe { (q.wrapping_add(0x3c0) as *const u32).read() };
    if guarded2 != 0 {
        let addr = unsafe { global::<u32>(LEAVE_SLOT).read() };
        let leave: extern "stdcall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(addr as usize) };
        leave(relocated(LOCK_CS));
    }
    unsafe { (out1 as *mut u32).write(second) };
    out1
});
