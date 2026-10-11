// original: 0x00BC9860 physics_submit_vector_command

const PHYSICS_MANAGER_SLOT: u32 = 0x012e_22a4;
const PHYSICS_REGISTRY_SLOT: u32 = 0x012b_d0c4;

/// Submit a three-component vector for a context and return the last hook's
/// 32-bit result. The cdecl inputs are a context handle, three `f32` bit
/// patterns, a pointer to a 32-bit result slot, and a mode word whose low byte
/// is forwarded to the request helper.
///
/// A resolver receives the context and a local `u32` initialized to all ones;
/// it writes the value later passed as the request helper's first argument.
/// The request helper also receives a pointer to a contiguous three-word
/// vector copy, the literal mode `1`, the low mode byte, and the context and
/// scratch pointers. Its result becomes the object for the follow-up hooks.
/// The third helper's result is stored through the caller's result pointer.
/// Two relocated singleton slots supply the manager and registry receivers.
/// All eight calls occur in order. The local scratch pointers are compared by
/// snapshots of their contents because their stack addresses vary by side.
lf_checker_rt::export!(
    cdecl,
    rw_00BC9860(
        context_handle: u32,
        x_bits: u32,
        y_bits: u32,
        z_bits: u32,
        result_out: u32,
        mode_word: u32,
    ) -> u32 {
        let mut resolved_value = u32::MAX;
        let mut vector_words = [x_bits, y_bits, z_bits];
        let resolved_value_ptr = core::ptr::addr_of_mut!(resolved_value) as usize as u32;
        let vector_ptr = vector_words.as_mut_ptr() as usize as u32;

        let _resolved = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            context_handle,
            resolved_value_ptr
        );
        let object = lf_checker_rt::callee_cdecl!(
            2,
            u32,
            resolved_value,
            vector_ptr,
            1u32,
            mode_word & 0xff,
            context_handle,
            resolved_value_ptr,
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

        let physics_registry = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<u32>(PHYSICS_REGISTRY_SLOT))
        };
        let _refresh = lf_checker_rt::callee_thiscall!(7, u32, physics_registry);
        lf_checker_rt::callee_thiscall!(8, u32, object)
    }
);
