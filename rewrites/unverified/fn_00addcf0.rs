// original: 0x00addcf0 CRenderPhaseSetDefaultRenderState::vf8
/// Default-render-state step: tag a device object and align its state word.
///
/// Asks the device factory (cdecl/2, stubbed) for an object; when one is
/// returned, stamps it with two successive table pointers, mixes a global
/// sequence counter into the state word at object + 4 (bumping the counter),
/// and records a code address at object + 8. Then queries the object's second
/// virtual slot twice — through the real table slot, which the checker
/// redirects to its recorder — and folds the answers into bits 14..20 of the
/// state word with the same align-up-by-16 computation as [`rw_00addc10`]. A
/// null factory answer faults reading address 0, exactly like the original.

export!(thiscall, rw_00addcf0(_this: u32) -> u32 {
    unsafe {
        const FIELD_MASK: u32 = 0x01ffc000;
        let edi = callee_cdecl!(1, u32, 0x0cu32, 0u32);
        if edi != 0 {
            let first = *((edi + 4) as *const u32);
            *(edi as *mut u32) = relocated(0xE7E048);
            let counter = global::<u32>(0x10327A0);
            let mix = (first ^ *counter) & 0x3fff;
            *((edi + 4) as *mut u32) = first ^ mix;
            *counter = (*counter).wrapping_add(1);
            *(edi as *mut u32) = relocated(0xE7E080);
            *((edi + 8) as *mut u32) = relocated(0xD68B20);
        }
        // Virtual call through slot 2 of the object's table. Faults when the
        // object pointer is null, matching the original fault for fault parity.
        let vtable = *(edi as *const u32);
        let slot: u32 = *((vtable + 8) as *const u32);
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let first = query(edi) as i32;
        let phase = 0x10i32.wrapping_sub(first % 16) % 16;
        let second = query(edi);
        let aligned = ((second.wrapping_add(phase as u32) as i32 / 16) as u32) << 14;
        let cell = (edi + 4) as *mut u32;
        let current = *cell;
        let field = (aligned ^ current) & FIELD_MASK;
        *cell = current ^ field;
        field
    }
});
