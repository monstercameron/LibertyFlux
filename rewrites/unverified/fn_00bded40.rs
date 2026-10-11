// original: 0x00BDED40 CTaskComplexReact::CTaskComplexReact

/// Initialize a reaction task from scalar fields and an optional referenced entity. When the entity is present, its nested data block is preferred, with a block at entity offset `0x10` as the fallback; when it is absent, the explicit fallback block is used. Four source words are copied bit-for-bit when a source exists. The reference helper is called only for a non-null entity.
lf_checker_rt::export!(thiscall, rw_00bded40(this: u32, arg0: u32, arg1: u32, arg2: u32, subject: u32, fallback: u32, arg5: u32) -> u32 {
unsafe {
    let task = this as *mut u8;
    let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this);
    task.cast::<u32>().write(lf_checker_rt::relocated(0x00eb94ac));
    task.add(0x20).cast::<u32>().write(0);
    task.add(0x24).cast::<u32>().write(0);
    task.add(0x28).cast::<u32>().write(0);
    task.add(0x30).cast::<u32>().write(arg2);
    task.add(0x34).cast::<u32>().write(arg0);
    task.add(0x38).cast::<u32>().write(0);
    let reference_slot = task.add(0x3c).cast::<u32>();
    reference_slot.write(0);
    task.add(0x40).cast::<u32>().write(0);
    task.add(0x44).cast::<u16>().write(0);
    task.add(0x46).write(arg1 as u8);
    task.add(0x47).write(u8::from(arg2 != u32::MAX));
    task.add(0x48).write(arg5 as u8);

    let source = if subject != 0 {
        reference_slot.write(subject);
        let _reference_result = lf_checker_rt::callee_thiscall!(2, u32, subject, reference_slot as u32);
        let nested = (subject as *const u32).add(8).read();
        if nested != 0 { nested.wrapping_add(0x30) } else { subject.wrapping_add(0x10) }
    } else {
        fallback
    };
    if source != 0 {
        let source_words = source as *const u32;
        let destination = task.add(0x20).cast::<u32>();
        for index in 0..4 {
            destination.add(index).write(source_words.add(index).read());
        }
    }
    this
}
});
