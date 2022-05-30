#![no_std]

#[macro_export]
macro_rules! bitstruct {
    {
        $(
            $(#[$($outter:tt)*])*
            $vs:vis struct $name:ident: $inner:ty {
                $(
                    $f_vs:vis $f_get:ident: $from:tt..=$to:tt
                ),*
            }
        )*
    } => {
        $(
            $(#[$($outter)*])*
            #[repr(transparent)]
            $vs struct $name($inner);
    
            impl $name {
                $vs fn bits(&self) -> $inner {
                    self.0
                }

                $vs fn from_bits(bits: $inner) -> Self {
                    Self(bits)
                }

                $(
                    $f_vs fn $f_get(&self) -> $inner {
                        let mask = !(<$inner>::MAX << $to + 1);
                        let m1 = self.0 & mask;
                        let m2 = m1 >> $from;
                        m2
                    }
                )*
            }

            impl From<$inner> for $name {
                fn from(v: $inner) -> Self {
                    Self(v)
                }
            }

            impl From<$name> for $inner {
                fn from(v: $name) -> Self {
                    v.0
                }
            }
        )*
    }
}