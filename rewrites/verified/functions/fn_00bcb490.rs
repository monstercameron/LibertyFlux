// original: 0x00BCB490 physics_combine_object_vectors

const PHYSICS_MANAGER_SLOT: u32 = 0x012e_22a4;
const OBJECT_MATRIX_POINTER_OFFSET: u32 = 0x20;
const LOCAL_TRANSFORM_CONTEXT_OFFSET: u32 = 0;
const TRANSFORM_OUTPUT_OFFSET: u32 = 0x30;
const OUTPUT_WORD_COUNT: u32 = 4;

/// Resolve two physics handles and, when both objects exist, write four words
/// from a scripted transform helper through the result pointer. The cdecl
/// arguments are two handles, a pointer to a 16-byte output, and a final tag.
///
/// Both resolvers receive the manager loaded from a relocated singleton slot.
/// The transform helper receives the second object's matrix pointer followed
/// by the first object's matrix pointer and a local context in ECX. It writes
/// four words at byte offset `0x30` from that context. The words are copied
/// verbatim to the caller's output. Two later helpers receive the local
/// context and final tag in order. If either resolver returns null, those
/// helpers and the output writes are skipped. The contract scripts both
/// nonnull and null resolver outcomes.
///
/// Stack comparison is disabled because the original uses its local frame as
/// a context. The transform context and the later helper pointers are skipped
/// in the call log and their relevant words are checked through snapshots.
lf_checker_rt::export!(
    cdecl,
    rw_00BCB490(
        first_handle: u32,
        second_handle: u32,
        result_out: u32,
        final_tag: u32,
    ) -> u32 {
        let mut local_words = [0u32; 24];
        let local_ptr = local_words.as_mut_ptr() as usize as u32;
        let physics_manager = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<u32>(PHYSICS_MANAGER_SLOT))
        };
        let first_object = lf_checker_rt::callee_thiscall!(1, u32, physics_manager, first_handle);
        let second_object = lf_checker_rt::callee_thiscall!(2, u32, physics_manager, second_handle);

        if first_object != 0 && second_object != 0 {
            let first_matrix = unsafe {
                core::ptr::read_unaligned((first_object + OBJECT_MATRIX_POINTER_OFFSET) as *const u32)
            };
            let second_matrix = unsafe {
                core::ptr::read_unaligned((second_object + OBJECT_MATRIX_POINTER_OFFSET) as *const u32)
            };
            let transform_context = local_ptr.wrapping_add(4 * 4);
            let _combined = lf_checker_rt::callee_thiscall!(
                3,
                u32,
                transform_context,
                second_matrix,
                first_matrix,
            );

            let output_start = 4 + (TRANSFORM_OUTPUT_OFFSET / 4) as usize;
            for word_index in 0..OUTPUT_WORD_COUNT {
                unsafe {
                    core::ptr::write_unaligned(
                        result_out.wrapping_add(word_index * 4) as *mut u32,
                        local_words[output_start + word_index as usize],
                    );
                }
            }

            let _constructed = lf_checker_rt::callee_thiscall!(
                4,
                u32,
                local_ptr.wrapping_add(LOCAL_TRANSFORM_CONTEXT_OFFSET),
                local_ptr.wrapping_add(16),
            );
            let _finalized = lf_checker_rt::callee_thiscall!(5, u32, local_ptr, final_tag);
        }
        0
    }
);
