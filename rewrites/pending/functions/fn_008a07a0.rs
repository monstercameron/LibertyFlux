// original: 0x008A07A0 rage::audRetriggeredOverlappedSound::vf7
/// Start a retriggered overlapped sound (vtable slot 7).
///
/// Validates the request, snapshots the 24-byte voice parameters, resolves
/// the sound entry through the bank table, caches the timing fields from
/// the parameter block, asks the voice hooks for the two live parameter
/// pointers, and records the loop mode flag. Returns 1 when a live entry
/// was armed, 0 when the request was rejected or no entry resolved.
///
/// The entry index comes from locating the resolved row address back
/// within its bank row; a null resolution selects slot 0xFF (empty), which
/// always fails the final lookup. The two voice-hook calls go through the
/// object's own vtable slot and are intercepted by planted stubs.
lf_k2_rt::export!(thiscall, rw_008A07A0(this: *mut u8, arg0: u32, arg1: u32, arg2: *const u8) -> u32 {
    unsafe {
        let ok: u32 =
            lf_k2_rt::callee_thiscall!(1, u32, this as u32, arg0, arg1, arg2 as u32);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        core::ptr::copy_nonoverlapping(arg2, this.add(0xB0), 24);
        *this.add(0x3A) |= 4;
        let ebp = *(this.add(0x94) as *const u32);
        let stride = *lf_k2_rt::global::<u32>(0x115D964);
        let table = *lf_k2_rt::global::<u32>(0x115D988);
        let bank = *this.add(0x40);
        let row = *((table
            .wrapping_add((bank as u32).wrapping_mul(0x6F40))
            .wrapping_add(0x6F10)) as *const u32);
        let found: u32 = lf_k2_rt::callee_thiscall!(
            2,
            u32,
            lf_k2_rt::relocated(0x115DC18),
            *((ebp + 0xC) as *const u32),
            this as u32,
            arg1,
            arg2 as u32
        );
        let slot = if found == 0 {
            0xFF
        } else {
            found.wrapping_sub(row).wrapping_div(stride) as u8
        };
        *this.add(0x48) = slot;
        *(this.add(0xD0) as *mut i32) = *(ebp as *const i16) as i32;
        *(this.add(0xCC) as *mut u32) = *((ebp as *const u16).add(1)) as u32;
        *(this.add(0xD4) as *mut u32) = *((ebp + 0xC) as *const u32);
        let vtable = *(this as *const u32);
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vtable + 0x10) as *const u32) as usize);
        *(this.add(0xD8) as *mut u32) =
            hook(this as u32, *((ebp + 8) as *const u32));
        *(this.add(0xDC) as *mut u32) =
            hook(this as u32, *((ebp + 4) as *const u32));
        let mode = *(this.add(0x70) as *const u32);
        *this.add(0xE0) = if (mode & 0xC0000) == 0x40000 { 1 } else { 0 };
        if slot == 0xFF {
            return 0;
        }
        if row.wrapping_add((slot as u32).wrapping_mul(stride)) == 0 {
            return 0;
        }
        1
    }
});
