use lunatic::{
    process::{Message, ProcessRef, Request, StartProcess},
    Mailbox,
};
use lunatic_macros::*;

struct Counter {
    count: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Inc;
#[derive(serde::Serialize, serde::Deserialize)]
struct Count;

#[abstract_process]
impl Counter {
    #[init]
    fn init(_: ProcessRef<Self>, initial_count: u32) -> Self {
        Self::new(initial_count)
    }

    fn new(count: u32) -> Self {
        Self { count }
    }

    #[process_message]
    fn increment(&mut self, _: Inc) {
        self.count += 1;
        self.check_count();
    }

    #[process_request]
    fn count(&self, _: Count) -> u32 {
        self.count
    }

    fn check_count(&self) {
        if self.count >= 5 {
            println!("high counts!: {}", self.count);
        }
    }

    fn increment_twice(&mut self) {
        self.increment(Inc);
        self.increment(Inc);
    }
}

#[lunatic::main]
fn main(_: Mailbox<()>) {
    // use counter locally
    let mut counter = Counter::new(0);
    counter.increment(Inc);
    println!("count = {}", counter.count(Count));
    counter.increment_twice();
    println!("count = {}", counter.count(Count));

    // use counter as a process
    let counter = Counter::start_link(0, None);
    counter.increment(Inc);
    counter.increment(Inc);
    counter.increment(Inc);
    counter.increment(Inc);
    println!("count = {}", counter.count(Count));
    counter.increment(Inc);
    counter.increment(Inc);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn increment() {
        let mut counter = Counter::new(0);
        for i in 0..10 {
            assert_eq!(i, counter.count);
            counter.increment(Inc);
        }
    }

    #[test]
    fn increment_twice() {
        let mut counter = Counter::new(0);
        for i in 0..10 {
            assert_eq!(i * 2, counter.count);
            counter.increment_twice();
        }
    }
}
