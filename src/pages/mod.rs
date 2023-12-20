macro_rules! import_page {
    ($($ident:ident),*) => {
        $(
            mod $ident;
            pub use $ident::generate as $ident;
        )*
    };
}

macro_rules! import_router {
    ($($ident:ident),*) => {
        $(
            mod $ident;
            pub use $ident::router as $ident;
        )*
    };
}

import_page! { _404, index }
import_router! { assets }
