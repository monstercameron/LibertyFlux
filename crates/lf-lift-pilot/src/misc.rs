//! Small ends: a value list and a one-shot table init
//! (lifted from `list_contains` and `oneshot_table_init`).

/// Index into [`ListArena::nodes`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ListId(pub u32);

/// A `{value, next}` list node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListNode {
    /// Payload word.
    pub value: u32,
    /// Next link.
    pub next: Option<ListId>,
}

/// All nodes the list walk can reach.
#[derive(Clone, Debug, Default)]
pub struct ListArena {
    /// Nodes by index.
    pub nodes: Vec<ListNode>,
}

/// Report whether any node holds `val`. (Original: `list_contains`.)
pub fn list_contains(arena: &ListArena, head: Option<ListId>, val: u32) -> bool {
    let mut node = head;
    while let Some(id) = node {
        let nd = &arena.nodes[id.0 as usize];
        if nd.value == val {
            return true;
        }
        node = nd.next;
    }
    false
}

/// Which setup program the one-shot registration runs. The original passes
/// a code address; the lift names the program instead.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SetupProgram {
    /// The table setup reached through the original's code pointer.
    Default,
}

/// State for the one-shot table init: the done latch, the table handle and
/// the entry count. All three were globals in the original.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OneShotState {
    /// Done latch (the original's flag byte).
    pub done: bool,
    /// Table handle (the original's table pointer, opaque here).
    pub table: u32,
    /// Entry count (the original's count word).
    pub count: u16,
}

/// First call with a nonzero entry count registers the table through the
/// setup step, then latches the done flag so later calls do nothing.
/// (Original: `oneshot_table_init`.)
pub fn oneshot_table_init(
    st: &mut OneShotState,
    setup: &mut dyn FnMut(u32, u32, u32, SetupProgram),
) {
    if st.done {
        return;
    }
    if st.count != 0 {
        setup(st.table, st.count as u32, 8, SetupProgram::Default);
    }
    st.done = true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_contains_model() {
        let arena = ListArena {
            nodes: vec![
                ListNode {
                    value: 5,
                    next: Some(ListId(1)),
                },
                ListNode {
                    value: 7,
                    next: Some(ListId(2)),
                },
                ListNode {
                    value: 9,
                    next: None,
                },
            ],
        };
        assert!(list_contains(&arena, Some(ListId(0)), 7));
        assert!(!list_contains(&arena, Some(ListId(0)), 8));
        assert!(!list_contains(&arena, None, 5));
        assert!(list_contains(&arena, Some(ListId(2)), 9));
    }

    #[test]
    fn oneshot_latches() {
        let mut st = OneShotState {
            done: false,
            table: 0x1234,
            count: 3,
        };
        let mut calls = Vec::new();
        {
            let mut setup = |t: u32, c: u32, w: u32, p: SetupProgram| calls.push((t, c, w, p));
            oneshot_table_init(&mut st, &mut setup);
            oneshot_table_init(&mut st, &mut setup);
        }
        assert!(st.done);
        assert_eq!(calls, vec![(0x1234, 3, 8, SetupProgram::Default)]);
        // Zero count still latches without calling setup.
        let mut st0 = OneShotState {
            done: false,
            table: 1,
            count: 0,
        };
        {
            let mut setup = |t: u32, c: u32, w: u32, p: SetupProgram| calls.push((t, c, w, p));
            oneshot_table_init(&mut st0, &mut setup);
        }
        assert!(st0.done);
        assert_eq!(calls.len(), 1);
    }
}
