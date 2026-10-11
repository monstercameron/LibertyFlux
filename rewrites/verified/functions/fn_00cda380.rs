// original: 0x00CDA380 script_vm_resource_task_init (proposed)

/// Initialize the resource-bearing task object from seven stack arguments.
///
/// `this` is the destination; the first two values are forwarded to its base
/// initializer, `resource_id` and `mode` occupy fields at `+0x28` and `+0x2c`,
/// `first_record` and `second_record` point to three-word records copied to
/// `+0x40..+0x48` and `+0x50..+0x58`, and `resource` is stored at `+0x30`.
/// The embedded handle at `+0x64` is initialized before the optional resource
/// release and id update. The update is skipped when `resource_id` is
/// `u32::MAX`; the release is skipped when `resource` is null. The function
/// installs the task vtable and returns `this` in EAX. The object layout and
/// pointer fields follow the original 32-bit ABI.
lf_checker_rt::export!(thiscall, rw_00cda380(
    this: u32,
    base_arg_a: u32,
    base_arg_b: u32,
    resource_id: u32,
    mode: u32,
    first_record: u32,
    resource: u32,
    second_record: u32,
) -> u32 {
    unsafe {
        const OBJECT_VTABLE: u32 = 0x00ED_D59C;
        const RESOURCE_ID: u32 = 0x28;
        const MODE: u32 = 0x2c;
        const RESOURCE: u32 = 0x30;
        const FIRST_WORDS: u32 = 0x40;
        const HANDLE: u32 = 0x64;
        const NO_RESOURCE_ID: u32 = u32::MAX;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        lf_checker_rt::callee_thiscall!(1, u32, this, base_arg_a, base_arg_b);

        write_word(this, lf_checker_rt::relocated(OBJECT_VTABLE));
        write_word(this + RESOURCE_ID, resource_id);
        write_word(this + MODE, mode);
        write_word(this + RESOURCE, resource);
        for word in 0..3u32 {
            write_word(
                this + FIRST_WORDS + word * 4,
                read_word(first_record + word * 4),
            );
            write_word(
                this + 0x50 + word * 4,
                read_word(second_record + word * 4),
            );
        }
        (this.wrapping_add(0x60) as *mut u8).write(0);

        lf_checker_rt::callee_thiscall!(2, u32, this + HANDLE);
        if resource != 0 {
            lf_checker_rt::callee_thiscall!(3, u32, resource, this + RESOURCE);
        }
        if resource_id != NO_RESOURCE_ID {
            lf_checker_rt::callee_thiscall!(4, u32, this + HANDLE, resource_id);
        }
        this
    }
});
