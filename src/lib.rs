#![deny(missing_docs)]
//! A crate containing some helpful utility macros.

/// A helper macro for constructing `BTreeMap`s.
#[macro_export]
macro_rules! btree_map {
  {} => (::std::collections::BTreeMap::new());
  {$($k:expr => $v:expr),+ $(,)?} => ({
    let mut out = $crate::btree_map!{};
    $(out.insert($k, $v);)+
    out
  });
}

/// A helper macro for constructing `BTreeSet`s.
#[macro_export]
macro_rules! btree_set {
  {} => (::std::collections::BTreeSet::new());
  {$($x:expr),+ $(,)?} => ({
    let mut out = $crate::btree_set!{};
    $(out.insert($x);)+
    out
  });
}

/// A helper macro for constructing `HashMap`s.
#[macro_export]
macro_rules! hash_map {
  {} => (::std::collections::HashMap::new());
  {$($k:expr => $v:expr),+ $(,)?} => ({
    let mut out = $crate::hash_map!{};
    $(out.insert($k, $v);)+
    out
  });
}

/// A helper macro for constructing `HashSet`s.
#[macro_export]
macro_rules! hash_set {
  {} => (::std::collections::HashSet::new());
  {$($x:expr),+ $(,)?} => ({
    let mut out = $crate::hash_set!{};
    $(out.insert($x);)+
    out
  });
}

/// Abstracts away the process of accessing or mutating an `RwLock`.
#[macro_export]
macro_rules! rwlock {
  ($state:expr) => (::std::sync::RwLock::read(&$state).unwrap());
  (mut $state:expr) => (::std::sync::RwLock::write(&$state).unwrap());
  ($state:expr, |$var:ident| $action:expr) => ({
    let $var = $crate::rwlock!($state);
    let __out__ = $action;
    drop($var); __out__
  });
  (mut $state:expr, |$var:ident| $action:expr) => ({
    let mut $var = $crate::rwlock!(mut $state);
    let __out__ = $action;
    drop($var); __out__
  });
}

/// Abstracts away the process of accessing or mutating a `Mutex`.
#[macro_export]
macro_rules! mutex {
  ($state:expr) => (::std::sync::Mutex::lock(&$state).unwrap());
  ($state:expr, |$var:ident| $action:expr) => ({
    let mut $var = $crate::mutex!($state);
    let __out__ = $action;
    drop($var); __out__
  });
}

/// Creates an enum who's variants all contain one item each, and implements `From`
/// for the enum such that each variant can be converted from the type they contain.
/// 
/// This is useful for error enum types where you need to be able to return multiple
/// errors and the try syntax (`?`) can be used to convert other errors into your custom
/// error enum.
/// 
/// Note: only supports generic lifetimes.
#[macro_export]
macro_rules! error_enum {
  {
    $v:vis enum $Enum:ident
    $(<$($lt:lifetime),+ $(,)?>)?
    { $($Variant:ident($type:ty)),+ $(,)? }
  } => {
    $v enum $Enum$(<$($lt),+>)? {
      $($Variant($type)),+
    }

    impl$(<$($lt),+>)? std::fmt::Debug for $Enum$(<$($lt),+>)? {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          $(Self::$Variant(t) => t.fmt(f),)+
        }
      }
    }

    $crate::error_enum_impl!($Enum$(<$($lt),+>)? { $($Variant($type)),+ });
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_impl {
  (
    $Enum:ident
    $(<$($lt:lifetime),+ $(,)?>)?
    { $Variant:ident($type:ty), $($VRest:ident($trest:ty)),+ }
  ) => {
    $crate::error_enum_impl!($Enum$(<$($lt),+>)? { $Variant($type) });
    $crate::error_enum_impl!($Enum$(<$($lt),+>)? { $($VRest($trest)),+});
  };
  (
    $Enum:ident
    $(<$($lt:lifetime),+ $(,)?>)?
    { $Variant:ident($type:ty) }
  ) => {
    impl$(<$($lt),+>)? From<$type> for $Enum$(<$($lt),+>)? {
      fn from(value: $type) -> Self {
        $Enum::$Variant(value)
      }
    }
  };
}

/// Allows you to have optional macro variables default to something.
/// Useful in the creation of macros.
#[macro_export]
macro_rules! macro_default {
  (@item, $d:item, $o:item) => ($o);
  (@item, $d:item $(,)?) => ($d);
  (@block, $d:block, $o:block) => ($o);
  (@block, $d:block $(,)?) => ($d);
  (@stmt, $d:stmt, $o:stmt) => ($o);
  (@stmt, $d:stmt $(,)?) => ($d);
  (@pat, $d:pat, $o:pat) => ($o);
  (@pat, $d:pat $(,)?) => ($d);
  (@expr, $d:expr, $o:expr) => ($o);
  (@expr, $d:expr $(,)?) => ($d);
  (@ty, $d:ty, $o:ty) => ($o);
  (@ty, $d:ty $(,)?) => ($d);
  (@ident, $d:ident, $o:ident) => ($o);
  (@ident, $d:ident $(,)?) => ($d);
  (@path, $d:path, $o:path) => ($o);
  (@path, $d:path $(,)?) => ($d);
  (@tt, $d:tt, $o:tt) => ($o);
  (@tt, $d:tt $(,)?) => ($d);
  (@meta, $d:meta, $o:meta) => ($o);
  (@meta, $d:meta $(,)?) => ($d);
  (@lifetime, $d:lifetime, $o:lifetime) => ($o);
  (@lifetime, $d:lifetime $(,)?) => ($d);
  (@vis, $d:vis, $o:vis) => ($o);
  (@vis, $d:vis $(,)?) => ($d);
  (@literal, $d:literal, $o:literal) => ($o);
  (@literal, $d:literal $(,)?) => ($d);
}
