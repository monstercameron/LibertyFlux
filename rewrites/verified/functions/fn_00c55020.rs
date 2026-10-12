// original: 0x00C55020 initialize_path_bound_record (proposed)
/// Initialize a path-bound record from an owner, a signed sentinel field, and
/// a three-word vector. The base initializer receives `this` in ECX and the
/// owner pointer on the stack. The vector is copied to `+0x20..+0x28`; when
/// the sentinel equals `u32::MAX`, the field at `+0x1c` is replaced by the
/// relocated global and the owner's nested bound values are copied to
/// `+0x20..+0x2c`. The method returns `this` in EAX and removes three stack
/// words. The two nested coordinates are moved bit-for-bit without arithmetic.
lf_checker_rt::export!(thiscall, rw_c55020(
    this: u32, owner: u32, path_id: u32, vector: u32
) -> u32 {
    unsafe {
        const TYPE_TAG_VA: u32 = 0x00ec_ac84;
        const PATH_ID: u32 = 0x1c;
        const GLOBAL_PATH_ID_VA: u32 = 0x0117_35b4;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, owner);
        let type_tag = lf_checker_rt::relocated(TYPE_TAG_VA);
        (this as *mut u32).write_unaligned(type_tag);
        (this.wrapping_add(PATH_ID) as *mut u32).write_unaligned(path_id);

        let vector_words = vector as *const u32;
        (this.wrapping_add(0x20) as *mut u32).write_unaligned(vector_words.read_unaligned());
        (this.wrapping_add(0x24) as *mut u32).write_unaligned(vector_words.add(1).read_unaligned());
        (this.wrapping_add(0x28) as *mut u32).write_unaligned(vector_words.add(2).read_unaligned());

        if path_id == u32::MAX {
            let global_path_id = lf_checker_rt::global::<u32>(GLOBAL_PATH_ID_VA).read_unaligned();
            (this.wrapping_add(PATH_ID) as *mut u32).write_unaligned(global_path_id);
            let nested_bounds = (owner.wrapping_add(0x20) as *const u32).read_unaligned();
            (this.wrapping_add(0x20) as *mut u32)
                .write_unaligned((nested_bounds.wrapping_add(0x30) as *const u32).read_unaligned());
            (this.wrapping_add(0x24) as *mut u32)
                .write_unaligned((nested_bounds.wrapping_add(0x34) as *const u32).read_unaligned());
            (this.wrapping_add(0x28) as *mut u32)
                .write_unaligned((nested_bounds.wrapping_add(0x38) as *const u32).read_unaligned());
            (this.wrapping_add(0x2c) as *mut u32)
                .write_unaligned((nested_bounds.wrapping_add(0x3c) as *const u32).read_unaligned());
        }
        this
    }
});
