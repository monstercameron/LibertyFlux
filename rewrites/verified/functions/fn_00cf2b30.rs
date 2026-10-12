// original: 0x00CF2B30 phMaterialMgrGta::get_material_index

/// Copy a name into a local buffer, look up its table slot, and resolve it.
///
/// The thiscall wrapper copies the supplied NUL-terminated name into a
/// 100-byte scratch buffer, asks the table at `this+8` for the value slot,
/// and returns zero on a miss. On a hit, it loads the slot's index and calls
/// the manager's virtual method at vtable offset `0x20`. The stack cookie is
/// restored through the original helper before the one stack argument is
/// removed. The proof snapshots both stack-buffer pointers at their calls;
/// the stack-write comparison is disabled for this derived local buffer.
lf_checker_rt::export!(thiscall, rw_fn_00cf2b30(this: u32, name: u32) -> u32 {
    const NAME_BYTES: u32 = 0x64;
    const NAME_WORDS: usize = 25;
    const COPY_CALLEE: u32 = 1;
    const TABLE_FIND: u32 = 2;
    const RESOLVE_SLOT: u32 = 3;
    const COOKIE_CHECK: u32 = 4;
    const TABLE_FIELD: u32 = 8;
    const SLOT_VALUE: u32 = 0;
    const COOKIE_VA: u32 = 0x0105_7fb4;

    let mut name_buffer = [0u32; NAME_WORDS];
    let buffer_address = name_buffer.as_mut_ptr() as usize as u32;
    let _ = lf_checker_rt::callee_cdecl!(COPY_CALLEE, u32, buffer_address, name, NAME_BYTES);

    let table = this.wrapping_add(TABLE_FIELD);
    let value_slot = lf_checker_rt::callee_thiscall!(TABLE_FIND, u32, table, buffer_address);
    let result = if value_slot == 0 {
        0
    } else {
        let material_index = unsafe {
            (value_slot.wrapping_add(SLOT_VALUE) as *const u32).read_unaligned()
        };
        lf_checker_rt::callee_thiscall!(RESOLVE_SLOT, u32, this, material_index)
    };

    // The original saves (cookie XOR its frame address) and XORs it with the
    // same address before this call. The observable helper argument is the
    // image cookie itself, independent of Rust's local-frame placement.
    let cookie = unsafe { lf_checker_rt::global::<u32>(COOKIE_VA).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, cookie);
    result
});
