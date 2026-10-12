// original: 0x00B491E0 CEventSeenPanickedPed::vf8

/// Whether the seen-panicked-ped event affects a ped: the ped must not be flagged unaware, the event must name another ped whose task and status checks pass, and the squared distance between them must be below 100.
///
/// `this` points to the event, `ped` to the candidate ped. Direct helpers
/// are intercepted by the contract; indirect calls go through fabricated
/// heap objects exactly like the original. Returns 1 when the event
/// affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b491e0(this: u32, ped: u32) -> u32 {
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
        let other = read_u32(this.wrapping_add(0x18));
        if other == 0 || other == ped {
            return 0;
        }
        let tasks = read_u32(other.wrapping_add(0x224));
        let vtable = read_u32(tasks);
        let slot = read_u32(vtable.wrapping_add(0x1C));
        let node_of: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
        let node = node_of(tasks);
        let status = lf_checker_rt::callee_thiscall!(2, u32, node.wrapping_add(0x20));
        if status == 0 {
            return 0;
        }
        let vtable = read_u32(status);
        let slot = read_u32(vtable.wrapping_add(0x34));
        let flag_of: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
        if flag_of(status) == 0 {
            return 0;
        }
        let anchor = read_u32(other.wrapping_add(0x20));
        let base = read_u32(ped.wrapping_add(0x20));
        let dx = sub_ss(f32::from_bits(read_u32(base.wrapping_add(0x30))), f32::from_bits(read_u32(anchor.wrapping_add(0x30))));
        let dy = sub_ss(f32::from_bits(read_u32(base.wrapping_add(0x34))), f32::from_bits(read_u32(anchor.wrapping_add(0x34))));
        let dz = sub_ss(f32::from_bits(read_u32(base.wrapping_add(0x38))), f32::from_bits(read_u32(anchor.wrapping_add(0x38))));
        let dist2 = add_ss(add_ss(mul_ss(dy, dy), mul_ss(dx, dx)), mul_ss(dz, dz));
        let limit = f32::from_bits(lf_checker_rt::global::<u32>(0x00FE8BB0).read_unaligned());
        (limit > dist2) as u32
    }
});
