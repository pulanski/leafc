/// This macro comes from https://github.com/Kixiron/lasso/blob/master/src/util.rs. It is used to
/// compile a list of items with different `#[cfg]` attributes.
#[macro_export]
macro_rules! compile {
    ($(
        if #[$meta:meta] {
            $($item:item)*
        } $(else if #[$else_if_meta:meta] {
            $($else_if_item:item)*
        })* $(else {
            $($else_item:item)*
        })?
    )+) => {
        $(
            $(
                #[cfg($meta)]
                $item
            )*

            compile!{
                @inner
                ( $meta, )
                $(else if #[$else_if_meta] {
                    $( $else_if_item )*
                })* $(else {
                    $( $else_item )*
                })?
            }
        )+
    };

    (@recurse
        ($($prev_metas:tt)*)
        ($new_meta:meta)
        $($rem:tt)*
    ) => {
        compile!{
            @inner
            ($( $prev_metas )* $new_meta,)
            $( $rem )*
        }
    };

    (@inner
        $prev_metas:tt
        else if #[$meta:meta] {
            $($else_if_item:item)*
        }
        $($rem:tt)*

    ) => {
        $(
            #[cfg(all(not(any $prev_metas), $meta))]
            $else_if_item
        )*

        compile! {
            @recurse $prev_metas ($meta) $( $rem )*
        }
    };

    (@inner
        $prev_metas:tt
        else {
            $($else_item:item)*
        }
    )=>{
        $(
            #[cfg(not(any $prev_metas))]
            $else_item
        )*
    };

    (@inner ($($prev_metas:tt)*))=>{};
}
