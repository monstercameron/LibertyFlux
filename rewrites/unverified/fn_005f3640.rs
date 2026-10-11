// original: 0x005f3640 rage::fragInst::vf40

/// Looks up the fragment key in the runtime bucket table when the incoming
/// trigger byte is set. A match returns false; otherwise the method sets the
/// fragment's update flag, calls the runtime registration callback, optionally
/// checks the current object's category, and invokes the update and cleanup
/// helpers. The table is rooted at the relocated global pointer; heap offsets
/// `+0x94`, `+0x5c`, `+0x6c`, `+0x90`, `+0x60`, and `+0x9c` are read or written.
lf_checker_rt::export!(thiscall, rw_005f3640(this: u32, trigger: u32) -> u32 {
    const RUNTIME_ROOT_VA: u32 = 0x018b_7448;
    const FRAGMENT_KEY: u32 = 0x94;
    const CURRENT_OBJECT: u32 = 0x5c;
    const FALLBACK_OBJECT: u32 = 0x6c;
    const OBJECT_KIND: u32 = 0x90;
    const UPDATE_FLAGS: u32 = 0x60;
    const REGISTRATION_ARGUMENT: u32 = 0x9c;
    const CALLBACK_TABLE: u32 = 0x32c;
    const TABLE_FUNCTION: u32 = 0x0c;
    const TABLE_ROOT: u32 = 0x2f0;
    const TABLE_LINK: u32 = 0x1c;
    const BUCKETS: u32 = 0x08;
    const VTABLE_SLOT: u32 = 0xcc;
    const KIND_MASK: u32 = 0x3000_0000;
    const REQUIRED_KIND: u32 = 0x3000_0000;

    let runtime_root = unsafe { lf_checker_rt::global::<u32>(RUNTIME_ROOT_VA).read_unaligned() };
    let fragment_key = unsafe { (this.wrapping_add(FRAGMENT_KEY) as *const u32).read_unaligned() };
    if trigger as u8 != 0 && fragment_key != 0 {
        let root_record = unsafe { (runtime_root.wrapping_add(TABLE_ROOT) as *const u32).read_unaligned() };
        let hash_table = unsafe { (root_record.wrapping_add(TABLE_LINK) as *const u32).read_unaligned() };
        let mut hash = fragment_key.wrapping_mul(0x1001);
        hash ^= hash >> 22;
        hash = hash.wrapping_add(hash.wrapping_shl(4));
        hash ^= hash >> 9;
        hash = hash.wrapping_mul(0x401);
        let mut mixed = hash ^ (hash >> 2);
        mixed = mixed.wrapping_mul(0x81);
        hash = mixed ^ (mixed >> 12);
        hash = hash.wrapping_add(u32::from(hash == 0));
        let bucket_count_mask = unsafe { (hash_table as *const u32).read_unaligned() }.wrapping_sub(1);
        let bucket_array = unsafe { (hash_table.wrapping_add(BUCKETS) as *const u32).read_unaligned() };
        let bucket = hash & bucket_count_mask;
        let mut entry = unsafe {
            (bucket_array.wrapping_add(bucket.wrapping_mul(4)) as *const u32).read_unaligned()
        };
        while entry != 0 {
            let entry_key = unsafe { (entry as *const u32).read_unaligned() };
            if entry_key == fragment_key {
                return entry & 0xffff_ff00;
            }
            entry = unsafe { (entry.wrapping_add(8) as *const u32).read_unaligned() };
        }
    }

    unsafe {
        let flags = this.wrapping_add(UPDATE_FLAGS) as *mut u32;
        flags.write_unaligned(flags.read_unaligned() | 1);
    }
    let callback_table = runtime_root.wrapping_add(CALLBACK_TABLE);
    let callback_target = unsafe {
        (callback_table.wrapping_add(TABLE_FUNCTION) as *const u32).read_unaligned()
    };
    let registration_argument = unsafe {
        (this.wrapping_add(REGISTRATION_ARGUMENT) as *const u32).read_unaligned()
    };
    // SAFETY: the runtime table's callback slot is intercepted by the contract.
    let register: extern "C" fn(u32, u32) -> u32 = unsafe {
        core::mem::transmute(callback_target as usize)
    };
    let _ = register(callback_table, registration_argument);

    let current_object = unsafe {
        (this.wrapping_add(CURRENT_OBJECT) as *const u32).read_unaligned()
    };
    let resolved_object = if current_object == 0 {
        unsafe { (this.wrapping_add(FALLBACK_OBJECT) as *const u32).read_unaligned() }
    } else {
        let object_flags = unsafe { (current_object.wrapping_add(8) as *const u32).read_unaligned() };
        if object_flags & KIND_MASK == REQUIRED_KIND {
            unsafe { (current_object as *const u32).read_unaligned() }
        } else {
            0
        }
    };
    if resolved_object != 0 {
        let kind = unsafe { (this.wrapping_add(OBJECT_KIND) as *const u32).read_unaligned() };
        if kind != 3 {
            let vtable = unsafe { (this as *const u32).read_unaligned() };
            let target = unsafe { (vtable.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned() };
            // SAFETY: the object vtable target is intercepted when this path is contracted.
            let update: extern "thiscall" fn(u32) -> u32 = unsafe {
                core::mem::transmute(target as usize)
            };
            let _ = update(this);
        }
    }
    if current_object != 0 {
        let cleanup_this = this.wrapping_add(0x50);
        let _ = lf_checker_rt::callee_thiscall!(3, u32, cleanup_this, current_object);
    }
    1
});
