pub trait Logger {
    fn log(&self, message: impl Into<String>) {
        println!("{}", message.into());
    }

    fn elog(&self, message: impl Into<String>) {
        eprintln!("{}", message.into());
    }
}
