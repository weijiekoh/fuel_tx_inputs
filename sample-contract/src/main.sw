contract;

abi TickTock {
    #[storage(read, write)]
    fn tick();

    #[storage(read, write)]
    fn tock();

    #[storage(read, write)]
    fn ticktock();

    #[storage(read)]
    fn count_ticks() -> u64;

    #[storage(read)]
    fn count_tocks() -> u64;
}

storage {
    ticks: u64 = 0,
    tocks: u64 = 0,
}

impl TickTock for Contract {
    #[storage(read)]
    fn count_ticks() -> u64 {
        storage.ticks.read()
    }

    #[storage(read)]
    fn count_tocks() -> u64 {
        storage.tocks.read()
    }

    #[storage(read, write)]
    fn tick() {
        let incremented = storage.ticks.read() + 1;
        storage.ticks.write(incremented);
    }

    #[storage(read, write)]
    fn tock() {
        let incremented = storage.tocks.read() + 1;
        storage.tocks.write(incremented);
    }

    #[storage(read, write)]
    fn ticktock() {
        storage.ticks.write(storage.ticks.read() + 1);
        storage.tocks.write(storage.tocks.read() + 1);
    }
}
