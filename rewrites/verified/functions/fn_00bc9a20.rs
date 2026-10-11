// original: 0x00BC9A20 physics_configure_vector

const PHYSICS_MANAGER_SLOT: u32 = 0x012e_22a4;

/// Configure a physics vector and write the manager result to the caller's
/// output slot. The cdecl arguments are a context handle, three `f32` bit
/// patterns, an input tag, and a pointer to a 32-bit result slot. The vector
/// words are copied into a contiguous local buffer without arithmetic.
///
/// The configuration helper receives the vector pointer, tag, context,
/// pointer to a local result word, then the constants `0`, `u32::MAX`,
/// `u32::MAX`, and `1`. It writes that local result word. The manager helper
/// receives the local result and its receiver comes from a relocated global
/// slot. Its return is both stored through the caller's pointer and returned.
/// The vector pointer is skipped in the call log and its three words are
/// compared through a call-time snapshot. The local result pointer address is
/// skipped because it varies by side; its initial contents are unobserved and
/// the helper overwrites it before the value is sent to the manager helper.
/// That resulting value is compared as the next call's argument.
lf_checker_rt::export!(
    cdecl,
    rw_00BC9A20(
        context_handle: u32,
        x_bits: u32,
        y_bits: u32,
        z_bits: u32,
        input_tag: u32,
        result_out: u32,
    ) -> u32 {
        let mut helper_value = 0u32;
        let mut vector_words = [x_bits, y_bits, z_bits];
        let helper_value_ptr = core::ptr::addr_of_mut!(helper_value) as usize as u32;
        let vector_ptr = vector_words.as_mut_ptr() as usize as u32;

        let _configured = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            vector_ptr,
            input_tag,
            context_handle,
            helper_value_ptr,
            0u32,
            u32::MAX,
            u32::MAX,
            1u32,
        );
        let physics_manager = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<u32>(PHYSICS_MANAGER_SLOT))
        };
        let result = lf_checker_rt::callee_thiscall!(2, u32, physics_manager, helper_value);
        unsafe {
            core::ptr::write_unaligned(result_out as *mut u32, result);
        }
        result
    }
);
