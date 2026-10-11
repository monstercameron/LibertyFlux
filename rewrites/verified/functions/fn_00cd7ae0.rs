// original: 0x00CD7AE0 CEntitySeekPosCalculatorXYOffset::vf1

/// Add the object's base XYZ coordinates to either the fallback offset in a
/// position descriptor or the offset vector selected by its optional
/// pointer, then pass the resulting four-word vector to the copy helper. The
/// first three words are single-precision additions in the operand order
/// shown here; the fourth word is zero under the contract's defined stack
/// fill. The helper's return value is forwarded unchanged.
///
/// Calling convention: thiscall with three 32-bit stack arguments: a source
/// handle, a descriptor pointer, and a destination pointer. The helper is
/// cdecl and receives those two handles around a pointer to the temporary
/// four-word vector.
lf_checker_rt::export!(thiscall, rw_00cd7ae0(this: u32, source_handle: u32, descriptor: u32, destination: u32) -> u32 {
    const BASE_X: u32 = 0x10;
    const BASE_Y: u32 = 0x14;
    const BASE_Z: u32 = 0x18;
    const OPTIONAL_VECTOR: u32 = 0x20;
    const VECTOR_X: u32 = 0x30;
    const FALLBACK_VECTOR: u32 = 0x10;
    const COPY_VECTOR: u32 = 1;

    unsafe {
        let selected = ((descriptor + OPTIONAL_VECTOR) as *const u32).read_unaligned();
        let vector = if selected == 0 {
            descriptor + FALLBACK_VECTOR
        } else {
            selected + VECTOR_X
        };

        let ordered_add_bits = |left_bits: u32, right_bits: u32| {
            let left = core::hint::black_box(f32::from_bits(left_bits));
            let right = core::hint::black_box(f32::from_bits(right_bits));
            core::hint::black_box(left + right).to_bits()
        };

        let base_x = ((this + BASE_X) as *const u32).read_unaligned();
        let base_y = ((this + BASE_Y) as *const u32).read_unaligned();
        let base_z = ((this + BASE_Z) as *const u32).read_unaligned();
        let vector_x = (vector as *const u32).read_unaligned();
        let vector_y = ((vector + 4) as *const u32).read_unaligned();
        let vector_z = ((vector + 8) as *const u32).read_unaligned();

        let mut coordinates = [
            ordered_add_bits(vector_x, base_x),
            ordered_add_bits(base_y, vector_y),
            ordered_add_bits(base_z, vector_z),
            0,
        ];
        lf_checker_rt::callee_cdecl!(
            COPY_VECTOR,
            u32,
            source_handle,
            coordinates.as_mut_ptr() as usize as u32,
            destination
        )
    }
});
