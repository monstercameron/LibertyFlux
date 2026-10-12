// original: 0x00B49340 CEventSoundDynamic::vf8

/// Whether the dynamic-sound event affects a ped: after the usual awareness, seat and task checks the event's own loudness flag must be set, the heard level must reach 30, and the ped's current task must be none of the three excluded kinds.
///
/// `this` points to the event, `ped` to the candidate ped. Direct helpers
/// are intercepted by the contract; indirect calls go through fabricated
/// heap objects exactly like the original. Returns 1 when the event
/// affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b49340(this: u32, ped: u32) -> u32 {
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    #[inline(always)]
    fn mul_ss(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    #[inline(always)]
    fn add_ss(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    #[inline(always)]
    fn sub_ss(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) - core::hint::black_box(b)
    }

    unsafe {
        if read_u8(ped.wrapping_add(0x219)) != 0 {
            return 0;
        }
        if read_u8(ped.wrapping_add(0x210)) != 0 {
            let seat = read_u32(ped.wrapping_add(0x0A74));
            if seat == 1 || seat == 2 {
                return 0;
            }
        }
        if read_u32(ped.wrapping_add(0x224)) == 0 {
            return 0;
        }
        let vtable = read_u32(this);
        let slot = read_u32(vtable.wrapping_add(0x34));
        let loud_of: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
        if loud_of(this) == 0 {
            return 0;
        }
        let ear = read_u32(ped.wrapping_add(0x20)).wrapping_add(0x30);
        let level: f32 = lf_checker_rt::callee_thiscall!(2, f32, this, 0, ear);
        let limit = f32::from_bits(lf_checker_rt::global::<u32>(0x00FE8B48).read_unaligned());
        if limit > level {
            return 0;
        }
        let tasks = read_u32(ped.wrapping_add(0x224));
        let vtable = read_u32(tasks);
        let slot = read_u32(vtable.wrapping_add(0x1C));
        let node_of: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
        let node = node_of(tasks);
        let current = lf_checker_rt::callee_thiscall!(4, u32, node.wrapping_add(0x20));
        if current == 0 {
            return 1;
        }
        let vtable = read_u32(current);
        let slot = read_u32(vtable.wrapping_add(0x04));
        let kind_of: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
        if kind_of(current) == 0x61 {
            return 0;
        }
        if kind_of(current) == 0x3E {
            return 0;
        }
        (kind_of(current) != 0x3D) as u32
    }
});
