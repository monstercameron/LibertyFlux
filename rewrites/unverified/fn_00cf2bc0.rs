// original: 0x00CF2BC0 phMaterialMgrGta::get_material_by_id

/// Dispatch a byte-sized material identifier through virtual slot `0x34`.
///
/// The input stack word is truncated to its low byte, zero-extended, and
/// written back before the tail dispatch. The selected thiscall method
/// receives that normalized word and its return value is returned unchanged.
lf_checker_rt::export!(thiscall, rw_fn_00cf2bc0(this: u32, material_id: u32) -> u32 {
    const MATERIAL_LOOKUP_SLOT: usize = 0x34;
    let vtable = unsafe { (this as *const u32).read_unaligned() };
    let lookup = unsafe { (vtable as *const u8).add(MATERIAL_LOOKUP_SLOT).cast::<u32>().read_unaligned() };
    let call: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(lookup as usize) };
    call(this, material_id as u8 as u32)
});
