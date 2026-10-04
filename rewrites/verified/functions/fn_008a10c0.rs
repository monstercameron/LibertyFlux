// original: 0x008A10C0 rage::audCrossfadeSound::vf7
/// Start a crossfade sound (vtable slot 7).
///
/// Validates the request, allocates a voice from the bank pool, snapshots
/// the 24-byte voice parameters into it, resolves both sound entries
/// through the bank table, creates the two fade handles, publishes the
/// voice parameters back, and wires the voice from the parameter block:
/// mode byte, three parameter words, two handle words, up to three
/// voice-hook results, then a final attach step. Returns 1 when the
/// voice was fully wired, 0 at any rejection.
///
/// The pool allocator, bank resolver and handle factory are intercepted
/// callees; the three voice-hook calls go through the object's own
/// vtable slot and are intercepted by planted stubs. The parameter block
/// holds unaligned words, read exactly as laid out.
lf_k2_rt::export!(thiscall, rw_008A10C0(this: *mut u8, arg0: u32, arg1: u32, arg2: *mut u8) -> u32 {
    unsafe {
        let ok: u32 =
            lf_k2_rt::callee_thiscall!(1, u32, this as u32, arg0, arg1, arg2 as u32);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        let stride = *lf_k2_rt::global::<u32>(0x115D964);
        let table = *lf_k2_rt::global::<u32>(0x115D988);
        let bank = *this.add(0x40);
        let row = *((table
            .wrapping_add((bank as u32).wrapping_mul(0x6F40))
            .wrapping_add(0x6F10)) as *const u32);
        let voice: u32 = lf_k2_rt::callee_thiscall!(
            2,
            u32,
            lf_k2_rt::relocated(0x115D8A0),
            0x78,
            bank as u32,
            1
        );
        if voice == 0 {
            return 0;
        }
        *this.add(0xB0) = voice.wrapping_sub(row).wrapping_div(stride) as u8;
        lf_k2_rt::callee_thiscall!(3, u32, voice);
        let v = voice as *mut u8;
        core::ptr::copy_nonoverlapping(arg2, v.add(0x28), 24);
        let ebp = *(this.add(0x94) as *const u32);
        let found0: u32 = lf_k2_rt::callee_thiscall!(
            4,
            u32,
            lf_k2_rt::relocated(0x115DC18),
            *(ebp as *const u32),
            this as u32,
            arg1,
            arg2 as u32
        );
        *this.add(0x48) = slot_index_008A10C0(found0, row, stride);
        let h0: u32 = lf_k2_rt::callee_cdecl!(5, u32, lf_k2_rt::relocated(0xE7A6FC), 0);
        *(this.add(0x9C) as *mut u32) = h0;
        core::ptr::copy_nonoverlapping(v.add(0x28), arg2, 24);
        let found1: u32 = lf_k2_rt::callee_thiscall!(
            4,
            u32,
            lf_k2_rt::relocated(0x115DC18),
            *((ebp + 4) as *const u32),
            this as u32,
            arg1,
            arg2 as u32
        );
        *this.add(0x49) = slot_index_008A10C0(found1, row, stride);
        let h1: u32 = lf_k2_rt::callee_cdecl!(5, u32, lf_k2_rt::relocated(0xE7A704), 0);
        *(this.add(0xA0) as *mut u32) = h1;
        let slot = *this.add(0x48);
        if slot == 0xFF {
            return 0;
        }
        if row.wrapping_add((slot as u32).wrapping_mul(stride)) == 0 {
            return 0;
        }
        let h: u32 = lf_k2_rt::callee_thiscall!(6, u32, this as u32, 1);
        if h == 0 {
            return 0;
        }
        let mode = *(ebp as *const u8).add(8);
        *v.add(0x72) = mode;
        core::ptr::copy_nonoverlapping(
            (ebp as *const u8).add(9),
            v.add(0x48),
            12,
        );
        *(v.add(0x64) as *mut u32) = *(ebp as *const u32);
        *(v.add(0x68) as *mut u32) = *((ebp + 4) as *const u32);
        if mode != 0 {
            *this.add(0x3A) |= 4;
        }
        let vtable = *(this as *const u32);
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vtable + 0x10) as *const u32) as usize);
        let g0 = *((ebp + 0x15) as *const u32);
        if g0 != 0 {
            *(v.add(0x54) as *mut u32) = hook(this as u32, g0);
        }
        let g1 = *((ebp + 0x19) as *const u32);
        if g1 != 0 {
            *(v.add(0x58) as *mut u32) = hook(this as u32, g1);
        }
        let g2 = *((ebp + 0x21) as *const u32);
        if g2 != 0 {
            *(v.add(0x60) as *mut u32) = hook(this as u32, g2);
        }
        lf_k2_rt::callee_thiscall!(
            7,
            u32,
            voice,
            *((ebp + 0x25) as *const u32)
        );
        1
    }
});

/// Locate a resolved row address back within its bank row, or 0xFF when
/// the resolution was null.
fn slot_index_008A10C0(found: u32, row: u32, stride: u32) -> u8 {
    if found == 0 {
        0xFF
    } else {
        found.wrapping_sub(row).wrapping_div(stride) as u8
    }
}
