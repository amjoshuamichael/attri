#![doc = include_str!("../README.md")]
#![feature(trait_upcasting)]
#![feature(arc_unwrap_or_clone)]

use std::any::{type_name, Any, TypeId};
#[cfg(feature = "debug")]
use std::fmt::Debug;
use unsized_vec::UnsizedVec;

#[cfg(not(feature = "debug"))]
pub trait Attribute: Any + Send + Sync + 'static {
    fn type_name(&self) -> &'static str;
}
#[cfg(not(feature = "debug"))]
impl<T> Attribute for T
where
    T: Any + Send + Sync + 'static,
{
    fn type_name(&self) -> &'static str {
        type_name::<T>()
    }
}

#[cfg(feature = "debug")]
pub trait Attribute: Any + Send + Sync + 'static + Debug {
    fn type_name(&self) -> &'static str;
}
#[cfg(feature = "debug")]
impl<T> Attribute for T
where
    T: Any + Send + Sync + 'static + Debug,
{
    fn type_name(&self) -> &'static str {
        type_name::<T>()
    }
}

#[derive(Default)]
pub struct Attributes {
    inner: UnsizedVec<dyn Attribute>,
}

#[cfg(feature = "debug")]
impl Debug for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut attributes = self.inner.iter();
        if let Some(attribute) = attributes.next() {
            write!(f, "[{} => {:?}", attribute.type_name(), attribute)?;

            for attribute in attributes {
                write!(f, ", {} => {:?}", attribute.type_name(), attribute)?;
            }

            write!(f, "]")?;
        }

        Ok(())
    }
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            inner: UnsizedVec::new(),
        }
    }

    pub fn insert<T: Attribute>(&mut self, new_data: T) {
        for attribute in &self.inner {
            if attribute.type_id() == TypeId::of::<T>() {
                panic!(
                    "attribute insertion error: type {} has already been inserted!",
                    type_name::<T>()
                );
            }
        }

        self.inner.push_unsize(new_data);
    }

    pub fn get<T: Attribute>(&self) -> Option<&T> {
        for attribute in &self.inner {
            if attribute.type_id() == TypeId::of::<T>() {
                return Some(
                    (attribute as &dyn Any)
                        .downcast_ref::<T>()
                        .expect("error getting type"),
                );
            }
        }

        None
    }

    pub fn remove<T: Attribute>(&mut self) {
        for index in 0..self.inner.len() {
            if self.inner[index].type_id() == TypeId::of::<T>() {
                self.inner.remove_into(index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Attributes;

    #[test]
    fn basic_test() {
        let mut attributes = Attributes::new();
        attributes.insert(43);
        attributes.insert("HELLO");
        attributes.insert([1, 1, 2, 3, 5, 8, 13]);

        let fourty_three = attributes.get::<i32>().unwrap();
        assert_eq!(*fourty_three, 43);

        let the_array = attributes.get::<[i32; 7]>().unwrap();
        assert_eq!(*the_array, [1, 1, 2, 3, 5, 8, 13]);

        let nonexistent = attributes.get::<String>();
        assert_eq!(nonexistent, None);
    }

    #[test]
    #[should_panic]
    fn double_insertion() {
        let mut attributes = Attributes::new();
        attributes.insert(43);
        attributes.insert(95);
    }

    #[test]
    fn removal() {
        let mut attributes = Attributes::new();
        attributes.insert(43);
        attributes.remove::<i32>();
        assert_eq!(attributes.get::<i32>(), None);
        attributes.insert(4389);
        assert_eq!(attributes.get::<i32>(), Some(&4389));
    }
}
