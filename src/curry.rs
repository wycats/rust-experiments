use std::{fmt::Debug, marker::PhantomData};

pub struct TypedFunction<In, Out>
where
    Out: 'static,
{
    function: Box<dyn FnMut(&mut In) -> Out + 'static>,
}

impl<In, Out> TypedFunction<In, Out>
where
    Out: 'static,
{
    pub fn new(function: impl FnMut(&mut In) -> Out + 'static) -> TypedFunction<In, Out> {
        TypedFunction {
            function: Box::new(function),
        }
    }

    #[cfg(test)]
    pub fn curry_mut(self, arg: &'_ mut In) -> TypedCurriedMutFunction<'_, In, Out> {
        TypedCurriedMutFunction {
            function: self.function,
            argument: arg,
        }
    }

    #[cfg(test)]
    pub fn curry(self, arg: In) -> TypedCurriedFunction<'static, In, Out>
    where
        In: Clone + 'static,
    {
        TypedCurriedFunction {
            function: self.function,
            argument: Box::new(move || arg.clone()),
        }
    }

    pub fn hide_fn<'a>(self, arg: impl FnMut() -> In + 'a) -> Function<'a, Out>
    where
        In: 'a,
    {
        TypedCurriedFunction {
            function: self.function,
            argument: Box::new(arg),
        }
        .hide()
    }

    #[cfg(test)]
    pub fn hide(self, arg: In) -> Function<'static, Out>
    where
        In: Clone + 'static,
    {
        self.curry(arg).hide()
    }
}

#[cfg(test)]
pub struct TypedCurriedMutFunction<'a, In, Out>
where
    In: 'static,
    Out: 'static,
{
    function: Box<dyn FnMut(&mut In) -> Out + 'static>,
    argument: &'a mut In,
}

#[cfg(test)]
impl<'a, In, Out> TypedCurriedMutFunction<'a, In, Out>
where
    In: 'static,
    Out: 'static,
{
    pub fn hide(self) -> Function<'a, Out> {
        let TypedCurriedMutFunction {
            mut function,
            argument,
        } = self;

        Function {
            function: Box::new(move || (function)(argument)),
            lt: PhantomData,
        }
    }
}

pub struct TypedCurriedFunction<'a, In, Out>
where
    In: 'a,
    Out: 'static,
{
    function: Box<dyn FnMut(&mut In) -> Out + 'static>,
    argument: Box<dyn FnMut() -> In + 'a>,
}

impl<'a, In, Out> TypedCurriedFunction<'a, In, Out>
where
    In: 'a,
    Out: 'static,
{
    pub fn hide(self) -> Function<'a, Out> {
        let TypedCurriedFunction {
            mut function,
            mut argument,
        } = self;

        Function {
            function: Box::new(move || {
                let mut argument = (argument)();
                (function)(&mut argument)
            }),
            lt: PhantomData,
        }
    }
}

pub struct Function<'a, Out> {
    function: Box<dyn FnMut() -> Out + 'a>,
    lt: PhantomData<&'a mut ()>,
}

impl<'a, Out> Debug for Function<'a, Out> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function")
    }
}

impl<'a, Out> Function<'a, Out> {
    pub fn call(&mut self) -> Out {
        (self.function)()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn len() -> TypedFunction<String, usize> {
        TypedFunction::new(|s: &mut String| s.len())
    }

    fn hide<T>(function: TypedCurriedFunction<T, usize>) -> Function<usize> {
        function.hide()
    }

    fn hide_mut<T>(function: TypedCurriedMutFunction<'_, T, usize>) -> Function<usize> {
        function.hide()
    }

    #[test]
    fn test_curry() {
        let niko_len = len().curry("Niko Matsakis".to_string());
        let mut niko_len = hide(niko_len);

        assert_eq!(niko_len.call(), "Niko Matsakis".len());

        // can be called twice
        assert_eq!(niko_len.call(), "Niko Matsakis".len());
    }

    #[test]
    fn test_mut() {
        let mut string = "Niko Matsakis".to_string();
        let niko_len = len().curry_mut(&mut string);
        let mut niko_len = hide_mut(niko_len);

        assert_eq!(niko_len.call(), "Niko Matsakis".len());

        // can be called twice
        assert_eq!(niko_len.call(), "Niko Matsakis".len());
    }

    #[test]
    fn test_typed() {
        let len_function = len();
        let mut niko_len = len_function.hide("Niko Matsakis".to_string());

        assert_eq!(niko_len.call(), "Niko Matsakis".len());

        // can be called twice
        assert_eq!(niko_len.call(), "Niko Matsakis".len());
    }
}
