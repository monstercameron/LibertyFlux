// original: 0x0099e3a0 voice_table_lookup
/// Looks up the cached voice value of an entity, acquiring it on a miss.
///
/// Returns zero when the linked object at offset 8 is null, and the cached
/// word at offset 0xe58 when the ready flag at 0xe54 is set. Otherwise, when
/// the low byte of the argument is clear it returns zero; when set, it
/// resolves the handler through a global table indexed by the signed word at
/// offset 0x2e, queries it through callee 1, refines the answer through
/// callee 2 (which fills one word through the argument slot), caches the
/// result and sets the ready flag. The argument slot the rewrite lends to
/// callee 2 is the compiler's spill copy rather than the incoming stack slot,
/// which Rust cannot name; the written value is verified through the cached
/// word, so the contract skips that call argument and the stack check.
export!(thiscall, rw_0099e3a0(this: u32, w: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x1295cd8;
        let inner = *((this.wrapping_add(8)) as *const u32);
        if inner == 0 {
            return 0;
        }
        if *((inner.wrapping_add(0xe54)) as *const u8) != 0 {
            return *((inner.wrapping_add(0xe58)) as *const u32);
        }
        if (w as u8) == 0 {
            return 0;
        }
        let idx = *((inner.wrapping_add(0x2e)) as *const i16) as i32;
        let tab = relocated(TABLE);
        let handler = *((tab.wrapping_add((idx.wrapping_mul(4)) as u32)) as *const u32);
        if handler == 0 {
            return *((inner.wrapping_add(0xe58)) as *const u32);
        }
        let b = callee_cdecl!(1, u32, inner, 0) as u8;
        let mut slot = w;
        let r = callee_thiscall!(2, u32, handler, b as u32, &mut slot as *mut u32 as u32);
        let v = if (r as u8) != 0 {
            slot
        } else {
            *((handler.wrapping_add(0xbc)) as *const u32)
        };
        *((inner.wrapping_add(0xe58)) as *mut u32) = v;
        *((inner.wrapping_add(0xe54)) as *mut u8) = 1;
        v
    }
});
