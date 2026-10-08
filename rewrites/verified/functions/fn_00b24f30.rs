//! Proof scope: complete motion-query body under valid pointer and scripted collaborator fixtures.
//! Output float bits match; native helper bodies, arbitrary objects and untested branches are outside scope.
//! Full-data checking is disabled. One virtual-call pointer identity and caller-frame scratch area are excluded.
//! EFLAGS, x87 and other floating status state are not claimed. Call shapes are not internal branch instrumentation.
//! Only the mutant export is removed; positive implementation and panic handler remain unchanged, projection unbuilt.
//! Earlier reconstructed build evidence is excluded; final immutable source, DLL and stock run receipts are used.
#![no_std]

use lf_checker_rt::{callee_thiscall, export};

/// Mirrors the motion-target probe. It clears the caller's output, builds a
/// three-float local query (optionally offset by the object's current basis),
/// then checks the target twice and calls its virtual motion method only when
/// both checks return a non-null object. The virtual method receives the local
/// query, the output buffer, and the caller's selector.
#[inline(never)]
unsafe fn implementation<const MUTANT: bool>(
    this_ptr: u32,
    output_ptr: u32,
    input_ptr: u32,
    mode: u32,
    selector: u32,
) -> u32 {
    let output = output_ptr as *mut f32;
    output.write(0.0);
    output.add(1).write(0.0);
    output.add(2).write(0.0);

    let input = input_ptr as *const f32;
    let mut query = [input.read(), input.add(1).read(), input.add(2).read()];

    if mode as u8 == 0 {
        let object = this_ptr as *const u8;
        let basis_owner = object.add(0x20).cast::<u32>().read();
        let basis = if basis_owner != 0 {
            (basis_owner.wrapping_add(0x30)) as *const f32
        } else {
            object.add(0x10).cast::<f32>()
        };
        for axis in 0..3 {
            let base_component = core::hint::black_box(basis.add(axis).read());
            let input_component = core::hint::black_box(query[axis]);
            query[axis] = if MUTANT {
                base_component - input_component
            } else {
                base_component + input_component
            };
        }
    }

    let first_check = callee_thiscall!(1, u32, this_ptr);
    if first_check != 0 {
        let motion_object = callee_thiscall!(2, u32, this_ptr);
        let vtable = *(motion_object as *const u32) as *const u32;
        let method_addr = *vtable.add(0x58 / 4);
        let method: unsafe extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(method_addr as usize);
        let _ = method(motion_object, query.as_mut_ptr() as u32, output_ptr, selector);
    }

    output_ptr
}

export!(thiscall, rw_00b24f30(this_ptr: u32, output_ptr: u32, input_ptr: u32, mode: u32, selector: u32) -> u32 {
    unsafe { implementation::<false>(this_ptr, output_ptr, input_ptr, mode, selector) }
});

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    loop { core::hint::spin_loop(); }
}
