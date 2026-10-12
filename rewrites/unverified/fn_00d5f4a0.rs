// original: 0x00D5F4A0 phMaterialGta::phMaterialGta

/// Construct a GTA physics material with its default coefficients and flags.
///
/// The direct base-material constructor runs first. This object then receives
/// its relocated vtable pointer, an enabled flag at byte offset 4, and zeroed
/// words at offsets `0x20`, `0x30`, `0x34`, and `0x38`. Its three friction and
/// restitution values are stored as `f32` bit patterns at offsets `0x14`,
/// `0x18`, `0x1c`, `0x24`, `0x28`, and `0x2c`. The zero-argument thiscall
/// constructor returns `this` and has no stack arguments.
lf_checker_rt::export!(thiscall, rw_ph_material_gta_ctor(this_ptr: u32) -> u32 {
    const BASE_MATERIAL_CTOR: u32 = 1;
    const MATERIAL_VTABLE_VA: u32 = 0x00EE8D7C;
    const ONE_BITS: u32 = 0x3F800000;
    const HALF_BITS: u32 = 0x3F000000;
    const TEN_BITS: u32 = 0x41200000;
    const NEGATIVE_QUARTER_BITS: u32 = 0xBE800000;

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    let _ = lf_checker_rt::callee_thiscall!(BASE_MATERIAL_CTOR, u32, this_ptr);
    unsafe {
        write_u32(this_ptr, lf_checker_rt::relocated(MATERIAL_VTABLE_VA));
        write_u32(this_ptr.wrapping_add(4), 1);
        write_u32(this_ptr.wrapping_add(0x14), ONE_BITS);
        write_u32(this_ptr.wrapping_add(0x18), ONE_BITS);
        write_u32(this_ptr.wrapping_add(0x1C), NEGATIVE_QUARTER_BITS);
        write_u32(this_ptr.wrapping_add(0x24), HALF_BITS);
        write_u32(this_ptr.wrapping_add(0x28), TEN_BITS);
        write_u32(this_ptr.wrapping_add(0x2C), HALF_BITS);
        write_u32(this_ptr.wrapping_add(0x20), 0);
        write_u32(this_ptr.wrapping_add(0x30), 0);
        write_u32(this_ptr.wrapping_add(0x34), 0);
        write_u32(this_ptr.wrapping_add(0x38), 0);
    }
    this_ptr
});
