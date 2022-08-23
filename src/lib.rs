// 1
// #[macro_export]
// macro_rules! value_enum {
//     (
//         $(#[$outer_attr:meta])*
//         $vis:vis enum $name:ident : $ty:ty {
//             $(
//                 $(#[$variant_attr:meta])*
//                 $variant:ident = $data:expr
//             ),*
//             $(,)?
//         }
//     ) => (
//         $(#[$outer_attr])*
//         $vis struct $name($ty);
//         #[allow(non_upper_case_globals)]
//         impl $name {
//             $(
//                 $(#[$variant_attr])*
//                 $vis const $variant: Self = Self($data);
//             )*
//         }
//         impl From<$name> for $ty {
//             fn from(v: $name) -> $ty {
//                 v.0
//             }
//         }
//     )
// }
// #[derive(Debug)]
// pub struct ValueEnumSealed(());

// 2
// struct Sealed<T>(T);

// #[macro_export]
// macro_rules! value_enum {
//     (
//         $(#[$outer_attr:meta])*
//         $vis:vis enum $name:ident : $ty:ty {
//             $(
//                 $(#[$variant_attr:meta])*
//                 $variant:ident = $data:expr
//             ),*
//             $(,)?
//         }
//     ) => (
//         // fn __value_enum_marker_func(_x: $ty) {}
//         struct __ValueEnumMarker;
//         $(#[$outer_attr])*
//         $vis struct $name($ty, $crate::Sealed<__ValueEnumMarker>);
//         #[allow(non_upper_case_globals)]
//         impl $name {
//             $(
//                 $(#[$variant_attr])*
//                 $vis const $variant: Self = Self($data, $crate::Sealed(__ValueEnumMarker));
//             )*
//         }
//         impl From<$name> for $ty {
//             fn from(v: $name) -> $ty {
//                 v.0
//             }
//         }
//     )
// }

// 3
// macro_rules! __concat_idents {
//     ($($e:ident),+ $(,)?) => (
//         ::std::concat_idents!($($e),+)
//     )
// }

#[macro_export]
macro_rules! value_enum {
    (
        $(#[$outer_attr:meta])*
        $vis:vis enum $name:ident : $ty:ty {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident = $data:expr
            ),*
            $(,)?
        }
    ) => (
        $(#[$outer_attr])*
        ///
        /// `Note:` This enum was automatically created by the `value_enum` crate.
        $vis enum $name {
            $(
                $(#[$variant_attr])*
                ///
                /// # Value
                /// ```ignore
                #[doc = ::std::stringify!($data)]
                /// ```
                $variant,
            )*
        }
        impl $name {
            #[allow(non_upper_case_globals)]
            $vis const fn value(&self) -> $ty {
                $(
                    pub const $variant: $ty = $data;
                )*
                match self {
                    $(
                        $name::$variant => $variant,
                    )*
                }
            }
        }
        impl ::std::convert::From<$name> for $ty {
            #[inline]
            fn from(v: $name) -> $ty {
                v.value()
            }
        }
    )
}
