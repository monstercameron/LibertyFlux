// original: 0x009fa640 playstat_indexed_report_emit
/// Build and emit a report for the indexed name entry.
///
/// Constructs a temporary report object on the stack, tags it, resolves
/// its payload, looks up the name table slot for the given index (falling
/// back to the default name when the slot is empty), attaches it, emits
/// the report, and tears the object down. Returns nothing meaningful.
export!(cdecl, rw_009fa640(index: u32) -> u32 {
    unsafe {
        // File VA of the report tag object (relocated; see 5D0).
        const TAG: u32 = 0x00e998d8;
        let mut frame = [0u32; 32];
        let base = frame.as_mut_ptr();
        let temp = base.add(8);
        let aux = base.add(21);
        callee_thiscall!(1, u32, temp as u32, 5, 0x2e);
        temp.write(relocated(TAG));
        (aux.add(1) as *mut u8).write(0);
        // Resolve buffer is 16 bytes below the temp object, overlapping it.
        let resolved = callee_cdecl!(2, u32, base.add(4) as u32);
        callee_cdecl!(3, u32, resolved, aux as u32);
        let picked = global::<u32>(0x0103fc88).add(index as usize).read();
        let name = if picked == 0 { relocated(0x00e99408) } else { picked };
        let token = callee_cdecl!(4, u32, name, 0);
        callee_thiscall!(5, u32, temp as u32, token, name);
        callee_cdecl!(6, u32, temp as u32, 0x58);
        callee_thiscall!(7, u32, temp as u32);
        // Stack-cookie check: argument excluded from comparison (see 740).
        callee_thiscall!(8, u32, 0);
        0
    }
});
