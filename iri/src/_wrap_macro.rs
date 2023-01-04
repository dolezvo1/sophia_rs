#[macro_export]
/// This macro is used to create a read-only wrapper around a type T,
/// usually for the purpose of guaranteeing that the wrapped value verifies some condition.
/// This macro takes care of defining all the usual traits for the wrapper type.
///
/// In its most general form, it is used like this:
/// ```
/// # #[macro_use] extern crate sophia_iri;
/// # trait SomeTrait { fn check_some_property(&self) -> bool; }
/// # #[derive(Debug)]
/// # struct SomeError {}
/// wrap! { MyWrapper<T: SomeTrait> :
///     // NB: the trait bound is required, as well as the trailing colon (':').
///
///     // You can include members in the impl of the wrapper type.
///     // At the very least, you must define a `new` constructor,
///     // that will check the appropriate condition on the wrapped value,
///     // and return a `Result`.
///
///     /// The `new` constructor should carry the documentation of the type;
///     /// since the generated documentation for the type will point to it.
///     pub fn new(inner: T) -> Result<Self, SomeError> {
///         if inner.check_some_property() {
///             Ok(Self(inner))
///         } else {
///             Err(SomeError {})
///         }
///     }
/// }
/// ```
///
/// A more specific form is when the trait that T must satisfy
/// is [`std::borrow::Borrow<U>`], in which case more traits
/// can be implemented by the wrapper. This is achieved with
/// the following syntax:
/// ```
/// # #[macro_use] extern crate sophia_iri;
/// # #[derive(Debug)]
/// # struct SomeError {}
/// wrap! { Foo borrowing str :
///     /// As before
///     pub fn new(inner: T) -> Result<Self, String> {
///         if inner.borrow().contains("foo") {
///             Ok(Self(inner))
///         } else {
///             Err(format!("{:?} is not a valid Foo", inner.borrow()))
///         }
///     }
/// }
/// ```
///
/// Two [examples](crate::wrap_macro_examples) are availble,
/// illustrating the members and trait implementation generated by this macro.
///
/// NB: the documentation of the wrapper will point to the documentation of the `new` method.
macro_rules! wrap {
    ($wid:ident<$tid:ident: $bound:path>: $new:item $($item:item)*) => {
        #[derive(Clone, Copy, Debug)]

        #[doc = concat!(
            "See [`",
            stringify!($wid),
            "::new`].",
        )]
        pub struct $wid<$tid: $bound>($tid);

        impl<$tid: $bound> $wid<$tid> {
            $new
            $($item)*

            #[doc = concat!(
                "Construct a `",
                stringify!($wid),
                "<T>` without checking that the inner value is valid. ",
                "If it is not, it may result in undefined behaviour.",
            )]
            #[allow(dead_code)]
            pub fn new_unchecked(inner: $tid) -> Self {
                if cfg!(debug_assertions) {
                    Self::new(inner).unwrap()
                } else {
                    Self(inner)
                }
            }

            /// Returns the wrapped value, consuming `self`.
            #[allow(dead_code)]
            pub fn unwrap(self) -> $tid {
                self.0
            }

            #[doc = concat!(
                "Map a `",
                stringify!($wid),
                "<T>` to a `",
                stringify!($wid),
                "<U>` by applying a function to the wrapped value. ",
                "It does not check that the value returned by the function is valid. ",
                "If it is not, it may result in undefined behaviour.",
            )]
            #[allow(dead_code)]
            pub fn map_unchecked<F, U>(self, f: F) -> $wid<U>
            where
                F: FnOnce(T) -> U,
                U: $bound,
            {
                let inner = self.unwrap();
                let new_inner: U = f(inner);
                $wid(new_inner)
            }
        }

        impl<T: $bound> std::ops::Deref for $wid<T> {
            type Target = T;
            fn deref(&self) -> &T {
                &self.0
            }
        }

        impl<T: $bound> std::convert::AsRef<T> for $wid<T> {
            fn as_ref(&self) -> &T {
                &self.0
            }
        }

        impl<T: $bound> std::borrow::Borrow<T> for $wid<T> {
            fn borrow(&self) -> &T {
                &self.0
            }
        }

        impl<T, U> std::cmp::PartialEq<$wid<T>> for $wid<U>
        where
            T: $bound,
            U: $bound + std::cmp::PartialEq<T>,
        {
            fn eq(&self, rhs: &$wid<T>) -> bool {
                self.0 == rhs.0
            }
        }

        impl<T> std::cmp::Eq for $wid<T>
        where
            T: $bound + std::cmp::Eq,
        {}

        impl<T, U> std::cmp::PartialOrd<$wid<T>> for $wid<U>
        where
            T: $bound,
            U: $bound + std::cmp::PartialOrd<T>,
        {
            fn partial_cmp(&self, rhs: &$wid<T>) -> std::option::Option<std::cmp::Ordering> {
                std::cmp::PartialOrd::partial_cmp(&self.0, &rhs.0)
            }
        }

        impl<T> std::cmp::Ord for $wid<T>
        where
            T: $bound + std::cmp::Eq + std::cmp::Ord,
        {
            fn cmp(&self, rhs: &$wid<T>) -> std::cmp::Ordering {
                std::cmp::Ord::cmp(&self.0, &rhs.0)
            }
        }


        impl<T> std::hash::Hash for $wid<T>
        where
            T: $bound + std::hash::Hash,
        {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
    };
    ($wid:ident borrowing $bid:ty: $new:item $($item:item)*) => {
        $crate::wrap!($wid<T: std::borrow::Borrow<$bid>>: $new $($item)*);

        impl<T> $wid<T>
        where
            T: std::borrow::Borrow<$bid>,
        {
            #[doc = concat!(
                "Convert from `&",
                stringify!($wid),
                "<T>` to `",
                stringify!($wid),
                "<&",
                stringify!($bid),
                ">`.",
            )]
            #[allow(dead_code)]
            pub fn as_ref(&self) -> $wid<&$bid> {
                $wid(self.0.borrow())
            }
        }

        impl $wid<&'static $bid> {
            #[doc = concat!(
                "Construct a `",
                stringify!($wid),
                "<&'static ",
                stringify!($bid),
                ">` without checking that the inner value is valid. ",
                "If it is not, it may result in undefined behaviour.",
            )]
            #[allow(dead_code)]
            pub const fn new_unchecked_const(inner: &'static $bid) -> Self {
                $wid(inner)
            }
        }

        impl<T: std::borrow::Borrow<$bid>> std::convert::AsRef<$bid> for $wid<T> {
            fn as_ref(&self) -> &$bid {
                &self.0.borrow()
            }
        }

        impl<T: std::borrow::Borrow<$bid>> std::borrow::Borrow<$bid> for $wid<T> {
            fn borrow(&self) -> &$bid {
                &self.0.borrow()
            }
        }

        impl<T> std::cmp::PartialEq<$bid> for $wid<T>
        where
            T: std::borrow::Borrow<$bid>,
            $bid: std::cmp::PartialEq,
        {
                fn eq(&self, other: &$bid) -> bool {
                    self.0.borrow() == other
                }
        }

        impl<T> std::cmp::PartialEq<$wid<T>> for $bid
        where
            T: std::borrow::Borrow<$bid>,
            $bid: std::cmp::PartialEq,
        {
                fn eq(&self, other: &$wid<T>) -> bool {
                    self == other.0.borrow()
                }
        }

        impl<T> std::cmp::PartialOrd<$bid> for $wid<T>
        where
            T: std::borrow::Borrow<$bid>,
            $bid: std::cmp::PartialOrd,
        {
            fn partial_cmp(&self, other: &$bid) -> std::option::Option<std::cmp::Ordering> {
                self.0.borrow().partial_cmp(other)
            }
        }

        impl<T> std::cmp::PartialOrd<$wid<T>> for $bid
        where
            T: std::borrow::Borrow<$bid>,
            $bid: std::cmp::PartialOrd,
        {
            fn partial_cmp(&self, other: &$wid<T>) -> std::option::Option<std::cmp::Ordering> {
                self.partial_cmp(other.0.borrow())
            }
        }
    };
}

#[cfg(test)]
pub mod test_simple_wrap {
    pub trait Number {
        fn even(&self) -> bool;
    }
    impl Number for i32 {
        fn even(&self) -> bool {
            *self % 2 == 0
        }
    }
    impl Number for isize {
        fn even(&self) -> bool {
            *self % 2 == 0
        }
    }

    wrap! { Even<T: Number>:
        pub fn new(inner: T) -> Result<Self, ()> {
            if inner.even() {
                Ok(Even(inner))
            } else {
                Err(())
            }
        }
    }

    #[test]
    fn constructor_succeeds() {
        assert!(Even::new(42).is_ok());
    }

    #[test]
    fn constructor_fails() {
        assert!(Even::new(43).is_err());
    }

    // only check that this compiles
    #[allow(dead_code)]
    fn unwrap() {
        let even = Even(42);
        let _: isize = even.unwrap();
    }

    // only check that this compiles
    #[allow(dead_code)]
    fn deref() {
        let even = Even(42);
        let _: &isize = &even;
    }

    // only check that this compiles
    #[allow(dead_code)]
    fn as_ref() {
        let even = Even(42);
        let _: &isize = even.as_ref();
    }

    // only check that this compiles
    #[allow(dead_code)]
    fn borrow() {
        use std::borrow::Borrow;
        let even = Even(42);
        let _: &isize = even.borrow();
    }
}

#[cfg(test)]
pub mod test_wrap_borrowing {
    wrap! { Foo borrowing str :
        /// The constructor of Foo
        pub fn new(inner: T) -> Result<Self, ()> {
            if inner.borrow().contains("foo") {
                Ok(Foo(inner))
            } else {
                Err(())
            }
        }

        pub fn as_str(&self) -> &str {
            self.0.borrow()
        }
    }

    #[test]
    fn new_succeeds() {
        assert!(Foo::new("this foo is good").is_ok());
    }

    #[test]
    fn new_fails() {
        assert!(Foo::new("this bar is bad").is_err());
    }

    #[test]
    fn partial_eq() {
        let f1a = Foo::new("foo1".to_string()).unwrap();
        let f1b = Foo::new("foo1").unwrap();
        let f2 = Foo::new("foo2").unwrap();
        assert_eq!(&f1a, "foo1");
        assert_eq!(&f1a, "foo1");
        assert_eq!(&f2, "foo2");
        assert_ne!(&f2, "foo1");
        assert_eq!("foo1", &f1a);
        assert_eq!("foo1", &f1a);
        assert_eq!("foo2", &f2);
        assert_ne!("foo1", &f2);
        assert_eq!(f1a.as_str(), f1b.as_str());
    }

    // only check that this compiles
    #[allow(dead_code)]
    fn new_unchecked() {
        let _: Foo<String> = Foo::new_unchecked("".into());
    }

    // only check that this compiles
    #[allow(dead_code)]
    fn unwrap() {
        let foo = Foo("foo".to_string());
        let _: String = foo.unwrap();
    }

    // only check that this compiles
    #[allow(dead_code)]
    fn deref() {
        let foo = Foo("this foo is good".to_string());
        let _: &String = &foo;
        let _: &str = &foo;
    }

    // only check that this compiles
    #[allow(dead_code)]
    fn as_ref_trait() {
        let foo = Foo("this foo is good".to_string());
        let _: &String = AsRef::as_ref(&foo);
        let _: &str = AsRef::as_ref(&foo);
    }

    // only check that this compiles
    #[allow(dead_code)]
    fn borrow() {
        use std::borrow::Borrow;
        let foo = Foo("this foo is good".to_string());
        let _: &String = foo.borrow();
        let _: &str = foo.borrow();
    }

    // only check that this compiles
    #[allow(dead_code)]
    fn as_ref() {
        let foo = Foo("this foo is good".to_string());
        let _: Foo<&str> = foo.as_ref();
    }
}
