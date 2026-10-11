// original: 0x00BCA9B0 physics_vector_magnitude

const PHYSICS_MANAGER_SLOT: u32 = 0x012e_22a4;
const OBJECT_VTABLE_OFFSET: u32 = 0;
const VECTOR_GETTER_SLOT_OFFSET: u32 = 0xec;
const VECTOR_X_OFFSET: u32 = 0;
const VECTOR_Y_OFFSET: u32 = 4;
const VECTOR_Z_OFFSET: u32 = 8;

/// Resolve a physics object, obtain its vector through a vtable method, and
/// write the vector magnitude through the second cdecl argument. The first
/// argument is a context handle and the second is a pointer to a 32-bit
/// result slot.
///
/// The manager receiver is loaded from a relocated singleton slot. Its
/// resolver returns an object whose first word points to a function table;
/// the method at byte offset `0xec` returns a pointer to three consecutive
/// `f32` values. The function squares x, y, then z, adds x²+y² before z², and
/// takes the square root of that sum. The local method out-pointer is passed
/// as an argument, but its contents are not consumed by this routine. Its
/// address is skipped in call comparison and the pointed-to word is
/// snapshotted with a defined zero stack fill. The result pointer is also
/// returned in EAX.
lf_checker_rt::export!(
    cdecl,
    rw_00BCA9B0(object_handle: u32, result_out: u32) -> u32 {
        let physics_manager = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<u32>(PHYSICS_MANAGER_SLOT))
        };
        let object = lf_checker_rt::callee_thiscall!(1, u32, physics_manager, object_handle);
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

        let x = f32::from_bits(unsafe {
            core::ptr::read_unaligned((vector + VECTOR_X_OFFSET) as *const u32)
        });
        let y = f32::from_bits(unsafe {
            core::ptr::read_unaligned((vector + VECTOR_Y_OFFSET) as *const u32)
        });
        let z = f32::from_bits(unsafe {
            core::ptr::read_unaligned((vector + VECTOR_Z_OFFSET) as *const u32)
        });
        let x_squared = core::hint::black_box(x) * core::hint::black_box(x);
        let y_squared = core::hint::black_box(y) * core::hint::black_box(y);
        let z_squared = core::hint::black_box(z) * core::hint::black_box(z);
        let xy_squared = core::hint::black_box(x_squared) + core::hint::black_box(y_squared);
        let squared_magnitude = core::hint::black_box(xy_squared) + core::hint::black_box(z_squared);
        let magnitude_bits = squared_magnitude.sqrt().to_bits();

        unsafe {
            core::ptr::write_unaligned(result_out as *mut u32, magnitude_bits);
        }
        result_out
    }
);
