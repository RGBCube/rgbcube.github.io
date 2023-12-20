macro_rules! import {
    ($($ident:ident),*) => {
        $(
            mod $ident;
            pub use $ident::generate as $ident;
        )*
    };
}

import! { index }
