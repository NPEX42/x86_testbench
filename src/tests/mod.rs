use crate::logger::{log, print, println};

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    use crate::{logger, qemu::{self, exit}};
    logger::debug!("Running {} Tests...", tests.len());
    for test in tests {
        test.run();
    }
    exit(qemu::ExitCode::Success);
}



#[cfg(test)]
crate::boot!(crate::tests::test_boot);

#[cfg(test)]
pub fn test_boot(_info: &'static bootloader::BootInfo) {
    use crate::{logger, qemu};

    logger::log!("Booted!");
    crate::test_start();
    crate::qemu::exit(qemu::ExitCode::Success);
}


pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("\x1b[0;34m[TEST]: \x1b[0m{}...\t", core::any::type_name::<T>());
        self();
        println!("\x1b[0;32m[OK]\x1b[0m");
    }
}