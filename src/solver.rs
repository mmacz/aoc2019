use std::fmt::Display;

pub trait Solver {
    type Ans1: Display;
    type Ans2: Display;

    fn solution1(&self) -> Self::Ans1;
    fn solution2(&self) -> Self::Ans2;

    fn solve(&self, day: u32) {
        let a1 = self.solution1();
        let a2 = self.solution2();
        println!("Day {} answer 1: {}", day, a1);
        println!("Day {} answer 2: {}", day, a2);
    }
}


