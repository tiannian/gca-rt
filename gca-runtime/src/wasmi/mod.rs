mod module;
pub use module::*;

mod instance;
pub use instance::*;

mod memory;
pub use memory::*;

mod backend;
pub use backend::*;

mod external;
pub use external::*;

mod imports;
pub use imports::*;

#[cfg(test)]
mod test {
    use super::WasmiBackend;

    #[test]
    fn test_unlock() {
        env_logger::init();

        crate::executor::tests::test_empty::<WasmiBackend>();
    }
}
