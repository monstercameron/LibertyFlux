// original: 0x008DB2F0 task_ctor_initialize_motion_task
/// Initialises a motion-task object, calls its base initializer, and forwards
/// the resource identifier, two object fields and float bits to the task
/// helper. The helper writes one word to each of the object's +0x398 and
/// +0x39c fields. The function then reads the subject's word at +0x58,
/// invokes the per-task helper and registers the task with the shared manager.
/// The float is carried as raw 32-bit bits; no arithmetic is performed.
/// Unused stack parameters remain in the signature to preserve the 32-bit
/// thiscall layout and the ten-word callee cleanup.
lf_checker_rt::export!(thiscall, rw_008db2f0(
    this: u32,
    task_code: u32,
    subject: u32,
    resource_id: u32,
    field_value: u32,
    byte_a: u32,
    byte_b: u32,
    byte_c: u32,
    secondary_subject: u32,
    float_bits: u32,
    _unused_9: u32,
) -> u32 {
    unsafe {
        const COOKIE_MASK: u32 = 0x3fff;
        const COOKIE_MARKER: u32 = 0x00e7e048;
        const OBJECT_VTABLE: u32 = 0x00e81894;
        const COUNTER_FILE_VA: u32 = 0x010327a0;
        const MANAGER_CONTEXT_FILE_VA: u32 = 0x011d4ec8;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn write_byte(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        let counter = lf_checker_rt::global::<u32>(COUNTER_FILE_VA);
        let current_count = counter.read_unaligned();
        let old_cookie = read_word(this.wrapping_add(4));
        write_word(this, lf_checker_rt::relocated(COOKIE_MARKER));
        write_word(
            this.wrapping_add(4),
            old_cookie ^ ((old_cookie ^ current_count) & COOKIE_MASK),
        );
        counter.write_unaligned(current_count.wrapping_add(1));
        write_word(this, lf_checker_rt::relocated(OBJECT_VTABLE));

        let base_context = this.wrapping_add(0x20);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, base_context);

        write_byte(this.wrapping_add(0x0a), byte_a as u8);
        write_byte(this.wrapping_add(0x0b), byte_b as u8);
        write_byte(this.wrapping_add(0x0c), byte_c as u8);
        write_word(this.wrapping_add(0x390), field_value);
        write_word(this.wrapping_add(0x3a0), secondary_subject);
        write_word(this.wrapping_add(0x39c), u32::MAX);
        (this.wrapping_add(8) as *mut u16).write_unaligned(task_code as u16);

        let _ = lf_checker_rt::callee_cdecl!(
            2,
            u32,
            resource_id,
            this.wrapping_add(0x398),
            this.wrapping_add(0x39c),
            0u32,
            1u32,
            0u32,
            task_code,
            float_bits,
        );

        write_word(this.wrapping_add(0x10), subject);
        let subject_value = read_word(subject.wrapping_add(0x58));
        let _ = lf_checker_rt::callee_thiscall!(3, u32, base_context, subject_value);
        write_word(this.wrapping_add(0x14), _unused_9);

        let manager_context = lf_checker_rt::relocated(MANAGER_CONTEXT_FILE_VA);
        let _ = lf_checker_rt::callee_thiscall!(4, u32, manager_context, task_code);
        this
    }
});
