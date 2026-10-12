// original: 0x008db2f0 task_ctor_initialize_motion_task
/// Initialise a motion-task object (thiscall, ten stack words, callee cleanup
/// 0x28) and return the object pointer. Entry in ECX is the object; stack
/// words s0..s9 are: task code, subject pointer, resource id, a field value,
/// three low-byte-only values, a secondary subject pointer, float bits carried
/// without arithmetic, and a status word.
/// The object word first takes a marker, word +0x04 is mixed with the low 14
/// bits of (old word xor shared counter) and the counter increments, then
/// word +0x00 takes the object vtable. The base initializer (thiscall/0) runs
/// on the object +0x20. Then: s4/s5/s6 low bytes to +0x0a/+0x0b/+0x0c, s3 to
/// +0x390, s7 to +0x3a0, -1 to +0x39c, s0 low word to +0x08. The task helper
/// (cdecl/8) takes s2, the +0x398/+0x39c field addresses, 0, 1, 0, s0 and the
/// float bits, and stores one word to each field address. s1 is stored to
/// +0x10, s1 word +0x58 feeds the per-task helper (thiscall/1 on object
/// +0x20), s9 is stored to +0x14, and the manager at its relocated slot is
/// told (thiscall/1) with s0.
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
    status_word: u32,
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
        write_word(this.wrapping_add(0x14), status_word);

        let manager_context = lf_checker_rt::relocated(MANAGER_CONTEXT_FILE_VA);
        let _ = lf_checker_rt::callee_thiscall!(4, u32, manager_context, task_code);
        this
    }
});
