// original: 0x00B7AF60 CTaskComplexCarDriveBasic::clone

/// Clone the basic car-drive task and make the clone's low mode bit match the source while preserving its other flag bits.
///
/// `this` points to the source task. All task fields are read at their byte
/// offsets below; signed byte and word fields are sign-extended before they
/// reach the constructor, while unsigned fields are zero-extended. The task
/// pool allocator and constructor are outgoing calls intercepted by the
/// contract. The returned pointer is the constructor's result, or zero when
/// allocation returns null.
lf_checker_rt::export!(thiscall, rw_00b7af60(this: u32) -> u32 {
    const TASK_POOL_GLOBAL: u32 = 0x0167E2A0;
    const ALLOCATOR: u32 = 1;
    const CONSTRUCTOR: u32 = 2;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_i8(address: u32) -> i8 { unsafe { (address as *const i8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_i16(address: u32) -> i16 { unsafe { (address as *const i16).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) { unsafe { (address as *mut u8).write_unaligned(value) } }

    unsafe {
        let pool_manager = lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL).read_unaligned();
        let allocated = lf_checker_rt::callee_thiscall!(ALLOCATOR, u32, pool_manager);
        if allocated == 0 {
            let _ = read_u8(0x28);
            return 0;
        }
        
        let copy = lf_checker_rt::callee_thiscall!(CONSTRUCTOR, u32, allocated, read_u32(this.wrapping_add(0x14)), 0);
        let source_flags = read_u8(this.wrapping_add(0x28));
        let clone_flags = read_u8(copy.wrapping_add(0x28));
        write_u8(copy.wrapping_add(0x28), clone_flags ^ ((source_flags ^ clone_flags) & 1));
        copy
    }
});
