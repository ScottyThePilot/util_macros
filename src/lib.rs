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
  ($state:expr, |$var:ident| $action:block) => ({
    let $var = $crate::rwlock!($state);
    let __out__ = $action;
    drop($var); __out__
  });
  (mut $state:expr, |$var:ident| $action:block) => ({
    let mut $var = $crate::rwlock!(mut $state);
    let __out__ = $action;
    drop($var); __out__
  });
}

/// Abstracts away the process of accessing or mutating a `Mutex`.
#[macro_export]
macro_rules! mutex {
  ($state:expr) => (::std::sync::Mutex::lock(&$state).unwrap());
  ($state:expr, |$var:ident| $action:block) => ({
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
macro_rules! union_enum {
  {
    $v:vis enum $Enum:ident
    $(<$($lt:lifetime),+ $(,)?>)?
    { $($Variant:ident($type:ty)),+ $(,)? }
  } => {
    $v enum $Enum$(<$($lt),+>)? {
      $($Variant($type)),+
    }

    $crate::union_enum_impl!($Enum$(<$($lt),+>)? { $($Variant($type)),+ });
  };
  
}

#[doc(hidden)]
#[macro_export]
macro_rules! union_enum_impl {
  (
    $Enum:ident
    $(<$($lt:lifetime),+ $(,)?>)?
    { $Variant:ident($type:ty), $($VRest:ident($trest:ty)),+ }
  ) => {
    $crate::union_enum_impl!($Enum$(<$($lt),+>)? { $Variant($type) });
    $crate::union_enum_impl!($Enum$(<$($lt),+>)? { $($VRest($trest)),+});
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
