macro_rules! oserr {
    () => {
        std::io::Error::last_os_error()
    };
}
#[allow(unused)]
macro_rules! assert_ok {
    ($val:ident) => {
        assert!($val.is_ok(), "{:?}", $val.unwrap_err())
    };
    ($e:expr) => {
        let tmp = $e;
        assert_ok!(tmp)
    };
}
