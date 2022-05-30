#![no_std]

#[macro_export]
macro_rules! __get_offset_abs {
    ($from:tt..$to:tt) => { 0 };
    ($from:tt..=$to:tt) => { 1 };
}

#[macro_export]
macro_rules! __get_offset_from {
    ($from:tt..$to:tt) => { $from };
    ($from:tt..=$to:tt) => { $from };
}

#[macro_export]
macro_rules! __get_offset_to {
    ($from:tt..$to:tt) => { $to };
    ($from:tt..=$to:tt) => { $to };
}

#[macro_export]
macro_rules! bitstruct {
    {
        $(
            $(#[$($outter:tt)*])*
            $vs:vis struct $name:ident: $inner:ty {
                $(
                    $f_vs:vis $f_get:ident$(($f_set:ident))?: [$($range:tt)*]
                ),*
            }
        )*
    } => {
        $(
            $(#[$($outter)*])*
            #[repr(transparent)]
            $vs struct $name($inner);
    
            impl $name {
                $(
                    $f_vs fn $f_get(&self) -> $inner {
                        let mask = !(<$inner>::MAX << ($crate::__get_offset_to!($($range)*) + $crate::__get_offset_abs!($($range)*)));
                        let m1 = self.0 & mask;
                        let m2 = m1 >> $crate::__get_offset_from!($($range)*);
                        m2
                    }
                )*
            }
        )*
    }
}