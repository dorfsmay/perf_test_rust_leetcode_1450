use std::cmp::{Ord, Ordering};
use std::env;
use std::fmt;
use std::time::{Duration, Instant};
use std::hint::black_box;

pub fn busy_student1(start_time: &[u32], end_time: &[u32], query_time: u32) -> u32 {
    start_time
        .iter()
        .zip(end_time.iter())
        .filter(|(start, end)| (**start <= query_time && **end >= query_time))
        .count() as u32
}

pub fn busy_student2(start_time: &[u32], end_time: &[u32], query_time: u32) -> u32 {
    start_time
        .iter()
        .copied()
        .zip(end_time.iter().copied())
        .filter(|(start, end)| *start <= query_time && query_time <= *end)
        .count() as u32
}

pub fn busy_student3(start_time: &[u32], end_time: &[u32], query_time: u32) -> u32 {
    let mut count = 0;
    for n in 0..start_time.len() {
        if start_time[n] <= query_time && end_time[n] >= query_time {
            count += 1;
        }
    }
    count
}

#[derive(Eq)]
struct Timing<'a> {
    description: &'a str,
    elapsed: Duration,
}

impl<'a> Ord for Timing<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.elapsed.cmp(&other.elapsed)
    }
}

impl<'a> PartialOrd for Timing<'a> {
    fn partial_cmp(&self, other: &Timing) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Timing<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.elapsed.eq(&other.elapsed)
    }
}

impl<'a> fmt::Debug for Timing<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:.2?}", self.description, self.elapsed)
    }
}

/*
struct Timings<'a> {
    results: Vec<Timing<'a>>,
}


impl<'a> Timings<'a> {
    fn new() -> Timings<'a> {
        Timings {
            results: Vec::new(),
        }
    }

}
*/

fn run<'a>(
    loops: u64,
    msg: &'a str,
    start: &'a &[u32],
    stop: &'a &[u32],
    query: u32,
    runme: &'a dyn Fn(&[u32], &[u32], u32) -> u32,
) -> Timing<'a> {
    let now = Instant::now();
    for _ in 0..loops {
        let _ = black_box(runme(black_box(start), black_box(stop), black_box(query)));
    }
    Timing {
        description: msg,
        elapsed: now.elapsed(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{} {:?}", args.len(), &args);
    if args.len() >= 2 {
        if args[1] == "verify" {
            // 3rd => 1
            let s = [1, 2, 3];
            let e = [3, 2, 7];
            let q = 4;
            println!("{:?} {:?} {} => {} (should be 1)", s, e, q, busy_student3(&s, &e, q));

            // 1
            let s = [4];
            let e = [4];
            let q = 4;
            println!("{:?} {:?} {} => {} (should be 1)", s, e, q, busy_student3(&e, &s, q));

            // 2nd, 3rd, 4th, 7th, 8th, 9th => 6
            let s = [1, 1, 1, 2,  2, 5, 5, 5, 6, 7, 12];
            let e = [2, 7, 6, 99, 4, 5, 6, 7, 6, 7, 28];
            let q = 6;
            println!("{:?} {:?} {} => {} (should be 6)", s, e, q, busy_student3(&s, &e, q));

            /*
             *
             */
        } else if args[1] == "perf" {
            let s1: &[u32] = &[1, 2, 3];
            let e1: &[u32] = &[3, 2, 7];
            let q1 = 4;
            let loops = u64::pow(2, 23);
            println!("number of loops: {}", loops);
            let label1 = "test 1 - single itier, no copied";
            let label2 = "test 2 - using iters and copies from Demon";
            let label3 = "test 3 - using a dumb iteration and square indexes";

            // warmup
            let _ = busy_student1(s1, e1, q1);
            let _ = busy_student2(s1, e1, q1);
            let _ = busy_student3(s1, e1, q1);

            // tests
            let mut res = vec![] as Vec<Timing>;
            res.push(black_box(run(black_box(loops), black_box(label1), black_box(&s1), black_box(&e1), black_box(q1), &busy_student1)));
            res.push(black_box(run(black_box(loops), black_box(label2), black_box(&s1), black_box(&e1), black_box(q1), &busy_student2)));
            res.push(black_box(run(black_box(loops), black_box(label3), black_box(&s1), black_box(&e1), black_box(q1), &busy_student3)));
            res.sort();
            println!("{:?}", &res);

            /*
            let mut res = vec![] as Vec<Timing>;
            res.push(run(loops, label1, &s1, &e1, q1, &busy_student1));
            res.push(run(loops, label3, &s1, &e1, q1, &busy_student3));
            res.push(run(loops, label2, &s1, &e1, q1, &busy_student2));
            res.sort();
            println!("{:?}", &res);

            let mut res = vec![] as Vec<Timing>;
            res.push(run(loops, label2, &s1, &e1, q1, &busy_student2));
            res.push(run(loops, label1, &s1, &e1, q1, &busy_student1));
            res.push(run(loops, label3, &s1, &e1, q1, &busy_student3));
            res.sort();
            println!("{:?}", &res);

            let mut res = vec![] as Vec<Timing>;
            res.push(run(loops, label2, &s1, &e1, q1, &busy_student2));
            res.push(run(loops, label3, &s1, &e1, q1, &busy_student3));
            res.push(run(loops, label1, &s1, &e1, q1, &busy_student1));
            res.sort();
            println!("{:?}", &res);

            let mut res = vec![] as Vec<Timing>;
            res.push(run(loops, label3, &s1, &e1, q1, &busy_student3));
            res.push(run(loops, label1, &s1, &e1, q1, &busy_student1));
            res.push(run(loops, label2, &s1, &e1, q1, &busy_student2));
            res.sort();
            println!("{:?}", &res);

            let mut res = vec![] as Vec<Timing>;
            res.push(run(loops, label3, &s1, &e1, q1, &busy_student3));
            res.push(run(loops, label2, &s1, &e1, q1, &busy_student2));
            res.push(run(loops, label1, &s1, &e1, q1, &busy_student1));
            res.sort();
            println!("{:?}", &res);
            */
        } else {
            println!("\"{}\" is not an option.", args[1]);
        }
    } else {
        println!("Use argument \"perf\" or \"verify\".");
    }
}
