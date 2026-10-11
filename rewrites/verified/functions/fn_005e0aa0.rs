// original: 0x005E0AA0 CTaskComplexUseWaterCannon::create_next_sub_task

/// Query the current child through its virtual method at vtable offset +0x0c,
/// discard that method's result, and return a null successor. The extra
/// character argument is preserved on the thiscall stack but is not read.
lf_checker_rt::export!(thiscall, rw_005e0aa0(this: u32, _character: u32) -> u32 {
    unsafe {
        const CURRENT_TASK: u32 = 0x08;
        const TASK_TYPE_SLOT: u32 = 0x0c;
        type GetTaskValue = extern "thiscall" fn(u32) -> u32;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let current_task = read_u32(this.wrapping_add(CURRENT_TASK));
        let vtable = read_u32(current_task);
        let get_task_value: GetTaskValue = core::mem::transmute(
            read_u32(vtable.wrapping_add(TASK_TYPE_SLOT)) as usize,
        );
        let _ = get_task_value(current_task);
        0
    }
});
