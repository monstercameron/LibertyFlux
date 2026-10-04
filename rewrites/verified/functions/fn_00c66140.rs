// original: 0x00c66140 cutscene_tag_status_is_6
/// True (1) when the dispatched tag handler reports status 6.
///
/// Selects the handler from the runtime dispatch table by the tag
/// halfword at [this+0x2e] and calls its vtable slot 3.
export!(thiscall, rw_c66140(this: u32) -> u32 {
    /// File VA of the runtime-filled tag dispatch table.
    const TAG_DISPATCH_TABLE_VA: u32 = 0x0129_5CD8;
    let index = unsafe { ((this as *const u8).byte_add(0x2e) as *const i16).read() };
    let slot_va =
        TAG_DISPATCH_TABLE_VA.wrapping_add((index as i32 as u32).wrapping_mul(4));
    let entry = unsafe { global::<u32>(slot_va).read() };
    let vtable = unsafe { (entry as *const u32).read() };
    let slot = unsafe { (vtable as *const u32).byte_add(0xc).read() };
    let target: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(slot as usize) };
    ((target(entry) & 0xFF) == 6) as u32
});
