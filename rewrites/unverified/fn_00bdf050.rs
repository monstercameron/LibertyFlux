// original: 0x00BDF050 CTaskComplexUseEffect::CTaskComplexUseEffect

/// Initialize the effect task's identifier, optional effect reference and counters. A reference helper is called only when the effect pointer is non-null; the task object is returned.
lf_checker_rt::export!(thiscall, rw_00bdf050(this: u32, effect_id: u32, effect: u32) -> u32 {
unsafe {
    let task = this as *mut u8;
    let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this);
    task.add(0x14).cast::<u32>().write(effect_id);
    let effect_ref = task.add(0x18).cast::<u32>();
    effect_ref.write(effect);
    task.cast::<u32>().write(lf_checker_rt::relocated(0x00eb8ee4));
    task.add(0x1c).cast::<u32>().write(0);
    task.add(0x20).cast::<u32>().write(0);
    task.add(0x24).cast::<u32>().write(2);
    task.add(0x28).write(0);
    if effect != 0 {
        let _reference_result = lf_checker_rt::callee_thiscall!(2, u32, effect, effect_ref as u32);
    }
    this
}
});
