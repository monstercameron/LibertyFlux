// original: 0x00A87DF0 CRenderPhase::vf4
/// Refresh the phase from its source view (virtual slot 4).
///
/// Copies the source tag, refreshes the two alike members from the source
/// rows, and when the gate check passes and any flag word is set applies the
/// unit-blend preset. Then propagates the source mode bit, and when the
/// commit check passes fetches the sink pair through the table, stores it to
/// the source and hands it to the sink writer. Returns the last answer.
export!(thiscall, rw_a87df0(this: u32, source: u32) -> u32 {
    const ONE: u32 = 0x3F80_0000;
    unsafe {
        let tag = ((source + 0x538) as *const u32).read();
        ((this + 0x10) as *mut u32).write(tag);
    }
    let rows = source.wrapping_add(0x10);
    callee_thiscall!(1, u32, this.wrapping_add(0xB0), rows);
    callee_thiscall!(1, u32, this.wrapping_add(0x4A0), rows);
    let any_flag = unsafe {
        let w0 = ((this + 0x890) as *const u32).read();
        let w1 = ((this + 0x8A0) as *const u32).read();
        let w2 = ((this + 0x8B0) as *const u32).read();
        let w3 = ((this + 0x8C0) as *const u32).read();
        (w0 | w1 | w2 | w3) != 0
    };
    // Indirect calls go through the object's table exactly like the
    // original; the checker plants its stubs in the fabricated table.
    let gate = unsafe {
        let vt = (this as *const u32).read();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((vt + 0x18) as *const u32).read()) as usize);
        f(this)
    };
    if (gate as u8) != 0 && any_flag {
        callee_thiscall!(3, u32, this.wrapping_add(0xB0), 0, 0, ONE, ONE, 0, ONE);
    }
    unsafe {
        let mode = (((source + 0x558) as *const u8).read() >> 2) & 1;
        ((this + 0x1D) as *mut u8).write(mode);
    }
    let commit = unsafe {
        let vt = (this as *const u32).read();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((vt + 0x30) as *const u32).read()) as usize);
        f(this)
    };
    if (commit as u8) == 0 {
        return commit;
    }
    let pair = unsafe {
        let vt = (this as *const u32).read();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((vt + 0x2C) as *const u32).read()) as usize);
        f(this)
    };
    let (w30, w38) = unsafe {
        (
            ((pair + 0x30) as *const u32).read(),
            ((pair + 0x38) as *const u32).read(),
        )
    };
    unsafe {
        ((source + 0x550) as *mut u32).write(w30);
        ((source + 0x548) as *mut u32).write(w38);
    }
    callee_cdecl!(6, u32, w30, w38)
});
