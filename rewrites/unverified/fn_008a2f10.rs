// original: 0x008a2f10 audSound_lookup_and_tailcall
/// Table lookup with early exits: forward the sum, or return it as found.
///
/// Takes the object pointer on the stack. When the object's kind byte
/// reads all set, the pointer itself is the result. Otherwise the sum of
/// the scaled kind byte and the selected table word is formed: a zero sum
/// is returned as the table base itself, and a non-zero sum transfers
/// control to the shared successor, returning whatever that call answers.
export!(cdecl, rw_008a2f10(obj: u32) -> u32 {
    unsafe {
        let kind = *(obj.wrapping_add(0x48) as *const u8);
        if kind == 0xFF {
            return obj;
        }
        let row = *(obj.wrapping_add(0x40) as *const u8);
        let scale = *(global::<u32>(0x0115D964) as *const u32);
        let base = *(global::<u32>(0x0115D988) as *const u32);
        let slot = (row as u32).wrapping_mul(0x6F40);
        let word = *((base.wrapping_add(slot).wrapping_add(0x6F10)) as *const u32);
        let sum = scale.wrapping_mul(kind as u32).wrapping_add(word);
        if sum == 0 {
            return base;
        }
        callee_thiscall!(1, u32, sum)
    }
});
