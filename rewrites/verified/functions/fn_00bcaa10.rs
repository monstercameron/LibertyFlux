// original: 0x00BCAA10 physics_copy_and_transform_vector

const PHYSICS_MANAGER_SLOT: u32 = 0x012e_22a4;
const OBJECT_VTABLE_OFFSET: u32 = 0;
const VECTOR_GETTER_SLOT_OFFSET: u32 = 0xec;
const OBJECT_MATRIX_POINTER_OFFSET: u32 = 0x20;
const VECTOR_X_OFFSET: u32 = 0;
const VECTOR_Y_OFFSET: u32 = 4;
const VECTOR_Z_OFFSET: u32 = 8;
const VECTOR_WORDS: u32 = 4;
const MATRIX_XY_OFFSET: u32 = 4;
const MATRIX_XX_OFFSET: u32 = 0;
const MATRIX_XZ_OFFSET: u32 = 8;
const MATRIX_YX_OFFSET: u32 = 0x10;
const MATRIX_YY_OFFSET: u32 = 0x14;
const MATRIX_YZ_OFFSET: u32 = 0x18;
const MATRIX_ZX_OFFSET: u32 = 0x20;
const MATRIX_ZY_OFFSET: u32 = 0x24;
const MATRIX_ZZ_OFFSET: u32 = 0x28;

/// Resolve a physics object and copy four words from its vector method into
/// the caller's 16-byte output. The cdecl inputs are an object handle, an
/// output pointer, and a transform flag read from its low byte.
///
/// A zero object skips the vector method and leaves the output untouched. A
/// nonzero object supplies a function table at offset zero; the method at
/// byte offset `0xec` returns a pointer to four words. When the transform flag
/// is nonzero, the first three output words are treated as floats and
/// multiplied by the object's 3-by-3 matrix at offset `0x20`, whose rows are
/// stored with a 16-byte stride. Each result preserves the original product
/// and sum order. A null object with a nonzero flag faults while reading its
/// matrix, as the original does.
///
/// The resolver's return and the method's returned vector are scripted by
/// the contract. The method's frame-local output pointer is skipped in the
/// call log; its contents are snapshotted with zero stack fill. The contract
/// uses nonnull resolver results, so the null-object paths are not exercised.
/// It does exercise nonzero objects with both transform states.
lf_checker_rt::export!(
    cdecl,
    rw_00BCAA10(object_handle: u32, result_out: u32, transform_flag: u32) -> u32 {
        #[inline(never)]
        fn multiply(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(never)]
        fn add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        let physics_manager = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<u32>(PHYSICS_MANAGER_SLOT))
        };
        let object = lf_checker_rt::callee_thiscall!(1, u32, physics_manager, object_handle);
        if object != 0 {
            let vtable = unsafe {
                core::ptr::read_unaligned((object + OBJECT_VTABLE_OFFSET) as *const u32)
            };
            let getter_address = unsafe {
                core::ptr::read_unaligned((vtable + VECTOR_GETTER_SLOT_OFFSET) as *const u32)
            };
            let mut unused_output = 0u32;
            let unused_output_ptr = core::ptr::addr_of_mut!(unused_output) as usize as u32;
            let getter: unsafe extern "thiscall" fn(u32, u32) -> u32 = unsafe {
                core::mem::transmute(getter_address as usize)
            };
            let vector = unsafe { getter(object, unused_output_ptr) };
            for word_index in 0..VECTOR_WORDS {
                let word = unsafe {
                    core::ptr::read_unaligned((vector + word_index * 4) as *const u32)
                };
                unsafe {
                    core::ptr::write_unaligned((result_out + word_index * 4) as *mut u32, word);
                }
            }
        }

        if transform_flag & 0xff != 0 {
            let matrix = unsafe {
                core::ptr::read_unaligned((object + OBJECT_MATRIX_POINTER_OFFSET) as *const u32)
            };
            let x = f32::from_bits(unsafe {
                core::ptr::read_unaligned((result_out + VECTOR_X_OFFSET) as *const u32)
            });
            let y = f32::from_bits(unsafe {
                core::ptr::read_unaligned((result_out + VECTOR_Y_OFFSET) as *const u32)
            });
            let z = f32::from_bits(unsafe {
                core::ptr::read_unaligned((result_out + VECTOR_Z_OFFSET) as *const u32)
            });
            let m_xy = f32::from_bits(unsafe {
                core::ptr::read_unaligned((matrix + MATRIX_XY_OFFSET) as *const u32)
            });
            let m_xx = f32::from_bits(unsafe {
                core::ptr::read_unaligned((matrix + MATRIX_XX_OFFSET) as *const u32)
            });
            let m_xz = f32::from_bits(unsafe {
                core::ptr::read_unaligned((matrix + MATRIX_XZ_OFFSET) as *const u32)
            });
            let m_yy = f32::from_bits(unsafe {
                core::ptr::read_unaligned((matrix + MATRIX_YY_OFFSET) as *const u32)
            });
            let m_yx = f32::from_bits(unsafe {
                core::ptr::read_unaligned((matrix + MATRIX_YX_OFFSET) as *const u32)
            });
            let m_yz = f32::from_bits(unsafe {
                core::ptr::read_unaligned((matrix + MATRIX_YZ_OFFSET) as *const u32)
            });
            let m_zy = f32::from_bits(unsafe {
                core::ptr::read_unaligned((matrix + MATRIX_ZY_OFFSET) as *const u32)
            });
            let m_zx = f32::from_bits(unsafe {
                core::ptr::read_unaligned((matrix + MATRIX_ZX_OFFSET) as *const u32)
            });
            let m_zz = f32::from_bits(unsafe {
                core::ptr::read_unaligned((matrix + MATRIX_ZZ_OFFSET) as *const u32)
            });

            let x_out = add(add(multiply(m_xy, y), multiply(m_xx, x)), multiply(m_xz, z));
            let y_out = add(add(multiply(m_yy, y), multiply(m_yx, x)), multiply(m_yz, z));
            let z_out = add(add(multiply(m_zy, y), multiply(m_zx, x)), multiply(m_zz, z));
            unsafe {
                core::ptr::write_unaligned((result_out + VECTOR_X_OFFSET) as *mut u32, x_out.to_bits());
                core::ptr::write_unaligned((result_out + VECTOR_Y_OFFSET) as *mut u32, y_out.to_bits());
                core::ptr::write_unaligned((result_out + VECTOR_Z_OFFSET) as *mut u32, z_out.to_bits());
            }
        }
        result_out
    }
);
