/// Proof scope: eligibility, array movement and virtual release are scripted helpers.
/// Empty/no-match arrays, last-element matches and matches after element zero
/// remain untested. All configured comparisons are enabled.
/// Remove the first matching resource from a pointer array when the direct
/// eligibility call returns zero, then release every non-null resource.
///
/// `this` is an array descriptor: its begin pointer is at byte offset 0 and
/// its one-past-end pointer is at byte offset 4. Each array element is a
/// 32-bit resource pointer. The stack argument is the resource being tested.
/// The eligibility hook receives that resource as `this`; if it returns zero,
/// the function searches the descriptor and removes one matching pointer by
/// calling the array-move helper when trailing elements remain. A match always
/// reduces the end pointer by one element. Finally, a non-null resource is
/// released through virtual slot 1 with the flag value 1. The helper calls
/// retain their original thiscall/cdecl conventions and order.
///
/// The layout is the original 32-bit ABI's pointer-array descriptor.
const ARRAY_BEGIN: u32 = 0;
const ARRAY_END: u32 = 4;
const ELEMENT_BYTES: u32 = 4;
const ELIGIBILITY_HOOK: u32 = 1;
const ARRAY_MOVE_HOOK: u32 = 2;
const RELEASE_VIRTUAL_HOOK: u32 = 3;
const RELEASE_FLAG: u32 = 1;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { core::ptr::read_unaligned(address as *const u32) }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { core::ptr::write_unaligned(address as *mut u32, value) }
}

#[inline(always)]
unsafe fn run(this: u32, resource: u32, end_adjustment: u32) -> u32 {
    let eligibility = lf_checker_rt::callee_thiscall!(
        ELIGIBILITY_HOOK, u32, resource
    );
    if eligibility != 0 {
        return eligibility;
    }

    let mut result = eligibility;
    {
        let begin = unsafe { read_u32(this.wrapping_add(ARRAY_BEGIN)) };
        let end = unsafe { read_u32(this.wrapping_add(ARRAY_END)) };
        let mut element = begin;

        while element != end {
            if unsafe { read_u32(element) } == resource {
                let next = element.wrapping_add(ELEMENT_BYTES);
                let trailing_bytes = end.wrapping_sub(next);
                if trailing_bytes != 0 {
                    let _ = lf_checker_rt::callee_cdecl!(
                        ARRAY_MOVE_HOOK, u32, element, next, trailing_bytes
                    );
                }
                unsafe {
                    write_u32(
                        this.wrapping_add(ARRAY_END),
                        end.wrapping_sub(end_adjustment),
                    );
                }
                break;
            }

            element = element.wrapping_add(ELEMENT_BYTES);
        }
    }

    if resource != 0 {
        result = lf_checker_rt::callee_thiscall!(
            RELEASE_VIRTUAL_HOOK, u32, resource, RELEASE_FLAG
        );
    }
    result
}

lf_checker_rt::export!(thiscall, rw_00da47f0(this: u32, resource: u32) -> u32 {
    unsafe { run(this, resource, ELEMENT_BYTES) }
});
