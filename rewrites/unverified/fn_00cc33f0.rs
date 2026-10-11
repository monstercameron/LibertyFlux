// original: 0x00CC33F0 has_sufficient_motion
/// Return one in AL after the entity and receiver flags are set, the mode
/// byte is zero, and at least 3,000 unsigned ticks have elapsed. The receiver's
/// manager supplies a single-precision speed delta; its absolute scripted
/// result must meet the exact speed limit 0x40402038.
lf_checker_rt::export!(thiscall, rw_00cc33f0(this: *mut u8, entity: *const u8) -> u32 {
    use core::arch::x86::{_mm_and_ps,_mm_cvtss_f32,_mm_set_ss,_mm_set1_epi32,_mm_castsi128_ps,_mm_sub_ss};
    const ENTITY_FLAGS_OFFSET: usize = 0x378;
    const PED_FLAGS_OFFSET: usize = 0x50;
    const LAST_UPDATE_OFFSET: usize = 0x7C;
    const MANAGER_OFFSET: usize = 0x24;
    const SPEED_DELTA_OFFSET: usize = 0xAA0;
    const ELAPSED_LIMIT: u32 = 0xBB8;
    const SPEED_LIMIT_BITS: u32 = 0x40402038;
    const WORLD_MODE_BYTE: u32 = 0x017F5EA0;
    const FRAME_COUNTER: u32 = 0x011735B4;
    if unsafe { entity.add(ENTITY_FLAGS_OFFSET).cast::<u32>().read_unaligned() } & (1 << 9) == 0 {
        return 0;
    }
    if lf_checker_rt::callee_thiscall!(1, u32, this as u32) & 0xFF != 0 {
        return 0;
    }
    if unsafe { lf_checker_rt::global::<u8>(WORLD_MODE_BYTE).read_unaligned() } != 0 {
        return 0;
    }
    if unsafe { this.add(PED_FLAGS_OFFSET).cast::<u32>().read_unaligned() } & (1 << 13) == 0 {
        return 0;
    }
    let frame_counter = unsafe { lf_checker_rt::global::<u32>(FRAME_COUNTER).read_unaligned() };
    let last_update = unsafe { this.add(LAST_UPDATE_OFFSET).cast::<u32>().read_unaligned() };
    if frame_counter.wrapping_sub(last_update) < ELAPSED_LIMIT {
        return 0;
    }
    let manager = unsafe { this.add(MANAGER_OFFSET).cast::<u32>().read_unaligned() } as *const u8;
    let current = unsafe { f32::from_bits(manager.add(SPEED_DELTA_OFFSET).cast::<u32>().read_unaligned()) };
    let previous = unsafe { f32::from_bits(manager.add(SPEED_DELTA_OFFSET + 4).cast::<u32>().read_unaligned()) };
    let left = core::hint::black_box(previous);
    let right = core::hint::black_box(current);
    let delta = unsafe { _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(left),_mm_set_ss(right))) };
    let speed = lf_checker_rt::callee_cdecl!(2, f32, delta.to_bits());
    let sign_mask = unsafe { _mm_castsi128_ps(_mm_set1_epi32(0x7fffffff)) };
    let absolute_speed = unsafe { _mm_cvtss_f32(_mm_and_ps(_mm_set_ss(speed),sign_mask)) };
    u32::from(absolute_speed >= f32::from_bits(SPEED_LIMIT_BITS) || absolute_speed.is_nan())
});
