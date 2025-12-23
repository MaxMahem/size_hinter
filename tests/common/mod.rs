/// Unified macro to test iterator property tracking during iteration
/// Can track either len or hint using labeled syntax:
///   next => Some(1), len: 3
///   next => Some(2), hint: (5, Some(10))
macro_rules! test_iter_hint_state {
    // (name, initial => len: len, ( method => expected, len: remaining );+ )
    ($name:ident, $initial:expr => len: $len:expr, $( $method:ident => $expected:expr, len: $remaining:expr );+ $(;)?) => {
        #[test]
        fn $name() {
            let mut iter = $initial;
            assert_eq!(iter.len(), $len, "len should be {} at start", $len);
            assert_eq!(iter.size_hint(), ($len, Some($len)), "size_hint should be ({}, Some({})) at start", $len, $len);
            $(
                assert_eq!(iter.$method(), $expected, "{} did not return {:?}", stringify!($method), $expected);
                assert_eq!(iter.len(), $remaining, "len should be {} after {}", $remaining, stringify!($method));
                assert_eq!(iter.size_hint(), ($remaining, Some($remaining)), "size_hint should be ({}, Some({})) after {}", $remaining, $remaining, stringify!($method));
            )+
        }
    };

    // (name, initial => hint: initial_hint, ( method => expected, hint: hint );+ )
    ($name:ident, $initial:expr => hint: $initial_hint:expr, $( $method:ident => $expected:expr, hint: $hint:expr );+ $(;)?) => {
        #[test]
        fn $name() {
            let mut iter = $initial;
            assert_eq!(iter.size_hint(), $initial_hint, "size_hint should be {:?} at start", $initial_hint);
            $(
                assert_eq!(iter.$method(), $expected, "{} did not return {:?}", stringify!($method), $expected);
                assert_eq!(iter.size_hint(), $hint, "size_hint should be {:?} after {}", $hint, stringify!($method));
            )+
        }
    };
}

macro_rules! test_size_hint {
    ($name:ident, $iter:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let iter = $iter;
            assert_eq!(iter.size_hint(), $expected);
            assert!(matches!(iter.into_inner(), Range { .. }))
        }
    };
}

macro_rules! test_len {
    ($name:ident, $iter:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let iter = $iter;
            assert_eq!(iter.len(), $expected);
        }
    };
}

pub(crate) use test_iter_hint_state;
pub(crate) use test_len;
pub(crate) use test_size_hint;
