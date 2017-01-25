//! Fixed Point Crate
//!
//! This crate offers type safe fixed point arthemtic mathmatics for base 10. This crate uses
//! core so it can be used anywhere you wish.
//!
//! There are MANY modules. The module name denotes the underlying integer holding the value.
//! while the FP# states how many decimal points there are. I highly recommend re-namming these
//! types on import
//!
//! Internally all calculations are integer based (except division which is fp64 based)
//!
//! This crate does not use standard so it can be used anywhere in the stack you wish
#![no_std]
#![allow(dead_code,non_snake_case)]

macro_rules! impl_thing {
    (@NORM $trait_name: ident, $kind: ty, $method: ident) => {
        impl $trait_name for $kind {
            type Output = $kind;
            #[inline(always)]
            fn $method(self, x: Self) -> Self {
                let mut a = self.clone();
                a.0 = (self.0).$method(x.0);
                a
            }
        }
    };
    (@MUT $trait_name: ident, $kind: ty, $method: ident) => {
        impl $trait_name for $kind {
            #[inline(always)]
            fn $method(&mut self, x: Self) {
                (self.0).$method(x.0);
            }
        }
    };
}
macro_rules! float_convert {
    ($kind: ty, $base: expr, $from: ty, $i: ident, $inner: ty) => {
        impl From<$from> for $kind {
            fn from(x: $from) -> Self {
                let v: $from = x * (10u64.pow($base) as $from);
                let b = v as $inner;
                $i( b )
            }
        }
        impl Into<$from> for $kind {
            fn into(self) -> $from {
                let v = self.0 as $from;
                v / (10u64.pow($base) as $from)
            }
        }
    }
}
macro_rules! kind {
    (TypeName: $name: ident; $repeat: ty; InnerKind: $kind: ty; Decimal: $val: expr; MutTraits: { $($mtn: ident => $mtm: ident),* }; ImmTraits: { $($itn: ident => $itm: ident),* }; Floats: {$($fp: ty),* }) => {
        #[derive(Copy,Clone)]
        pub struct $name ( $kind );
        impl $name {
            #[inline(always)]
            pub fn below_point(&self) -> $kind {
                let x: $kind = self.0.clone();
                x % (10u64.pow($val) as $kind)
            }
            #[inline(always)]
            pub fn above_point(&self) -> $kind {
                let x: $kind = self.0.clone();
                x / (10u64.pow($val) as $kind)
            }
        }
        impl Mul for $repeat {
            type Output = $repeat;
            #[inline(always)]
            fn mul(self, x: Self) -> Self::Output {
                let a = self.0;
                let b = x.0;
                let c = a*b;
                $name(c/ (10u64.pow($val) as $kind))
            }
        }
        impl MulAssign for $repeat {
            #[inline(always)]
            fn mul_assign(&mut self, x: Self) {
                *self = self.clone() * x;
            }
        }
        impl Div for $repeat {
            type Output = $repeat;
            #[inline(always)]
            fn div(self, x: Self) -> Self::Output {
                let a = self.0 as f64;
                let b = x.0 as f64;
                let c = a/b;
                Self::from(c)
            }
        }
        impl DivAssign for $repeat {
            fn div_assign(&mut self, x: Self) {
                *self = self.clone() / x;
            }
        }
        impl Neg for $repeat {
            type Output = $repeat;
            #[inline(always)]
            fn neg(self) -> Self::Output {
                $name( !self.0 )
            }
        }
        $(
            float_convert!($repeat, $val, $fp, $name, $kind);
        )*
        $(
            impl_thing!(@MUT $mtn, $repeat, $mtm);
        )*
        $(
            impl_thing!(@NORM $itn, $repeat, $itm);
        )*
    }
}
macro_rules! make_module {
    (@BOTH InnerType: $kind: ty; { $($name1: ident | $name2: ty => $val: expr),*}) => {
        use core::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Sub,SubAssign,Neg};
        $(
            kind!{
                TypeName: $name1; $name2;
                InnerKind: $kind;
                Decimal: $val;
                MutTraits: {
                    AddAssign => add_assign,
                    SubAssign => sub_assign
                };
                ImmTraits: {
                    Add => add,
                    Sub => sub
                };
                Floats: {
                    f64, f32
                }
            }
        )*
    };
    (InnerType: $kind: ty; { $($name1: ident | $name2: ty => $val: expr),*}) => {
        use core::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Sub,SubAssign,Neg};
        $(
            kind!{
                TypeName: $name1; $name2;
                InnerKind: $kind;
                Decimal: $val;
                MutTraits: {
                    AddAssign => add_assign,
                    SubAssign => sub_assign
                };
                ImmTraits: {
                    Add => add,
                    Sub => sub
                };
                Floats: {
                    f32
                }
            }
        )*
    };
}
pub mod I32 {
    make_module! {@BOTH
        InnerType: i32;
        {
            FP1 | FP1 => 1 ,
            FP2 | FP2 => 2 ,
            FP3 | FP3 => 3 ,
            FP4 | FP4 => 4 ,
            FP5 | FP5 => 5 ,
            FP6 | FP6 => 6 ,
            FP7 | FP7 => 7
        }
    }
}
pub mod U32 {
    make_module! {@BOTH
        InnerType: u32;
        {
            FP1 | FP1 => 1 ,
            FP2 | FP2 => 2 ,
            FP3 | FP3 => 3 ,
            FP4 | FP4 => 4 ,
            FP5 | FP5 => 5 ,
            FP6 | FP6 => 6 ,
            FP7 | FP7 => 7
        }
    }
}

pub mod I64 {
    make_module!{@BOTH
        InnerType: i64;
        {
            FP1 | FP1 => 1 ,
            FP2 | FP2 => 2 ,
            FP3 | FP3 => 3 ,
            FP4 | FP4 => 4 ,
            FP5 | FP5 => 5 ,
            FP6 | FP6 => 6 ,
            FP7 | FP7 => 7 ,
            FP8 | FP8 => 8 ,
            FP9 | FP9 => 9 ,
            FP10 | FP10 => 10 ,
            FP11 | FP11 => 11 ,
            FP12 | FP12 => 12 ,
            FP13 | FP13 => 13 ,
            FP14 | FP14 => 14
        }
    }

    /*
     * Additional tests to ensure negativity is done right
     */
     #[test]
     fn test_signed() {

     }
}
pub mod U64 {
    make_module!{@BOTH
        InnerType: u64;
        {
            FP1 | FP1 => 1 ,
            FP2 | FP2 => 2 ,
            FP3 | FP3 => 3 ,
            FP4 | FP4 => 4 ,
            FP5 | FP5 => 5 ,
            FP6 | FP6 => 6 ,
            FP7 | FP7 => 7 ,
            FP8 | FP8 => 8 ,
            FP9 | FP9 => 9 ,
            FP10 | FP10 => 10 ,
            FP11 | FP11 => 11 ,
            FP12 | FP12 => 12 ,
            FP13 | FP13 => 13 ,
            FP14 | FP14 => 14
        }
    }

    /*
     * Everything is based on the 1 input
     * powers. If it works for 1 it'll work for all
     *
     * So one test is provided which covers all paths the
     * the macro can generate
     */
    #[test]
    fn test_unsigned() {

        //tests with a f32
        let dut0 = FP3::from(3.14159f32);
        assert_eq!(dut0.0, 3141);
        assert_eq!(dut0.above_point(), 3);
        assert_eq!(dut0.below_point(), 141);

        //test multiplication
        let (x,y) = (dut0.clone(), dut0.clone());
        let z = x * y;
        assert_eq!(z.0, 9865);
        assert_eq!(z.above_point(), 9);
        assert_eq!(z.below_point(), 865);

        //test assigned multiplication
        let mut x = dut0.clone();
        let y = FP3::from(2f32);
        x *= y;
        assert_eq!(x.0, 6282);
        assert_eq!(x.above_point(), 6);
        assert_eq!(x.below_point(), 282);

        //test division
        let (x,y) = (dut0.clone(), dut0.clone());
        let z = x / y;
        assert_eq!(z.0, 1000);
        assert_eq!(z.above_point(), 1);
        assert_eq!(z.below_point(), 0);

        //more complex division
        let (mut x,y) = (FP3::from(15.18f32), FP3::from(3.256f32));
        x /= y;
        assert_eq!(x.0, 4662);
        assert_eq!(x.above_point(), 4);
        assert_eq!(x.below_point(), 662);

        //test addition
        let (x,y) = (FP3::from(105.189f32), FP3::from(256158.256f32));
        let z = x + y;
        assert_eq!(z.0, 256263445);
        assert_eq!(z.above_point(), 256263);
        assert_eq!(z.below_point(), 445);

        //test subtraction
        let (x,mut y) = (FP3::from(105.189f32), FP3::from(256158.256f32));
         y -=x;
        assert_eq!(y.0, 256053067);
        assert_eq!(y.above_point(), 256053);
        assert_eq!(y.below_point(), 67);
    }
}
