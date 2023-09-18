mod set;
mod operators;
pub use set::{Set, SetOrItem};
pub use ecow::EcoString;
pub use identconv::lower_strify;
pub use convert_case;

/// Construct a game oriented enum or enumset that uses strings as keys
/// this stores all its data in `flatlowercase` to avoid some typos.
#[macro_export]
macro_rules! str_set {
    ($name: ident : [$($fields: ident),*]) => {
        #[derive(Debug, Clone, Eq, Hash)]
        pub struct $name(::str_set::EcoString);

        const _: () = {
            use ::serde::{Serialize, Deserialize};

            #[allow(non_upper_case_globals)]
            impl $name {
                $(pub const $fields: Self = Self(::str_set::EcoString::inline(
                    ::str_set::lower_strify!($fields)
                ));)*

                pub fn new(s: &str) -> Self {
                    use ::str_set::convert_case::{Casing, Case::*};
                    Self(s.to_case(Flat).into())
                }

                fn new_unchecked(s: &str) -> Self {
                    Self(s.into())
                }
            }

            impl ::std::fmt::Display for $name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.0.as_str())
                }
            }

            impl ::std::str::FromStr for $name {
                type Err=::std::convert::Infallible;

                fn from_str(s: &str) -> Result<Self, Self::Err>{
                    Ok(Self(s.into()))
                }
            }

            impl Into<::str_set::Set<Self>> for $name {
                fn into(self) -> ::str_set::Set<Self> {
                    ::str_set::Set::new(self)
                }
            }

            impl ::str_set::SetOrItem<Self> for $name {
                type Iter<'t> = ::std::iter::Once<&'t Self>;
                fn items<'t>(&'t self) -> Self::Iter<'t>{
                    ::std::iter::once(self)
                }
            }

            impl AsRef<str> for $name {
                fn as_ref(&self) -> &str {
                    self.0.as_ref()
                }
            }

            impl<T: AsRef<str>> ::std::cmp::PartialEq<T> for $name {
                fn eq(&self, other: &T) -> bool {
                    self.0 == other.as_ref()
                }
            }

            impl ::std::ops::BitOr for $name {
                type Output = ::str_set::Set<Self>;
                fn bitor(self, rhs: Self) -> Self::Output {
                    ::str_set::Set::pair(self, rhs)
                }
            }

            impl Serialize for $name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
                    self.0.serialize(serializer)
                }
            }

            impl<'de> Deserialize<'de> for $name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                    if deserializer.is_human_readable() {
                        let s = <::std::borrow::Cow<str>>::deserialize(deserializer)?;
                        Ok(Self::new(s.as_ref()))
                    } else {
                        let s = <::std::borrow::Cow<str>>::deserialize(deserializer)?;
                        Ok(Self::new_unchecked(s.as_ref()))
                    }
                }
            }
        };

    };
}
