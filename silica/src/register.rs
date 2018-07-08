//! # The `read / modify / write` Register model
//!
//!

use core::fmt;
use core::mem::size_of;
use core::ops::{Deref, DerefMut, Not, ShrAssign};
use core::ptr;

#[repr(C)]
pub struct ReservedCell<T>(T);

#[repr(C)]
pub struct RoRegisterCell<T>(T);
impl<T: Copy> RoRegisterCell<T> {
    #[inline]
    pub fn get(&self) -> T {
        unsafe { ptr::read_volatile(&self.0 as *const _) }
    }
}

///
#[repr(C)]
pub struct RegisterCell<T>(T);
impl<T: Copy> RegisterCell<T> {
    #[inline]
    pub fn get(&self) -> T {
        unsafe { ptr::read_volatile(&self.0 as *const _) }
    }
    #[inline]
    pub fn get_mut(&mut self) -> RegisterCellProxy<T> {
        let v = self.get();
        RegisterCellProxy::new(self, v)
    }
    #[inline]
    pub fn update<F>(&mut self, f: F)
    where
        for<'u> F: FnOnce(&'u mut T),
    {
        let mut v = self.get();
        f(&mut v);
        self.set(v);
    }
    #[inline]
    pub fn set(&mut self, v: T) {
        unsafe { ptr::write_volatile(&mut self.0 as *mut _, v) }
    }
}
impl<T: fmt::Debug + Copy> fmt::Debug for RegisterCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.get().fmt(f)
    }
}

/// A proxy to a RegisterCell that flushes when it runs out of scope.
pub struct RegisterCellProxy<'a, T: 'a + Copy> {
    vc: &'a mut RegisterCell<T>,
    cache: T,
}
impl<'a, T: Copy> RegisterCellProxy<'a, T> {
    fn new(v: &'a mut RegisterCell<T>, t: T) -> RegisterCellProxy<T> {
        RegisterCellProxy { vc: v, cache: t }
    }
}
impl<'a, T: Copy> Deref for RegisterCellProxy<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}
impl<'a, T: Copy> DerefMut for RegisterCellProxy<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cache
    }
}
impl<'a, T: Copy> Drop for RegisterCellProxy<'a, T> {
    fn drop(&mut self) {
        self.vc.set(self.cache);
    }
}

/// Describes a Field
pub struct Field {
    msb: usize,
    lsb: usize,
}
impl Field {
    pub const fn new(msb: usize, lsb: usize) -> Field {
        Field { msb, lsb }
    }
    #[inline]
    pub fn lsb(&self) -> usize {
        self.lsb
    }
    #[inline]
    pub fn width(&self) -> usize {
        self.msb - self.lsb + 1
    }
    #[inline]
    pub fn mask<T>(&self) -> T
    where
        T: Default + Not<Output = T> + ShrAssign<usize>,
    {
        let reg_width = size_of::<T>() * 8;
        let field_width = self.width();
        let mut mask = !T::default();
        if field_width <= reg_width {
            mask >>= reg_width - field_width;
        }
        mask
    }
}

#[macro_export]
macro_rules! register_field {
    (bool: $(#[$attr:meta])* $vis:vis $name:ident, _: $id:expr) => {
        $(#[$attr])*
        $vis fn $name(&self) -> bool {
            self.extract(&Self::FIELDS[$id]) != 0
        }
    };
    ($t:ty: $(#[$attr:meta])* $vis:vis $name:ident, _: $id:expr) => {
        $(#[$attr])*
        $vis fn $name(&self) -> $t {
            self.extract(&Self::FIELDS[$id]).try_into().unwrap()
        }
    };
    ($t:ty: _, $(#[$attr:meta])* $vis:vis $name:ident: $id:expr) => {
        $(#[$attr])*
        $vis fn $name(&mut self, v: $t) {
            self.insert(&Self::FIELDS[$id], v.into())
        }
    }
}

#[macro_export]
macro_rules! register_fields {
    (([$(($fields:expr))*],$id:expr): $(#[$attr:meta])* bool: $(#[$gattr:meta])* $gvis:vis $getter:ident, $(#[$sattr:meta])* $svis:vis $setter:ident: $bit:expr; $($rest:tt)*) => {
        register_field!(bool: $(#[$attr])* $(#[$gattr])* $gvis $getter, _: $id);
        register_field!(bool: _, $(#[$attr])* $(#[$sattr])* $svis $setter: $id);
        register_fields!(([$(($fields))* (Field::new($bit, $bit))], $id + 1): $($rest)*);
    };
    (([$(($fields:expr))*],$id:expr): $(#[$attr:meta])* bool: $(#[$gattr:meta])* $gvis:vis $getter:ident, _: $bit:expr; $($rest:tt)*) => {
        register_field!(bool: $(#[$attr])* $(#[$gattr])* $gvis $getter, _: $id);
        register_fields!(([$(($fields))* (Field::new($bit, $bit))], $id + 1): $($rest)*);
    };
    (([$(($fields:expr))*],$id:expr): $(#[$attr:meta])* bool: _, $(#[$sattr:meta])* $svis:vis $setter:ident: $bit:expr; $($rest:tt)*) => {
        register_field!(bool: _, $(#[$attr])* $(#[$sattr])* $svis $setter: $id);
        register_fields!(([$(($fields))* (Field::new($bit, $bit))], $id + 1): $($rest)*);
    };
    (([$(($fields:expr))*],$id:expr): $(#[$attr:meta])* $t:ty: $(#[$gattr:meta])* $gvis:vis $getter:ident, $(#[$sattr:meta])* $svis:vis $setter:ident: $bit:expr; $($rest:tt)*) => {
        register_field!($t: $(#[$attr])* $(#[$gattr])* $gvis $getter, _: $id);
        register_field!($t: _, $(#[$attr])* $(#[$sattr])* $svis $setter: $id);
        register_fields!(([$(($fields))* (Field::new($bit, $bit))], $id + 1): $($rest)*);
    };
    (([$(($fields:expr))*],$id:expr): $(#[$attr:meta])* $t:ty: $(#[$gattr:meta])* $gvis:vis $getter:ident, _: $bit:expr; $($rest:tt)*) => {
        register_field!($t: $(#[$attr])* $(#[$gattr])* $gvis $getter, _: $id);
        register_fields!(([$(($fields))* (Field::new($bit, $bit))], $id + 1): $($rest)*);
    };
    (([$(($fields:expr))*],$id:expr): $(#[$attr:meta])* $t:ty: _, $(#[$sattr:meta])* $svis:vis $setter:ident: $bit:expr; $($rest:tt)*) => {
        register_field!($t: _, $(#[$attr])* $(#[$sattr])* $svis $setter: $id);
        register_fields!(([$(($fields))* (Field::new($bit, $bit))], $id + 1): $($rest)*);
    };
    (([$(($fields:expr))*],$id:expr): $(#[$attr:meta])* $t:ty: $(#[$gattr:meta])* $gvis:vis $getter:ident, $(#[$sattr:meta])* $svis:vis $setter:ident: $msb:expr, $lsb:expr; $($rest:tt)*) => {
        register_field!($t: $(#[$attr])* $(#[$gattr])* $gvis $getter, _: $id);
        register_field!($t: _, $(#[$attr])* $(#[$sattr])* $svis $setter: $id);
        register_fields!(([$(($fields))* (Field::new($msb, $lsb))], $id + 1): $($rest)*);
    };
    (([$(($fields:expr))*],$id:expr): $(#[$attr:meta])* $t:ty: $(#[$gattr:meta])* $gvis:vis $getter:ident, _: $msb:expr, $lsb:expr; $($rest:tt)*) => {
        register_field!($t: $(#[$attr])* $(#[$gattr])* $gvis $getter, _: $id);
        register_fields!(([$(($fields))* (Field::new($msb, $lsb))], $id + 1): $($rest)*);
    };
    (([$(($fields:expr))*],$id:expr): $(#[$attr:meta])* $t:ty: _, $(#[$sattr:meta])* $svis:vis $setter:ident: $msb:expr, $lsb:expr; $($rest:tt)*) => {
        register_field!($t: _, $(#[$attr])* $(#[$sattr])* $svis $setter: $id);
        register_fields!(([$(($fields))* (Field::new($msb, $lsb))], $id + 1): $($rest)*);
    };
    (([$(($fields:expr))*],$id:expr): ) => {
        #[allow(dead_code)]
        const FIELDS: [Field; $id] = [$($fields),*];
    }
}

#[macro_export]
macro_rules! register_impl_extract_insert {
    (@integer $name:ident : $t:ty) => {
        impl $name {
            #[inline]
            #[allow(dead_code)]
            fn extract(&self, f: &Field) -> $t {
                (self.0 >> f.lsb()) & f.mask::<$t>()
            }
            #[inline]
            #[allow(dead_code)]
            fn insert(&mut self, f: &Field, v: $t) {
                self.0 =
                    (self.0 & !(f.mask::<$t>() << f.lsb())) | ((v & f.mask::<$t>()) << f.lsb());
            }
        }
    };
    (@array $name:ident : $work_t:ty => $cell_t:ty) => {
        impl $name {
            fn extract(&self, _f: &Field) -> $work_t {
                unimplemented!();
            }
            fn insert(&mut self, _f: &Field, _v: $work_t) {
                unimplemented!();
            }
        }
    };
}

#[macro_export]
macro_rules! register_impl_debug {
    ($var:ident, $self:ident: $(#[$attr:meta])* $t:ty: $(#[$gattr:meta])* $gvis:vis $getter:ident, $(#[$sattr:meta])* $svis:vis $setter:ident: $bit:expr; $($rest:tt)*) => {
        // only expand getter
        register_impl_debug!($var, $self: $(#[$attr])* $t: $(#[$gattr])* $gvis $getter, _: $bit, $bit;);
        register_impl_debug!($var, $self: $($rest)*);
    };
    ($var:ident, $self:ident: $(#[$attr:meta])* $t:ty: $(#[$gattr:meta])* $gvis:vis $getter:ident, _: $bit:expr; $($rest:tt)*) => {
        // forward to regular "msb, lsb" format
        register_impl_debug!($var, $self: $(#[$attr])* $t: $(#[$gattr])* $gvis $getter, _: $bit, $bit;);
        register_impl_debug!($var, $self: $($rest)*);
    };
    ($var:ident, $self:ident: $(#[$attr:meta])* $t:ty: _, $(#[$sattr:meta])* $svis:vis $setter:ident: $bit:expr; $($rest:tt)*) => {
        // ignore write only
        register_impl_debug!($var, $self: $($rest)*);
    };
    ($var:ident, $self:ident: $(#[$attr:meta])* $t:ty: $(#[$gattr:meta])* $gvis:vis $getter:ident, $(#[$sattr:meta])* $svis:vis $setter:ident: $msb:expr, $lsb:expr; $($rest:tt)*) => {
        // only expand getter
        register_impl_debug!($var, $self: $(#[$attr])* $t: $(#[$gattr])* $gvis $getter, _: $msb, $lsb;);
        register_impl_debug!($var, $self: $($rest)*);
    };
    ($var:ident, $self:ident: $(#[$attr:meta])* $t:ty: $(#[$gattr:meta])* $gvis:vis $getter:ident, _: $msb:expr, $lsb:expr; $($rest:tt)*) => {
        $var.field(stringify!($getter), &$self.$getter());
        register_impl_debug!($var, $self: $($rest)*);
    };
    ($var:ident, $self:ident: $(#[$attr:meta])* $t:ty: _, $(#[$sattr:meta])* $svis:vis $setter:ident: $msb:expr, $lsb:expr; $($rest:tt)*) => {
        // ignore write only
        register_impl_debug!($var, $self: $($rest)*);
    };
    ($var:ident, $self:ident: ) => {
        // the end
    }
}

/// @optout_extract_insert must appear last.
#[macro_export]
macro_rules! register {
    (@impl_debug; $(@$opt:ident;)* $(#[$attr:meta])* $vis:vis struct $name:ident($t:ty) {$($fields:tt)*}) => {
        // impl debug
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut debug_struct = f.debug_struct(stringify!($name));
                register_impl_debug!(debug_struct, self: $($fields)*);
                debug_struct.finish()
            }
        }
        register!($(@$opt;)* $(#[$attr])* $vis struct $name($t) { $($fields)* });
    };
    (@optout_extract_insert; $(#[$attr:meta])* $vis:vis struct $name:ident($t:ty) {$($fields:tt)*}) => {
        $(#[$attr])*
        #[repr(C)]
        $vis struct $name($t);
        // impl fields
        impl $name {
            register_fields!(([], 0): $($fields)*);
        }
    };
    ($(#[$attr:meta])* $vis:vis struct $name:ident($t:ty) {$($fields:tt)*}) => {
        register_impl_extract_insert!(@integer $name: $t);
        register!(@optout_extract_insert; $(#[$attr])* $vis struct $name($t) { $($fields)* });
    };
    ($($t:tt)*) => { compile_error!(stringify!($($t)*)); };
}
