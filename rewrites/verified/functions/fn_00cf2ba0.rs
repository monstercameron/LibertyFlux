// original: 0x00CF2BA0 phMaterialMgrGta::get_default_material

/// Select the manager's built-in default material through two virtual calls.
///
/// The object vtable entry at byte offset `0x18` receives the relocated
/// default-name table pointer and returns a key. Entry `0x14` then receives
/// that key. Both methods are thiscall methods on the same object, each with
/// one stack argument; the second method's EAX result is returned.
lf_checker_rt::export!(thiscall, rw_fn_00cf2ba0(this: u32) -> u32 {
    const NAME_TABLE_VA: u32 = 0x00ed_e964;
    const LOOKUP_SLOT: usize = 0x18;
    const SELECT_SLOT: usize = 0x14;
    let vtable = unsafe { (this as *const u32).read_unaligned() };
    let lookup = unsafe { (vtable as *const u8).add(LOOKUP_SLOT).cast::<u32>().read_unaligned() };
    let select = unsafe { (vtable as *const u8).add(SELECT_SLOT).cast::<u32>().read_unaligned() };
    let lookup_call: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(lookup as usize) };
    let select_call: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(select as usize) };
    let default_key = lookup_call(this, lf_checker_rt::relocated(NAME_TABLE_VA));
    select_call(this, default_key)
});
