// original: 0x00CF2AB0 phMaterialMgrGta::get_material

/// Split a material name, look up its slot, and choose the default on a miss.
///
/// The thiscall wrapper asks the name splitter for a temporary key, copies
/// that NUL-terminated key into a second 100-byte local buffer, and queries
/// the table at `this+8`. A missing slot or a slot containing null dispatches
/// the manager's default-material virtual method at vtable offset `0x24`;
/// a non-null slot returns its stored value. The stack cookie is restored by
/// the original check helper. Buffer pointers are compared through snapshots.
lf_checker_rt::export!(thiscall, rw_fn_00cf2ab0(this: u32, name: u32) -> u32 {
    const SPLIT_NAME: u32 = 1;
    const COPY_NAME: u32 = 2;
    const TABLE_FIND: u32 = 3;
    const DEFAULT_MATERIAL: u32 = 4;
    const COOKIE_CHECK: u32 = 5;
    const TABLE_FIELD: u32 = 8;
    const NAME_BUFFER_WORDS: usize = 16;
    const NAME_BUFFER_BYTES: u32 = 0x64;
    const COOKIE_VA: u32 = 0x0105_7fb4;

    let mut split_output = [0u32; NAME_BUFFER_WORDS];
    let split_address = split_output.as_mut_ptr() as usize as u32;
    let _ = lf_checker_rt::callee_thiscall!(SPLIT_NAME, u32, this, name, split_address);

    let mut name_buffer = [0u32; NAME_BUFFER_WORDS];
    let name_buffer_address = name_buffer.as_mut_ptr() as usize as u32;
    let _ = lf_checker_rt::callee_cdecl!(COPY_NAME, u32, name_buffer_address, split_address, NAME_BUFFER_BYTES);

    let table = this.wrapping_add(TABLE_FIELD);
    let value_slot = lf_checker_rt::callee_thiscall!(TABLE_FIND, u32, table, name_buffer_address);
    let slot_value = if value_slot == 0 {
        0
    } else {
        unsafe { (value_slot as *const u32).read_unaligned() }
    };
    let result = if slot_value == 0 {
        lf_checker_rt::callee_thiscall!(DEFAULT_MATERIAL, u32, this)
    } else {
        slot_value
    };

    let cookie = unsafe { lf_checker_rt::global::<u32>(COOKIE_VA).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, cookie);
    result
});
