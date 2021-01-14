use std::iter::successors;

pub fn square_of_sum(n: u32) -> u32 {
    // (1..n + 1).into_iter().fold(0, |acc, x: u32| acc + x).pow(2)
    successors(Some(1), |x: &u32| Some(x + 1))
        .take(n as usize)
        .sum::<u32>()
        .pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // (1..n + 1).into_iter().fold(0, |acc, x: u32| acc + x.pow(2))
    (1..n + 1)
        .scan(0, |s, x: u32| {
            *s = x.pow(2);
            Some(*s)
        })
        .sum::<u32>()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

// use std::iter::successors;

// fn main() {
//     let n = 10;
//     // let s = successors(Some(1u32), |x: &u32| {
//     //     Some(x+1)
//     // })
//     // .map(|x| x);
//     // println!("{:?}", s.take(n).sum::<u32>().pow(2));
//     let ss = (1..n+1).scan(0, |s, x: u32| {
//         *s = x.pow(2);
//         Some(*s)
//     });
//     println!("{:?}", ss.take(10).sum::<u32>());
//     // let ss = successors(Some(1), |x: &u32| {
//     //     let num = (x + 1);
//     //     Some(num)
//     // });
//     // println!("{:?}", ss.take(n as usize).sum::<u32>());

//     // let sum = successors(Some((0, 1)), |&(prev, curr)| {
//     //     Some((curr, prev + curr))
//     // })
//     // .map(|(prev, _)| prev);
//     // println!("{:?}", sum.take(n).collect::<Vec<u32>>());
//     // let s1 = (1..n+1).scan(0, |s, x| {
//     //     *s += x;
//     //     Some(*s)
//     // });
//     // println!("{:?}", s1.take(n).collect::<Vec<_>>());
//     // let s2 = (1..n+1).into_iter().fold(0, |acc, x: u32| acc + x.pow(2));
//     // println!("{}", s2);
// }
