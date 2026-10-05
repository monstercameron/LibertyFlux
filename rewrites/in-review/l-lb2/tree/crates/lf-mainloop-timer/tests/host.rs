//! Host tests: descriptor sanity and the one-function lift.

use lf_mainloop_timer::{TimerDesc, TimerRegistrar, desc::DESCS, register_callback};

struct Reg {
    answer: u32,
    seen: Vec<u32>,
}

impl TimerRegistrar for Reg {
    fn register(&mut self, callback: &TimerDesc) -> u32 {
        self.seen.push(callback.index);
        self.answer
    }
}

#[test]
fn timer_registers_by_index() {
    assert_eq!(DESCS.len(), 81);
    for (i, d) in DESCS.iter().enumerate() {
        assert!(!d.name.is_empty());
        assert_eq!(d.index as usize, i);
    }
    let mut reg = Reg {
        answer: 42,
        seen: Vec::new(),
    };
    assert_eq!(register_callback(&mut reg, &DESCS[7]), 42);
    assert_eq!(reg.seen, vec![7]);
}
