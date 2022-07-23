use lunatic::{
    abstract_process,
    process::{ProcessRef, StartProcess},
    Mailbox,
};

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

    #[handle_message]
    fn increment(&mut self) {
        self.count += 1;
        self.check_count();
    }

    #[handle_request]
    fn count(&self) -> u32 {
        self.count
    }

    fn check_count(&self) {
        if self.count >= 5 {
            println!("high counts!: {}", self.count);
        }
    }

    fn increment_twice(&mut self) {
        self.increment();
        self.increment();
    }
}

#[lunatic::main]
fn main(_: Mailbox<()>) {
    // use counter locally
    let mut counter = Counter::new(0);
    counter.increment();
    println!("count = {}", counter.count());
    counter.increment_twice();
    println!("count = {}", counter.count());

    // use counter as a process
    let counter = Counter::start_link(0, None);
    counter.increment();
    counter.increment();
    counter.increment();
    counter.increment();
    println!("count = {}", counter.count());
    counter.increment();
    counter.increment();
}

#[cfg(test)]
mod test {
    use super::*;

    #[lunatic::test]
    fn increment() {
        let mut counter = Counter::new(0);
        for i in 0..10 {
            assert_eq!(i, counter.count);
            counter.increment();
        }
    }

    #[lunatic::test]
    fn increment_twice() {
        let mut counter = Counter::new(0);
        for i in 0..10 {
            assert_eq!(i * 2, counter.count);
            counter.increment_twice();
        }
    }
}
