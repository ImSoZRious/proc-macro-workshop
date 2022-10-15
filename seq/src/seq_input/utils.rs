macro_rules! shift {
      ($a: ident; $v: expr) => {
          $a = $v;
      };

      ($a: ident, $b: ident; $v: expr) => {
          $a = $b;
          $b = $v;
      };

      ($a: ident, $b: ident, $($c: ident),+; $v: expr) => {
          $a = $b;
          crate::seq_input::utils::shift!($b, $($c),+; $v);
      };
}

macro_rules! init {
      ($a: ident; $v: expr) => {
          let mut $a = $v;
      };

      ($a: ident, $($b: ident),*; $v: expr) => {
          let mut $a = $v;
          crate::seq_input::utils::init!($($b),*; $v);
      };
}

macro_rules! clear {
      ($a: ident; $v: expr) => {
          $a = $v;
      };

      ($a: ident, $($b: ident),*; $v: expr) => {
          $a = $v;
          crate::seq_input::utils::clear!($($b),*; $v);
      };
}

pub(super) use clear;
pub(super) use init;
pub(super) use shift;
