// original: 0x00B49D00 CEventCommunicateEvent::vf4

/// Clone the communicate event: allocate from the event pool, ask the stored speaker object through its virtual slot for the message word, and construct the copy from the stored word and that answer.
///
/// Indirect calls go through fabricated heap objects exactly like the
/// original, landing on the contract's planted stubs.
lf_checker_rt::export!(thiscall, rw_00b49d00(this: u32) -> u32 {
    const EVENT_POOL_GLOBAL: u32 = 0x016669C8;
    const ALLOCATOR: u32 = 1;
    const CONSTRUCTOR: u32 = 3;
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    unsafe {
        let pool_manager = lf_checker_rt::global::<u32>(EVENT_POOL_GLOBAL).read_unaligned();
        let allocated = lf_checker_rt::callee_thiscall!(ALLOCATOR, u32, pool_manager);
        if allocated == 0 {
            return 0;
        }
        let speaker = read_u32(this.wrapping_add(0x10));
        let vtable = read_u32(speaker);
        let slot = read_u32(vtable.wrapping_add(0x10));
        let ask: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        let message = ask(speaker, read_u32(this.wrapping_add(0x14)), read_u32(this.wrapping_add(0x18)),
            this.wrapping_add(0x20));
        lf_checker_rt::callee_thiscall!(CONSTRUCTOR, u32, allocated, read_u32(this.wrapping_add(0x0C)), message)
    }
});
