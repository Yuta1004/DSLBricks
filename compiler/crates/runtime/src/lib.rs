pub trait Runnable {
    fn run(&self) -> anyhow::Result<()>;
}
