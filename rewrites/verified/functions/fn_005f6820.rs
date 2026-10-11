// original: 0x005F6820 rage::fragInst::vf42

/// Ask the manager's service for an attached fragment object. If the service
/// returns null, invoke the instance's failure virtual slot and return zero.
/// Otherwise invoke its setup slot, pass the instance fields and setup result
/// to the attachment helper, store the instance at result offset `0x100`, and
/// return the result pointer.
///
/// The manager pointer is read from the relocated global slot and its service
/// pointer is at `+0x2f0`. The instance's function table is the word at `+0`,
/// with failure and setup slots at `+0xf4` and `+0xe0`. The attachment helper
/// receives the data words at `+0x5c` and `+0x6c`, pointers to the instance
/// and its `+0x10` subobject, and three scalar values. This is a thiscall
/// method with no stack arguments.
lf_checker_rt::export!(thiscall, rw_005f6820(this: u32) -> u32 {
    const MANAGER_GLOBAL: u32 = 0x018B_7448;
    const MANAGER_SERVICE: u32 = 0x2f0;
    const INSTANCE_DATA: u32 = 0x5c;
    const INSTANCE_FLAGS: u32 = 0x6c;
    const INSTANCE_PARENT: u32 = 0x10;
    const ATTACHED_INSTANCE: u32 = 0x100;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let manager = core::ptr::read_unaligned(lf_checker_rt::global::<u32>(MANAGER_GLOBAL));
        let service = read_u32(manager.wrapping_add(MANAGER_SERVICE));
        let created = lf_checker_rt::callee_thiscall!(1, u32, service);
        if created == 0 {
            let _ = lf_checker_rt::callee_thiscall!(2, u32, this);
            return 0;
        }

        let method_value = lf_checker_rt::callee_thiscall!(3, u32, this);
        let data = read_u32(this.wrapping_add(INSTANCE_DATA));
        let parent = read_u32(this.wrapping_add(INSTANCE_FLAGS));
        let _ = lf_checker_rt::callee_thiscall!(
            4,
            u32,
            created,
            data,
            parent,
            this.wrapping_add(INSTANCE_PARENT),
            this,
            0xff,
            method_value,
            0,
        );
        write_u32(created.wrapping_add(ATTACHED_INSTANCE), this);
        created
    }
});
