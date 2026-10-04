// original: 0x008a6aa0 aud_entity_predispatch
/// Run a pre-dispatch hook, then dispatch through the entity table.
///
/// Calls a hook with (object, arg1), then resolves a target from the audio
/// entity table for the object's index/count bytes (+0x40, +0x48), returning
/// 0 when the count is 0xff, or the table base with a cleared low byte when
/// the total is 0 (the original zeroes only AL there). Otherwise dispatches with the
/// second stack argument and returns the result. Note: the dispatch call
/// inherits whatever ECX the hook left behind, so the contract declares it
/// stdcall (no register compared); the rewrite passes only the stack word.
export!(cdecl, rw_008a6aa0(obj: u32, arg1: u32) -> u32 {
    unsafe {
        let this = obj as *mut u8;
        let _: u32 = callee_thiscall!(1, u32, obj, arg1);
        let count = *this.add(0x48);
        if count == 0xff {
            return 0;
        }
        let index = *this.add(0x40) as u32;
        let stride = *global::<u32>(0x115d964);
        let table = *global::<u32>(0x115d988);
        let slot = table
            .wrapping_add(index.wrapping_mul(0x6f40))
            .wrapping_add(0x6f10) as *const u32;
        let target = (*slot).wrapping_add(stride.wrapping_mul(count as u32));
        if target == 0 {
            // The original zeroes only AL here, leaving the table base it
            // just loaded in the upper bytes; that value is input-determined.
            return table & 0xffffff00;
        }
        callee_stdcall!(2, u32, arg1)
    }
});
