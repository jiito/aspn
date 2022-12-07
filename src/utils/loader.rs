#[macro_export]
macro_rules! with_loader {
    ($e:expr) => {{
        use spinners::{Spinner, Spinners};
        // Before calling $e call the spinner
        let mut spinner = Spinner::new(Spinners::Dots9, "Waiting...".into());
        let _res = $e;
        spinner.stop();
        _res
    }};
    ($s:stmt) => {{
        use spinners::{Spinner, Spinners};
        // Before calling $e call the spinner
        let mut spinner = Spinner::new(Spinners::Dots9, "Waiting...".into());
        let _res = $s;
        spinner.stop();
        _res
    }};
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    #[test]
    fn run_spinner() {
        with_loader!(sleep(Duration::from_secs(3)))
    }
}
