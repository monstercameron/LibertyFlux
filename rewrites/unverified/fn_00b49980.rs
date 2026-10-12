// original: 0x00B49980 CEventVehicleOnFire::vf8

/// Whether the vehicle-on-fire event affects a ped: the event must name a burning vehicle the ped is not already aboard, group and busy checks must pass, and either the group helper accepts or the ped stands within 25 units squared of the flames.
///
/// `this` points to the event, `ped` to the candidate ped. Outgoing helper
/// calls are intercepted by the contract; all comparisons are unsigned.
/// Returns 1 when the event affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b49980(this: u32, ped: u32) -> u32 {
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
        let vehicle = read_u32(this.wrapping_add(0x18));
        if vehicle == 0 {
            return 0;
        }
        let aboard = read_u32(ped.wrapping_add(0x0B30));
        if aboard != 0 && read_u8(ped.wrapping_add(0x26C)) & 4 != 0 && aboard != vehicle {
            return 0;
        }
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
        let group = lf_checker_rt::callee_thiscall!(1, u32, ped);
        if group == 0 {
            if read_u8(ped.wrapping_add(0x26C)) & 4 != 0 {
                return 1;
            }
        } else {
            let leader = lf_checker_rt::callee_thiscall!(2, u32, group.wrapping_add(8), ped);
            if leader & 0xFF != 0 {
                if read_u8(ped.wrapping_add(0x26C)) & 4 != 0 {
                    return 1;
                }
            } else if read_u8(ped.wrapping_add(0x26C)) & 4 != 0 {
                return 0;
            }
        }
        let anchor = read_u32(vehicle.wrapping_add(0x20));
        let base = read_u32(ped.wrapping_add(0x20)).wrapping_add(0x30);
        let dx = sub_ss(f32::from_bits(read_u32(base)), f32::from_bits(read_u32(anchor.wrapping_add(0x30))));
        let dy = sub_ss(f32::from_bits(read_u32(base.wrapping_add(4))), f32::from_bits(read_u32(anchor.wrapping_add(0x34))));
        let dz = sub_ss(f32::from_bits(read_u32(base.wrapping_add(8))), f32::from_bits(read_u32(anchor.wrapping_add(0x38))));
        let dist2 = add_ss(add_ss(mul_ss(dy, dy), mul_ss(dx, dx)), mul_ss(dz, dz));
        if dist2 > 25.0 { 0 } else { 1 }
    }
});
