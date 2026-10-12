// original: 0x00B49D80 CEventGivePedTask::vf4

/// Clone the give-ped-task event: read the task word through the event's own virtual slot, allocate from the event pool, and construct the copy from the stored owner, that word and the stored flag byte.
///
/// Indirect calls go through fabricated heap objects exactly like the
/// original, landing on the contract's planted stubs.
lf_checker_rt::export!(thiscall, rw_00b49d80(this: u32) -> u32 {
    const EVENT_POOL_GLOBAL: u32 = 0x016669C8;
    const ALLOCATOR: u32 = 1;
    const CONSTRUCTOR: u32 = 3;
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    unsafe {
        let vtable = read_u32(this);
        let slot = read_u32(vtable.wrapping_add(0x54));
        let task_of: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
        let task = task_of(this);
        let pool_manager = lf_checker_rt::global::<u32>(EVENT_POOL_GLOBAL).read_unaligned();
        let allocated = lf_checker_rt::callee_thiscall!(ALLOCATOR, u32, pool_manager);
        if allocated == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CONSTRUCTOR, u32, allocated, read_u32(this.wrapping_add(0x0C)), task,
            read_u8(this.wrapping_add(0x14)) as u32)
    }
});
