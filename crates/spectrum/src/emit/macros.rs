#[macro_export]
macro_rules! emit {
    ($out:expr, $body:expr) => {
        $crate::emit::EmitStyled::output_block(&mut $out, $body.for_emit())
    };
}

#[macro_export]
macro_rules! emitln {
    ($out:expr, $body:expr) => {{
        use $crate::emit::StyledFragmentTrait;

        $body.for_emitln().emit_into($out)
    }};

    ($out:expr) => {{
        use $crate::emit::StyledFragmentTrait;

        $crate::emit::StyledBlock::newline()
            .for_emitln()
            .emit_into($out)
    }};
}
