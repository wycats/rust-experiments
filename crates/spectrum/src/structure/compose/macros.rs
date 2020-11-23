#[macro_export]
macro_rules! list {
    ($($expr:expr),*) => {{
        $crate::structure::compose::DocList::new($crate::list_impl!($($expr),*))
    }}
}

#[macro_export]
macro_rules! group {
    ($($expr:expr),*) => {{
        $crate::structure::compose::Group::new($crate::list_impl!($($expr),*))
    }}
}

#[doc(hidden)]
#[macro_export]
macro_rules! list_impl {
    ($($expr:expr),*) => {{
        #[allow(unused)]
        use $crate::Doc;

        #[allow(unused_mut)]
        let mut vec: Vec<$crate::BoxedDoc> = vec![];

        $(
            vec.push($expr.boxed());
        )*

        vec
    }}
}

#[macro_export]
macro_rules! doc {
    ($name:ident as $struct_name:ident |$ctx:pat, $state:pat| $expr:expr) => {
        doc! { generate => $name as $struct_name lt = {} plus = {} struct = { ; } args = {} |_, $ctx, $state| $expr }
    };

    ($name:ident as $struct_name:ident { $($arg:ident : $arg_ty:ty),* } |$this:pat, $ctx:pat, $state:pat| $expr:expr) => {
        doc! { generate => $name as $struct_name lt = {} plus = {} struct = { { $($arg : $arg_ty),* } } args = { $($arg : $arg_ty),* } |$this, $ctx, $state| $expr }
    };

    ($name:ident as $struct_name:ident <$lt:tt> { $($arg:ident : $arg_ty:ty),* } |$this:pat, $ctx:pat, $state:pat| $expr:expr) => {
        doc! { generate => $name as $struct_name lt = { <$lt> } plus = { + $lt } struct = { { $($arg : $arg_ty),* } } args = { $($arg : $arg_ty),* } |$this, $ctx, $state| $expr }
    };

    (generate => $name:ident as $struct_name:ident lt = { $($lt:tt)* } plus = { $($plus:tt)* } struct = { $struct:tt } args = { $($arg:ident : $arg_ty:ty),* } |$this:pat, $ctx:pat, $state:pat| $expr:expr) => {
        #[derive(Debug)]
        pub struct $struct_name $($lt)* $struct

        impl $($lt)* $crate::structure::compose::Doc for $struct_name $($lt)* {
            fn render<'ctx>(&'ctx self, $ctx: &'ctx StyledArena<'ctx>, $state: $crate::render::RenderState) -> StyledDoc<'ctx> {
                let $this = self;

                $expr
            }
        }

        #[allow(unused)]
        pub fn $name$($lt)*($($arg : $arg_ty),*) -> impl $crate::structure::compose::Doc $($plus)* {
            $struct_name { $($arg),* }
        }
    };
}

#[macro_export]
macro_rules! either {
    (inline: $inline:expr, block: $block:expr) => {{
        use $crate::structure::compose::docs::either;
        use $crate::Doc;

        either($inline.boxed(), $block.boxed())
    }};
}

#[macro_export]
macro_rules! nest {
    ({ $($structure:tt)* } before = $start_gap:expr; after = $end_gap:expr; ) => {
        $crate::structure::compose::list::Nested::new(
            $crate::structure::render::Nesting::Configured(1),
            Box::new($crate::list![$($structure)*]),
            Box::new($start_gap),
            Box::new($end_gap),
        )
    };
}

#[macro_export]
macro_rules! join {
    ($items:expr, with $delimiter:expr, trail) => {
        join!(generate $items, with $delimiter, trail = true)
    };

    ($items:expr, with $delimiter:expr) => {
        join!(generate $items, with $delimiter, trail = false)
    };

    (generate $items:expr, with $delimiter:expr, trail = $trail:tt) => {
        $crate::structure::compose::join::JoinList::new(
            Box::new($delimiter),
            $crate::structure::nonempty::NonemptyList::new($items),
            $trail,
        )
    };
}
