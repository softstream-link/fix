macro_rules! asserted_short_name {
    ($name:literal, $ty:ty) => {{
        #[cfg(debug_assertions)]
        {
            use std::any::type_name;
            let expected_short_name = type_name::<$ty>().split('<').next().unwrap().split("::").last().unwrap_or("Unknown");
            debug_assert_eq!(
                $name, expected_short_name,
                "Please check that you correct manual Debug & Display impl after refactoring"
            );
            expected_short_name
        }
        #[cfg(not(debug_assertions))]
        {
            $name
        }
    }};
}

pub(crate) use asserted_short_name;
