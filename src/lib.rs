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
        /// <hr style="border-top:0px;border-color:#D2991D;">
        ///
        /// `Note:` This enum was automatically created with the `value_enum` crate.
        $vis enum $name {
            $(
                $(#[$variant_attr])*
                /// <hr style="border-top:0px;border-color:#D2991D;">
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
            /// Get the value associated with the enum variant.
            ///
            /// Go to the
            #[doc = ::std::concat!("[`Variants`](", ::std::stringify!($name), "#variants)")]
            /// section to see which enum variant corresponds to which associated value.
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
