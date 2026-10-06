// original: 0x009061e0 input_slot_destroy (proposed)
/// Destroy a slot: clear it, notify, and release its object when owned.
///
/// When `by_handle`'s low byte is set `id` is validated through the lookup
/// callee, otherwise it is used directly as the index; a negative index
/// (signed) or a null slot returns the index with only its low byte
/// cleared. Otherwise the object's word at `+0xc` is cleared, the drop
/// callee runs on the index, and when the thread entry's word at `+8`
/// (found through the fabricated TLS slot named by the static word) is set
/// the object is passed to the release callee. The slot is cleared and,
/// since only the low byte is set, the release answer with its low byte set
/// on the release path, else the thread entry with its low byte set, is
/// returned. Cdecl with two stack words.
export!(cdecl, rw_009061e0(id: u32, by_handle: u32) -> u32 {
    unsafe {
        /// Handle table base (file VA).
        const TABLE: u32 = 0x0118F6F8;
        /// Static word naming the TLS slot (file VA).
        const TLS_SLOT_WORD: u32 = 0x017ABA14;
        const CLEAR_OFF: u32 = 0x0C;
        const OWNED_OFF: u32 = 0x08;
        const LOOKUP_ID: u32 = 1;
        const DROP_ID: u32 = 2;
        const RELEASE_ID: u32 = 3;
        let idx: u32 = if (by_handle as u8) != 0 {
            callee_cdecl!(LOOKUP_ID, u32, id)
        } else {
            id
        };
        // Signed: the original returns early on the sign flag (jns).
        if (idx as i32) < 0 {
            return idx & 0xFFFFFF00;
        }
        let obj = ((relocated(TABLE).wrapping_add(idx.wrapping_mul(4))) as *const u32)
            .read_unaligned();
        if obj == 0 {
            return 0;
        }
        ((obj.wrapping_add(CLEAR_OFF)) as *mut u32).write_unaligned(0);
        let _: u32 = callee_cdecl!(DROP_ID, u32, idx);
        let slot = (global::<u32>(TLS_SLOT_WORD)).read_unaligned() as usize;
        let entry = tls_slot(slot);
        let owned = ((entry.wrapping_add(OWNED_OFF)) as *const u32).read_unaligned();
        if owned != 0 {
            let obj2 = ((relocated(TABLE).wrapping_add(idx.wrapping_mul(4))) as *const u32)
                .read_unaligned();
            let r: u32 = callee_cdecl!(RELEASE_ID, u32, obj2);
            ((relocated(TABLE).wrapping_add(idx.wrapping_mul(4))) as *mut u32)
                .write_unaligned(0);
            return (r & 0xFFFFFF00) | 1;
        }
        ((relocated(TABLE).wrapping_add(idx.wrapping_mul(4))) as *mut u32).write_unaligned(0);
        (entry & 0xFFFFFF00) | 1
    }
});
