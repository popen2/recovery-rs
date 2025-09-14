#[derive(recovery::Recovery)]
enum PrimaryError {
    #[recovery(auto)]
    Temporary,
    #[recovery(manual)]
    Maybe,

    #[recovery(never)]
    Fatal,
    #[recovery(never)]
    Fatal2((), #[allow(dead_code)] i32),
    #[recovery(never)]
    Fatal3 {
        #[allow(dead_code)]
        field1: String,
        #[allow(dead_code)]
        field2: bool,
    },

    #[recovery(transparent)]
    Secondary(SecondaryError),

    #[recovery(transparent)]
    WithDefault(WithDefault),
}

#[derive(recovery::Recovery)]
enum SecondaryError {
    #[recovery(auto)]
    SecondaryAuto,
}

#[derive(recovery::Recovery)]
#[recovery(never)]
enum WithDefault {
    A,
    B,
}

#[derive(recovery::Recovery)]
enum WithGenerics<G> {
    #[recovery(transparent)]
    WithDefault(WithDefault),

    #[recovery(auto)]
    Result(Result<G, ()>),
}

fn main() {
    use recovery::{Recovery, RecoveryStrategy};

    assert_eq!(PrimaryError::Temporary.recovery(), RecoveryStrategy::Auto);
    assert_eq!(PrimaryError::Maybe.recovery(), RecoveryStrategy::Manual);
    assert_eq!(PrimaryError::Fatal.recovery(), RecoveryStrategy::Never);
    assert_eq!(
        PrimaryError::Fatal2((), 2).recovery(),
        RecoveryStrategy::Never
    );
    assert_eq!(
        PrimaryError::Fatal3 {
            field1: "field1".into(),
            field2: false,
        }
        .recovery(),
        RecoveryStrategy::Never
    );

    assert_eq!(
        PrimaryError::Secondary(SecondaryError::SecondaryAuto).recovery(),
        RecoveryStrategy::Auto
    );

    assert_eq!(
        PrimaryError::WithDefault(WithDefault::A).recovery(),
        RecoveryStrategy::Never
    );
    assert_eq!(
        PrimaryError::WithDefault(WithDefault::B).recovery(),
        RecoveryStrategy::Never
    );

    assert_eq!(
        WithGenerics::<u64>::WithDefault(WithDefault::A).recovery(),
        RecoveryStrategy::Never
    );

    assert_eq!(
        WithGenerics::<u64>::Result(Result::Ok(5)).recovery(),
        RecoveryStrategy::Auto
    );
}
