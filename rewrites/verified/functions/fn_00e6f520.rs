// original: 0x00E6F520 timer_hook_dispatch
/// Dispatch the installed timer hook when one is present and enabled.
///
/// Stores the hook table pointer, then calls the handler slot with the
/// table address when the handler object exists, the enable flag is set and
/// the slot is filled. Returns the handler's answer, or the handler pointer
/// (or zero) when no call is made.
export!(cdecl, rw_00e6f520() -> u32 {
    unsafe {
        const TABLE_SLOT: u32 = 0x019F0B28;
        const TABLE_PTR: u32 = 0x00FE375C;
        const OBJ_SLOT: u32 = 0x019F0B34;
        const FLAG: u32 = 0x019F0B2C;
        const HANDLER_WORD: usize = 4;
        *global::<u32>(TABLE_SLOT) = relocated(TABLE_PTR);
        let obj = *global::<u32>(OBJ_SLOT);
        if obj == 0 {
            return 0;
        }
        if *global::<u8>(FLAG) & 1 == 0 {
            return obj;
        }
        let slot = *(obj as *const u32).add(HANDLER_WORD);
        if slot == 0 {
            return 0;
        }
        let handler: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        handler(relocated(TABLE_SLOT))
    }
});
