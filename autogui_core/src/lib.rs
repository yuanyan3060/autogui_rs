mod controller;
mod error;
pub use controller::{AdbBuilder, Controller, ADB};
#[cfg(test)]
mod tests {
    use crate::controller::AdbBuilder;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let mut adb = AdbBuilder::new()
            .with_target("127.0.0.1:7555")
            .with_bin_path(r"adb.exe")
            .build()?
            .into_dyn();
        adb.screenshot()?.save("screenshot.png")?;
        Ok(())
    }
}
