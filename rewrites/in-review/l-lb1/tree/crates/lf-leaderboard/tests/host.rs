//! Host tests for the lifted slots: edges, maps, domains, descriptors.
//!
//! These run on every host (no verified rewrites involved). The differential
//! proof against the verified rewrites is crate `lf-lb-diff` (32-bit only).

use lf_leaderboard::desc;
use lf_leaderboard::desc::LeaderboardDesc;
use lf_leaderboard::tables::BoardTables;
use lf_leaderboard::tables::BoardView;
use lf_leaderboard::tables::NOT_FOUND;
use lf_leaderboard::tables::{class_rank, class_tag, fetch_id, find_index, joined_fetch, reverse_lookup};

/// Owned fake collaborator with a call log.
struct Store {
    ok: bool,
    count: u32,
    a: Vec<u32>,
    b: Option<Vec<u32>>,
    answer: u32,
    calls: Vec<(char, u32)>,
}

impl Store {
    fn new(a: &[u32], count: u32) -> Self {
        Self {
            ok: true,
            count,
            a: a.to_vec(),
            b: None,
            answer: 0,
            calls: Vec::new(),
        }
    }

    fn desc(board_id: u32) -> LeaderboardDesc {
        LeaderboardDesc {
            board: "test",
            board_id,
            rows: 0,
        }
    }
}

impl BoardTables for Store {
    fn fetch(&mut self, board: u32) -> Option<BoardView<'_>> {
        self.calls.push(('f', board));
        if !self.ok {
            return None;
        }
        Some(BoardView {
            count: self.count,
            primary: &self.a,
            secondary: self.b.as_deref(),
        })
    }

    fn classify(&mut self, value: u32) -> u32 {
        self.calls.push(('c', value));
        { let _ = value; self.answer }
    }
}

#[test]
fn find_index_edges() {
    let d = Store::desc(0x5e);
    // Hit first, middle, last; miss.
    let mut s = Store::new(&[10, 20, 30], 3);
    assert_eq!(find_index(&mut s, &d, 10), 0);
    assert_eq!(find_index(&mut s, &d, 20), 1);
    assert_eq!(find_index(&mut s, &d, 30), 2);
    assert_eq!(find_index(&mut s, &d, 99), NOT_FOUND);
    // First match wins.
    let mut s = Store::new(&[7, 7, 7], 3);
    assert_eq!(find_index(&mut s, &d, 7), 0);
    // Fetch failure, zero and negative counts find nothing.
    let mut s = Store::new(&[10], 1);
    s.ok = false;
    assert_eq!(find_index(&mut s, &d, 10), NOT_FOUND);
    let mut s = Store::new(&[10], 0);
    assert_eq!(find_index(&mut s, &d, 10), NOT_FOUND);
    let mut s = Store::new(&[10], 0xFFFF_FFFF);
    assert_eq!(find_index(&mut s, &d, 10), NOT_FOUND);
    let mut s = Store::new(&[10], 0x8000_0000);
    assert_eq!(find_index(&mut s, &d, 10), NOT_FOUND);
    // The board id reaches the fetch.
    assert_eq!(s.calls[0], ('f', 0x5e));
}

#[test]
fn fetch_id_edges() {
    let d = Store::desc(1);
    let mut s = Store::new(&[0xAAAA, 0xBBBB], 2);
    assert_eq!(fetch_id(&mut s, &d, 0), 0xAAAA);
    assert_eq!(fetch_id(&mut s, &d, 1), 0xBBBB);
    s.ok = false;
    assert_eq!(fetch_id(&mut s, &d, 0), NOT_FOUND);
}

#[test]
fn class_tag_map() {
    let d = Store::desc(1);
    // 1->4, 2|3->8, 5->4, anything else->0.
    for (answer, tag) in [(1, 4), (2, 8), (3, 8), (4, 0), (5, 4), (0, 0), (6, 0), (0xFFFF_FFFF, 0)] {
        let mut s = Store::new(&[0x1234], 1);
        s.answer = answer;
        assert_eq!(class_tag(&mut s, &d, 0), tag, "answer {answer}");
        assert_eq!(s.calls, vec![('f', 1), ('c', 0x1234)]);
    }
    let mut s = Store::new(&[0x1234], 1);
    s.ok = false;
    assert_eq!(class_tag(&mut s, &d, 0), 0);
}

#[test]
fn class_rank_map() {
    let d = Store::desc(1);
    // 1->0, 2->1, 3->3, 4->MISS, 5->2, anything else->MISS.
    for (answer, rank) in [
        (1, 0),
        (2, 1),
        (3, 3),
        (4, NOT_FOUND),
        (5, 2),
        (0, NOT_FOUND),
        (6, NOT_FOUND),
        (0xFFFF_FFFF, NOT_FOUND),
    ] {
        let mut s = Store::new(&[0x1234], 1);
        s.answer = answer;
        assert_eq!(class_rank(&mut s, &d, 0), rank, "answer {answer}");
    }
    let mut s = Store::new(&[0x1234], 1);
    s.ok = false;
    assert_eq!(class_rank(&mut s, &d, 0), NOT_FOUND);
}

#[test]
fn reverse_lookup_edges() {
    let d = Store::desc(1);
    let mut s = Store::new(&[50, 60], 3);
    s.b = Some(vec![60, 70, 50]);
    assert_eq!(reverse_lookup(&mut s, &d, 0), 2);
    assert_eq!(reverse_lookup(&mut s, &d, 1), 0);
    // Key absent from B.
    let mut s = Store::new(&[50], 1);
    s.b = Some(vec![51]);
    assert_eq!(reverse_lookup(&mut s, &d, 0), NOT_FOUND);
    // Key of MISS, zero count, fetch failure.
    let mut s = Store::new(&[NOT_FOUND], 1);
    s.b = Some(vec![NOT_FOUND]);
    assert_eq!(reverse_lookup(&mut s, &d, 0), NOT_FOUND);
    let mut s = Store::new(&[50], 0);
    s.b = Some(vec![50]);
    assert_eq!(reverse_lookup(&mut s, &d, 0), NOT_FOUND);
    let mut s = Store::new(&[50], 1);
    s.b = Some(vec![50]);
    s.ok = false;
    assert_eq!(reverse_lookup(&mut s, &d, 0), NOT_FOUND);
}

#[test]
fn joined_fetch_edges() {
    let d = Store::desc(1);
    let mut s = Store::new(&[50, 60], 2);
    s.b = Some(vec![500, 600]);
    assert_eq!(joined_fetch(&mut s, &d, 50), 500);
    assert_eq!(joined_fetch(&mut s, &d, 60), 600);
    assert_eq!(joined_fetch(&mut s, &d, 70), NOT_FOUND);
    let mut s = Store::new(&[50], 0);
    s.b = Some(vec![500]);
    assert_eq!(joined_fetch(&mut s, &d, 50), NOT_FOUND);
    let mut s = Store::new(&[50], 0xFFFF_FFFF);
    s.b = Some(vec![500]);
    assert_eq!(joined_fetch(&mut s, &d, 50), NOT_FOUND);
    let mut s = Store::new(&[50], 1);
    s.b = Some(vec![500]);
    s.ok = false;
    assert_eq!(joined_fetch(&mut s, &d, 50), NOT_FOUND);
}

#[test]
fn descriptors_cover_every_board() {
    assert_eq!(desc::DESCRIPTORS.len(), 447);
    assert_eq!(desc::BOARDS_WITHOUT_IDS.len(), 30);
    let mut ids: Vec<u32> = desc::DESCRIPTORS.iter().map(|d| d.board_id).collect();
    ids.sort_unstable();
    ids.dedup();
    // Eight ids are each shared by a ranked/unranked pair; the sharing is
    // name-only (no two disagree on a known parameter: rows 0 is unknown).
    assert_eq!(ids.len(), 439);
    for id in &ids {
        let rows: Vec<u32> = desc::DESCRIPTORS
            .iter()
            .filter(|d| d.board_id == *id)
            .map(|d| d.rows)
            .collect();
        let known: Vec<u32> = rows.iter().copied().filter(|r| *r != 0).collect();
        assert!(
            known.iter().all(|r| *r == known[0]),
            "id {id} has mixed known rows"
        );
    }
    assert!(ids.iter().all(|&id| (1..=481).contains(&id)));
    // Spot lookups.
    assert!(desc::describe(ids[0]).is_some());
    assert!(desc::describe(0).is_none());
    assert!(desc::describe(0xFFFF_FFFF).is_none());
}

#[test]
#[should_panic(expected = "fetch_id: table index 2 outside 2 words")]
fn fetch_id_out_of_domain_panics() {
    let d = Store::desc(1);
    let mut s = Store::new(&[1, 2], 2);
    let _ = fetch_id(&mut s, &d, 2);
}

#[test]
#[should_panic(expected = "find_index: table index 1 outside 1 words")]
fn find_index_short_table_panics() {
    let d = Store::desc(1);
    let mut s = Store::new(&[9], 2);
    let _ = find_index(&mut s, &d, 8);
}

#[test]
#[should_panic(expected = "without a second table")]
fn two_table_slot_needs_both_tables() {
    let d = Store::desc(1);
    let mut s = Store::new(&[50], 1);
    let _ = reverse_lookup(&mut s, &d, 0);
}
