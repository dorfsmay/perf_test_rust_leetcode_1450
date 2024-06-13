
pub fn busy_student_1(start_time: &[u32], end_time: &[u32], query_time: u32) -> u32 {
    start_time
        .iter()
        .zip(end_time.iter())
        .filter(|(start, end)| (**start <= query_time && **end >= query_time))
        .count() as u32
}

pub fn busy_student_2(start_time: &[u32], end_time: &[u32], query_time: u32) -> u32 {
    start_time
        .iter()
        .copied()
        .zip(end_time.iter().copied())
        .filter(|(start, end)| *start <= query_time && query_time <= *end)
        .count() as u32
}

pub fn busy_student_3(start_time: &[u32], end_time: &[u32], query_time: u32) -> u32 {
    let mut count = 0;
    for n in 0..start_time.len() {
        if start_time[n] <= query_time && end_time[n] >= query_time {
            count += 1;
        }
    }
    count
}

pub fn busy_student_4(start_time: &[u32], end_time: &[u32], query_time: u32) -> u32 {
    let mut count = 0;
    let zipped = start_time.iter().zip(end_time);
    for end_start in zipped {
        if *end_start.0 <= query_time && *end_start.1 >= query_time {
            count += 1;
        }
    }
    count
}

pub mod tests;
