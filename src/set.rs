use std::{fmt::Display, str::FromStr};
use smallvec::SmallVec;

pub trait SetOrItem<T> {
    type Iter<'t>: Iterator<Item = &'t T> where Self: 't, T: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t>;
}

impl<T: PartialEq, const S: char> SetOrItem<T> for Set<T, S> where T: SetOrItem<T>{
    type Iter<'t> = core::slice::Iter<'t, T> where T: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        self.0.iter()
    }
}

pub struct Set<T: PartialEq, const SEP: char='|'>(pub(crate) SmallVec<[T; 2]>);

impl<T: PartialEq, const S: char> Set<T, S> {
    pub const EMPTY: Self = Self(SmallVec::new_const());

    pub fn new(value: T) -> Self {
        let mut vec = SmallVec::new_const();
        vec.push(value);
        Self(vec)
    }

    pub fn pair(value1: T, value2: T) -> Self {
        if value1 == value2{
            Self::new(value1)
        } else {
            Self(SmallVec::from_const([value1, value2]))
        }
    }

    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains(&self, t: impl SetOrItem<T>) -> bool{
        for i in t.items() {
            if self.0.iter().any(|x| x == i) {
                return true;
            }
        }
        false
    }

    pub fn iter<'t>(&'t self) -> impl Iterator<Item = &'t T>{
        self.0.iter()
    }
}

impl<T: PartialEq, const S: char> IntoIterator for Set<T, S> {
    type Item = T;
    type IntoIter = smallvec::IntoIter<[T; 2]>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'t, T: PartialEq, const S: char> IntoIterator for &'t Set<T, S> {
    type Item = &'t T;
    type IntoIter = std::slice::Iter<'t, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T: PartialEq, const S: char> Display for Set<T, S> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter();
        if let Some(item) = iter.next() {
            item.fmt(f)?;
        }
        for item in iter {
            S.fmt(f)?;
            item.fmt(f)?;
        }
        Ok(())
    }
}


impl<T: PartialEq, const S: char> FromStr for Set<T, S> where T: FromStr {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr: Result<_, _> = s.split(S).map(T::from_str).collect();
        Ok(Self(arr?))
    }
}
