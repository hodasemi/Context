/// check result macro, where the Ok() is the type which should be returned, else print error and return
#[macro_export]
macro_rules! check_and_return {
    ($v:expr) => {
        match $v {
            Ok(t) => t,
            Err(err) => {
                if let Some(trace) = err.backtrace() {
                    println!("{}", trace);
                }

                for cause in Fail::iter_causes(&err) {
                    println!("caused by: {}", cause);
                }

                return;
            }
        }
    };
}

#[macro_export]
macro_rules! create_error {
    ($v:expr) => {
        return Err(UtilError::from($v));
    };
}
