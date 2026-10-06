// original: 0x0089d610 rage::audOnStopSound::dtor_body (proposed)

/// Destructor body of `rage::audOnStopSound`: releases three slots, then chains.
///
/// Stamps the `audOnStopSound` virtual table (`VTABLE`, relocated like the
/// original's immediate store), then releases each of the three slot bytes at
/// `+0x48`..`+0x4a`: a slot of 0xFF is skipped, as is a null
/// `TABLE[bank*0x6f40+0x6f10]+STRIDE*slot` target (bank at `+0x40`).
/// Otherwise the slot's release runs through its virtual table (slot 0x14,
/// thiscall/1 with 1), intercepted by planting the stub address in the
/// fabricated object, and the slot byte is reset to 0xFF. Control tail-jumps
/// to the base destructor (callee 2, thiscall/0 on `this`). The original
/// redundantly re-tests the slot and recomputes the target mid-loop; those
/// re-tests cannot take (same unchanged values) and are not reproduced.
///
/// Original: 0x0089d610 (thiscall, no stack words; ends in a tail jump).
lf_checker_rt::export!(thiscall, rw_0089d610(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00e7a020;
        const BANK_OFF: u32 = 0x40;
        const SLOTS_OFF: u32 = 0x48;
        const N_SLOTS: u32 = 3;
        const NO_SLOT: u8 = 0xff;
        const TABLE_GLOB: u32 = 0x0115d988;
        const STRIDE_GLOB: u32 = 0x0115d964;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_SLOT: u32 = 0x6f10;
        const VT_SLOT_RELEASE: usize = 0x14;
        const RELEASE: u32 = 1;
        const BASE_DTOR: u32 = 2;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let bank = (this as *const u8).byte_add(BANK_OFF as usize).read() as u32;
        let table = lf_checker_rt::global::<u32>(TABLE_GLOB).read_unaligned();
        let stride = lf_checker_rt::global::<u32>(STRIDE_GLOB).read_unaligned();
        let row = (table.wrapping_add(bank.wrapping_mul(ROW_STRIDE)) as *const u32)
            .byte_add(ROW_SLOT as usize).read_unaligned();
        for i in 0..N_SLOTS {
            let slotp = (this as *mut u8).byte_add((SLOTS_OFF + i) as usize);
            let slot = slotp.read() as u32;
            if slot == NO_SLOT as u32 {
                continue;
            }
            let target = row.wrapping_add(stride.wrapping_mul(slot));
            if target == 0 {
                continue;
            }
            let vt = (target as *const u32).read_unaligned();
            let tgt = ((vt as *const u8).byte_add(VT_SLOT_RELEASE) as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            release(target, 1);
            slotp.write(NO_SLOT);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
