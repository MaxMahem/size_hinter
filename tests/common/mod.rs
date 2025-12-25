/// Unified macro to test iterator property tracking during iteration
///
///   next => Some(1), len: 3
///   next => Some(2), hint: (5, Some(10))
macro_rules! iter_state {
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
            assert_eq!(iter.size_hint(), Into::<(usize, Option<usize>)>::into($initial_hint), "size_hint should be {:?} at start", $initial_hint);
            $(
                assert_eq!(iter.$method(), $expected, "{} did not return {:?}", stringify!($method), $expected);
                assert_eq!(iter.size_hint(), Into::<(usize, Option<usize>)>::into($hint), "size_hint should be {:?} after {}", $hint, stringify!($method));
            )+
        }
    };
}

/// Macro to test initial state/construction of an iterator
///
/// (name, initial => hint: initial_hint)
/// (name, initial => len: initial_len)
/// (name, initial => panic: expected_msg)
macro_rules! initial_state {
    ($name:ident, $iter:expr, hint: $hint:expr) => {
        #[test]
        fn $name() {
            let iter = $iter;
            assert_eq!(iter.size_hint(), Into::<(usize, Option<usize>)>::into($hint), "expected size_hint to match");
            assert!(matches!(iter.into_inner(), Range { .. }))
        }
    };

    ($name:ident, $iter:expr, len: $expected:expr) => {
        #[test]
        fn $name() {
            let iter = $iter;
            assert_eq!(iter.len(), $expected);
            assert!(matches!(iter.into_inner(), Range { .. }))
        }
    };

    ($name:ident, $iter:expr, panic: $expected_msg:expr) => {
        #[test]
        #[should_panic(expected = $expected_msg)]
        fn $name() {
            let _ = $iter;
        }
    };
}

pub(crate) use initial_state;
pub(crate) use iter_state;
