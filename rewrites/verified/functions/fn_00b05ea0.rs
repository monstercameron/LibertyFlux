// original: 0x00B05EA0 accumulate_if_ready

/// If helper 1 returns AL=1, adds the object float at +0x1b0 to the first
/// output float and the object float at +0x1b4 to the second output float,
/// preserving that operand order bit for bit. Helper 1 receives `this` in ECX
/// and a pointer to a one-word local owner context in stack argument 0. The
/// contract skips that pointer value and snapshots the owner word at offset 0.
///
/// The stub cycles AL answers 0 and 1 across 1,000 trials (500 each),
/// exercising both the early return and update paths. Scope: the helper is
/// stubbed; EAX is not compared; the fixture has one owner and two float
/// destinations.
lf_checker_rt::export!(thiscall, rw_00b05ea0(this: u32, first_out: u32, second_out: u32) -> () {
    unsafe {
        let mut local_owner = this;
        let ready = lf_checker_rt::callee_thiscall!(
            1,
            u8,
            this,
            (&mut local_owner as *mut u32) as u32
        );
        if ready != 0 {
            let first_value = ((this as *const u8).add(ACCUMULATOR_FIRST) as *const f32).read_unaligned();
            let first_prior = (first_out as *const f32).read_unaligned();
            (first_out as *mut f32).write_unaligned(add_f32_in_order(first_value, first_prior));

            let second_value = ((this as *const u8).add(ACCUMULATOR_SECOND) as *const f32).read_unaligned();
            let second_prior = (second_out as *const f32).read_unaligned();
            (second_out as *mut f32).write_unaligned(add_f32_in_order(second_value, second_prior));
        }
    }
});

const ACCUMULATOR_FIRST: usize = 0x1b0;
const ACCUMULATOR_SECOND: usize = 0x1b4;

#[inline(always)]
fn add_f32_in_order(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}
