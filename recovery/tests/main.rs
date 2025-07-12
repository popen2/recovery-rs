use recovery::{Recovery, RecoveryStrategy};

#[derive(Recovery)]
#[recovery(never)]
enum Error {
    #[recovery(auto)]
    Temporary,
    #[recovery(manual)]
    Maybe,
    Fatal,
    Fatal2((), #[allow(dead_code)] i32),
    Fatal3 {
        #[allow(dead_code)]
        field1: String,
        #[allow(dead_code)]
        field2: bool,
    },
}

fn main() {
    assert_eq!(Error::Temporary.recovery(), RecoveryStrategy::Auto);
    assert_eq!(Error::Maybe.recovery(), RecoveryStrategy::Manual);
    assert_eq!(Error::Fatal.recovery(), RecoveryStrategy::Never);
    assert_eq!(Error::Fatal2((), 2).recovery(), RecoveryStrategy::Never);
    assert_eq!(
        Error::Fatal3 {
            field1: "field1".into(),
            field2: false,
        }
        .recovery(),
        RecoveryStrategy::Never
    );
}
