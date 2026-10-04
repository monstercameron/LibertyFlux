// original: 0x00975d10 audio_occlusion_group_refresh
/// Refresh an audio occlusion group from a sample entity.
///
/// The group (`this`) resolves its two output levels through one of three
/// routes: an early-out when muted or unconfigured, a sample-table route
/// keyed by the entity, or a probe route that samples nearby keys, averages
/// them through two filter stages, and optionally logs the result. A
/// placement branch then either reuses or re-seeds the group position, and a
/// final stage pushes both levels through per-group filters. Returns nothing
/// meaningful (ret:none).
export!(thiscall, rb37_975d10(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        if *(this as *const u8).add(0xa8) != 0
            || *(this as *const u8).add(0xa9) != 0
            || (*(this as *const u32).add(0xc0 / 4) == 0
                && *(this as *const u8).add(0x110) == 0)
        {
            let _: u32 = callee_thiscall!(3, u32, this);
            cookie14();
            return 0;
        }
        let mut slot24 = *(this as *const u32).add(0x18 / 4);
        let fetched: u32 = callee_thiscall!(1, u32, this);
        let mut entity = arg2;
        if entity == 0 && fetched != 0 {
            let vtable = *(fetched as *const u32);
            let slot = *((vtable as *const u8).add(0x18) as *const u32);
            let get: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot as usize);
            entity = get(fetched);
        }
        if entity == 0 {
            *(this as *mut u8).add(0x11c) = 0;
        } else {
            *(this as *mut u8).add(0x11c) = 1;
            let flags = *(entity as *const u32).add(0x24 / 4);
            let b = *(entity as *const i8).add(0x40) as i32;
            let k = *(entity as *const u32).add(0x48 / 4);
            let mut take_table = flags & 0x8000000 != 0 && b >= 0 && b != 0x3f;
            if take_table {
                take_table = (k as i32) >= 0;
            }
            if take_table {
                let gate_ptr = *global::<u32>(0x12fb214);
                let gate: u32 = callee_thiscall!(2, u32, gate_ptr, k);
                take_table = gate != 0;
            }
            if take_table {
                let _: u32 = callee_thiscall!(3, u32, this);
                let mut buf = [0u32; 4];
                let index: u32 = callee_thiscall!(
                    4,
                    u32,
                    relocated(0x1165880),
                    entity,
                    buf.as_mut_ptr() as u32
                );
                let idx = buf[0] as u8 as u32;
                let base = index.wrapping_add(idx.wrapping_mul(0x33));
                slot24 = core::ptr::read_unaligned((base + 0x18) as *const u32);
                *(this as *mut u32).add(0x108 / 4) =
                    core::ptr::read_unaligned((base + 0x14) as *const u32);
                *(this as *mut u32).add(0x10c / 4) =
                    core::ptr::read_unaligned((base + 0x10) as *const u32);
                *(this as *mut u32).add(0x114 / 4) = index;
                *(this as *mut u32).add(0x118 / 4) = idx;
            } else {
                *(this as *mut u32).add(0x114 / 4) = 0;
                *(this as *mut u32).add(0x118 / 4) = 0;
                probe_route(this, arg1, entity, &mut slot24);
            }
            if take_table {
                return tail_stage(this, arg1, slot24);
            }
        }
        if entity == 0 {
            probe_route(this, arg1, 0, &mut slot24);
        }
        tail_stage(this, arg1, slot24)
    }
});
