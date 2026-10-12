// original: 0x00CBE990 ped_move_blend_update_from_velocity
/// Advance the current blend weight by four single precision global steps,
/// keeping the previous weight in the neighboring object word. Query the
/// manager through the first intercepted helper; when it returns an object,
/// scale its three component values by the image-backed factor times
/// `current - previous`, then invoke the object's method at vtable byte
/// offset `0x70`. The local vector pointer is skipped by the contract and its
/// three words are captured at the call.
/// The method is thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00cbe990(this: *mut u8) -> u32 {
    const CURRENT_WEIGHT: usize = 0x60;
    const PREVIOUS_WEIGHT: usize = 0x64;
    const MANAGER_OFFSET: usize = 0x24;
    const MANAGER_VECTOR: usize = 0x20;
    const TABLE_METHOD_OFFSET: usize = 0x70;
    const GLOBAL_VECTOR_SCALE: u32 = 0x01051490;
    const GLOBAL_STEP_0: u32 = 0x0171BFC8;
    const GLOBAL_STEP_1: u32 = 0x0171BFE8;
    const GLOBAL_STEP_2: u32 = 0x0171C008;
    const GLOBAL_STEP_3: u32 = 0x0171C028;
    const VECTOR_SOURCE: u32 = 0x01B4B320;

    let current_bits = unsafe { this.add(CURRENT_WEIGHT).cast::<u32>().read_unaligned() };
    let previous_bits = unsafe { this.add(PREVIOUS_WEIGHT).cast::<u32>().read_unaligned() };
    let current = f32::from_bits(current_bits);
    let previous = f32::from_bits(previous_bits);
    let sub_ss = |left: f32, right: f32| unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_set_ss, _mm_sub_ss};
        _mm_cvtss_f32(_mm_sub_ss(
            _mm_set_ss(core::hint::black_box(left)),
            _mm_set_ss(core::hint::black_box(right)),
        ))
    };
    let add_ss = |left: f32, right: f32| unsafe {
        use core::arch::x86::{_mm_add_ss, _mm_cvtss_f32, _mm_set_ss};
        _mm_cvtss_f32(_mm_add_ss(
            _mm_set_ss(core::hint::black_box(left)),
            _mm_set_ss(core::hint::black_box(right)),
        ))
    };
    #[inline(never)]
    fn mul_ss_in_order(left: f32, right: f32) -> f32 {
        use core::arch::x86::{_mm_cvtss_f32, _mm_mul_ss, _mm_set_ss};
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        unsafe { _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(left), _mm_set_ss(right))) }
    }
    let delta = sub_ss(current, previous);
    let mut next_weight = f32::from_bits(unsafe {
        lf_checker_rt::global::<u32>(GLOBAL_STEP_0).read_unaligned()
    });
    next_weight = add_ss(next_weight, f32::from_bits(unsafe {
        lf_checker_rt::global::<u32>(GLOBAL_STEP_1).read_unaligned()
    }));
    next_weight = add_ss(next_weight, f32::from_bits(unsafe {
        lf_checker_rt::global::<u32>(GLOBAL_STEP_2).read_unaligned()
    }));
    next_weight = add_ss(next_weight, f32::from_bits(unsafe {
        lf_checker_rt::global::<u32>(GLOBAL_STEP_3).read_unaligned()
    }));
    unsafe {
        this.add(PREVIOUS_WEIGHT)
            .cast::<u32>()
            .write_unaligned(current_bits);
        this.add(CURRENT_WEIGHT)
            .cast::<u32>()
            .write_unaligned(next_weight.to_bits());
    }

    let manager = unsafe { this.add(MANAGER_OFFSET).cast::<u32>().read_unaligned() };
    let returned_object = lf_checker_rt::callee_thiscall!(1, u32, manager);
    if returned_object != 0 {
        let object = returned_object as *mut u8;
        let vtable = unsafe { object.cast::<u32>().read_unaligned() };
        let method_address = unsafe {
            (vtable as *const u32)
                .add(TABLE_METHOD_OFFSET / 4)
                .read_unaligned()
        };
        let manager_vector = unsafe {
            this.add(MANAGER_OFFSET).cast::<u32>().read_unaligned() as *const u8
        };
        let vector = unsafe {
            manager_vector.add(MANAGER_VECTOR).cast::<u32>().read_unaligned() as *const u32
        };
        let x = f32::from_bits(unsafe { vector.read_unaligned() });
        let y = f32::from_bits(unsafe { vector.add(1).read_unaligned() });
        let z = f32::from_bits(unsafe { vector.add(2).read_unaligned() });
        let global_scale = f32::from_bits(unsafe {
            lf_checker_rt::global::<u32>(GLOBAL_VECTOR_SCALE).read_unaligned()
        });
        let vector_scale = mul_ss_in_order(global_scale, delta);
        let scaled = [
            mul_ss_in_order(x, vector_scale).to_bits(),
            mul_ss_in_order(y, vector_scale).to_bits(),
            mul_ss_in_order(z, vector_scale).to_bits(),
        ];
        let pointer = scaled.as_ptr() as u32;
        let method: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(method_address as usize) };
        let _ = method(
            returned_object,
            pointer,
            lf_checker_rt::relocated(VECTOR_SOURCE),
            0,
        );
    }
    0
});
