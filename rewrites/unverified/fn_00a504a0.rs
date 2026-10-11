// original: 0x00A504A0 vehicle_dispatch_kind3_b

/// Reads an object's four-bit dispatch kind from bits 6 through 9 at offset
/// 0x28, asks the input helper for three coordinates and a weight, then calls
/// one kind-specific handler for kinds 2, 3, or 4. Each handler receives the
/// object pointer, a pointer to the local coordinate triplet, the weight's
/// f32 bits, and fixed trailing words. The three outgoing handlers are cdecl;
/// the input helper is thiscall and writes the four selected words through its
/// pointer argument. The function returns one in AL for handled kinds and the
/// unhandled kind value otherwise. The 32-bit object layout is inherent.
lf_checker_rt::export!(cdecl, rw_vehicle_dispatch_kind(this: u32, input: u32) -> u8 {
    const KIND_FIELD: u32 = 0x28;
    const KIND_SHIFT: u32 = 6;
    const KIND_MASK: u32 = 0x0F;
    const TYPE_TWO: u32 = 2;
    const TYPE_THREE: u32 = 3;
    const TYPE_FOUR: u32 = 4;
    const OUTPUT_X: usize = 16;
    const OUTPUT_Y: usize = 17;
    const OUTPUT_Z: usize = 18;
    const OUTPUT_WEIGHT: usize = 21;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    let mut helper_output = [0u32; 22];
    let helper_output_pointer = helper_output.as_mut_ptr() as u32;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, input, helper_output_pointer);

    let coordinates = [
        helper_output[OUTPUT_X],
        helper_output[OUTPUT_Y],
        helper_output[OUTPUT_Z],
    ];
    let coordinates_pointer = coordinates.as_ptr() as u32;
    let weight = helper_output[OUTPUT_WEIGHT];
    let object_kind =
        (unsafe { read_u32(this.wrapping_add(KIND_FIELD)) } >> KIND_SHIFT) & KIND_MASK;

    match object_kind {
        TYPE_THREE => {
            let _ = lf_checker_rt::callee_cdecl!(2, u32, this, coordinates_pointer, weight, 0);
            1
        }
        TYPE_TWO => {
            let _ = lf_checker_rt::callee_cdecl!(3, u32, this, coordinates_pointer, weight, 0, 1);
            1
        }
        TYPE_FOUR => {
            let _ = lf_checker_rt::callee_cdecl!(4, u32, this, coordinates_pointer, weight, 0);
            1
        }
        _ => object_kind as u8,
    }
});
