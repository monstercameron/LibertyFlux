// original: 0x00BC9860 physics_submit_vector_command

const PHYSICS_MANAGER_SLOT: u32 = 0x012e_22a4;
const PHYSICS_REGISTRY_SLOT: u32 = 0x012b_d0c4;

/// Submit a vector command for one physics object and return the final helper's
/// 32-bit result. The cdecl arguments are a context handle, three `f32` bit
/// patterns, a pointer to a 32-bit result slot, and a word whose low byte is
/// the mode flag. The vector is copied without arithmetic into a contiguous
/// three-word local buffer.
///
/// A resolver receives the context and a local word initialized to `u32::MAX`.
/// The request helper receives that word's address, the vector buffer, the
/// literal mode `1`, the low byte of the supplied mode word, and the context
/// and scratch pointers. Its return becomes the object passed to the follow-up
/// hooks. The next helper's return is stored through `result_out`; the final
/// hook's return is the function result. The two manager pointers are read
/// from relocated singleton slots. All eight helper calls occur in order.
///
/// The checker contract skips only scratch-pointer values that vary with the
/// Rust stack frame and snapshots the pointed-to words at each call. The
/// caller-owned result slot remains fully compared.
lf_checker_rt::export!(
    cdecl,
    rw_00BC9860(
        object_handle: u32,
        x_bits: u32,
        y_bits: u32,
        z_bits: u32,
        result_out: u32,
        mode_word: u32,
    ) -> u32 {
        let mut resolver_value = u32::MAX;
        let mut vector_bits = [x_bits, y_bits, z_bits];
        let resolver_value_ptr = core::ptr::addr_of_mut!(resolver_value) as usize as u32;
        let vector_ptr = vector_bits.as_mut_ptr() as usize as u32;

        let _resolved = lf_checker_rt::callee_cdecl!(1, u32, object_handle, resolver_value_ptr);
        let object = lf_checker_rt::callee_cdecl!(
            2,
            u32,
            resolver_value_ptr,
            vector_ptr,
            1u32,
            mode_word & 0xff,
            object_handle,
            resolver_value_ptr,
        );

        let physics_manager = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<u32>(PHYSICS_MANAGER_SLOT))
        };
        let result_id = lf_checker_rt::callee_thiscall!(3, u32, physics_manager, object);
        unsafe {
            core::ptr::write_unaligned(result_out as *mut u32, result_id);
        }

        let _flush = lf_checker_rt::callee_cdecl!(4, u32,);
        let _record = lf_checker_rt::callee_cdecl!(5, u32, object);
        let _notify = lf_checker_rt::callee_cdecl!(6, u32, object, 0u32);

        let registry = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<u32>(PHYSICS_REGISTRY_SLOT))
        };
        let _refresh = lf_checker_rt::callee_thiscall!(7, u32, registry);
        lf_checker_rt::callee_thiscall!(8, u32, object)
    }
);
