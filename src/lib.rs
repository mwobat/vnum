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
