#[cfg(test)]
use super::*;

pub fn slots(set: &str) -> (Vec<u32>, Vec<u32>, u32, u32) {
    match set {
        // 3rd => 1
        "three" => (vec![1, 2, 3], vec![3, 2, 7], 4, 1),
        // 2nd, 3rd, 4th, 7th, 8th, 9th => 6
        "eleven" => (
            vec![1, 1, 1, 2, 2, 5, 5, 5, 6, 7, 12],
            vec![2, 7, 6, 99, 4, 5, 6, 7, 6, 7, 28],
            6,
            6,
        ),
        _ => (vec![4], vec![4], 4, 1),
    }
}

// single
#[test]
fn single_slots_busy_student_1() {
    let (s, e, q, res) = slots("single");
    assert_eq!(busy_student_1(&s, &e, q), res);
}
#[test]
fn single_slots_busy_student_2() {
    let (s, e, q, res) =slots("single");
    assert_eq!(busy_student_2(&s, &e, q), res);
}
#[test]
fn single_slots_busy_student_3() {
    let (s, e, q, res) =slots("single");
    assert_eq!(busy_student_3(&s, &e, q), res);
}
#[test]
fn single_slots_busy_student_4() {
    let (s, e, q, res) =slots("single");
    assert_eq!(busy_student_4(&s, &e, q), res);
}

// three
#[test]
fn three_slot_busy_student_1() {
    let (s, e, q, res) =slots("three");
    assert_eq!(busy_student_1(&s, &e, q), res);
}
#[test]
fn three_slot_busy_student_2() {
    let (s, e, q, res) =slots("three");
    assert_eq!(busy_student_2(&s, &e, q), res);
}
#[test]
fn three_slot_busy_student_3() {
    let (s, e, q, res) =slots("three");
    assert_eq!(busy_student_3(&s, &e, q), res);
}
#[test]
fn three_slot_busy_student_4() {
    let (s, e, q, res) =slots("three");
    assert_eq!(busy_student_4(&s, &e, q), res);
}

// eleven
#[test]
fn eleven_slot_busy_student_1() {
    let (s, e, q, res) =slots("eleven");
    assert_eq!(busy_student_1(&s, &e, q), res);
}
#[test]
fn eleven_slot_busy_student_2() {
    let (s, e, q, res) =slots("eleven");
    assert_eq!(busy_student_2(&s, &e, q), res);
}
#[test]
fn eleven_slot_busy_student_3() {
    let (s, e, q, res) =slots("eleven");
    assert_eq!(busy_student_3(&s, &e, q), res);
}
#[test]
fn eleven_slot_busy_student_4() {
    let (s, e, q, res) =slots("eleven");
    assert_eq!(busy_student_4(&s, &e, q), res);
}
