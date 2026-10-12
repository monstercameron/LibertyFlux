// original: 0x00A04540 NativeImpl_GET_LEVEL_DESIGN_COORDS_FOR_OBJECT
/// Looks up the object for a handle, asks its coordinate helper to fill a
/// stack-local three-word vector, then copies the helper's three returned
/// words to the caller's output pointers. The helper writes zero to all three
/// vector words; its returned pointer is used only to read those words.
lf_checker_rt::export!(cdecl, rw_00A04540(handle: u32, selector: u32, out_x: u32, out_y: u32, out_z: u32) -> () {
    unsafe {
        const OBJECT_POOL: u32 = 0x0163_2C60;
        let pool = lf_checker_rt::global::<u32>(OBJECT_POOL).read_volatile();
        let object = lf_checker_rt::callee_thiscall!(1, u32, pool, handle);

        let mut local_output = core::mem::MaybeUninit::<[u32; 3]>::uninit();
        let output_address = local_output.as_mut_ptr() as *mut u32 as u32;
        let returned_output = lf_checker_rt::callee_thiscall!(2, u32, object, output_address, selector);
        let returned_words = returned_output as *const u32;
        let x_bits = returned_words.read_unaligned();
        let y_bits = returned_words.add(1).read_unaligned();
        let z_bits = returned_words.add(2).read_unaligned();

        (out_x as *mut u32).write_unaligned(x_bits);
        (out_y as *mut u32).write_unaligned(y_bits);
        (out_z as *mut u32).write_unaligned(z_bits);
    }
});
