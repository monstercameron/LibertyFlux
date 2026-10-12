// original: 0x00B49E40 CEventObjectCollision::vf4

/// Clone the object-collision event from its stored words, float bits and substructure pointers, then copy the one-byte state flag onto the clone.
///
/// `this` points to the source event. Fields are read at their byte offsets
/// below; the event pool allocator and the copy constructor are outgoing
/// calls intercepted by the contract. The returned pointer is the
/// constructor's result, or zero when allocation returns null.
lf_checker_rt::export!(thiscall, rw_00b49e40(this: u32) -> u32 {
    const EVENT_POOL_GLOBAL: u32 = 0x016669C8;
    const ALLOCATOR: u32 = 1;
    const CONSTRUCTOR: u32 = 2;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_i16(address: u32) -> i16 { unsafe { (address as *const i16).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) { unsafe { (address as *mut u8).write_unaligned(value) } }

    unsafe {
        let pool_manager = lf_checker_rt::global::<u32>(EVENT_POOL_GLOBAL).read_unaligned();
        let allocated = lf_checker_rt::callee_thiscall!(ALLOCATOR, u32, pool_manager);
        if allocated == 0 {
            return 0;
        }
        let copy = lf_checker_rt::callee_thiscall!(CONSTRUCTOR, u32, allocated, read_u16(this.wrapping_add(0x10)) as u32, read_u32(this.wrapping_add(0x0C)), read_u32(this.wrapping_add(0x14)), this.wrapping_add(0x20), this.wrapping_add(0x30), read_i16(this.wrapping_add(0x12)) as u32);
        write_u8(copy.wrapping_add(0x40), read_u8(this.wrapping_add(0x40))); copy
    }
});
