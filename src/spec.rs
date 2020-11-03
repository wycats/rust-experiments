use derive_new::new;
use laboratory_expectations::traits::MatchResult;

use crate::{
    curry::Function, curry::TypedFunction, spec_result::FinishedSpec, suite::mutable::WeakRef,
};

use std::fmt::Debug;

// type Callback<T: 'static> = Box<dyn FnMut(&mut T) -> Result<(), String> + 'static>;

type Callback<T> = TypedFunction<T, MatchResult>;

#[derive(new)]
pub struct It {
    #[new(default)]
    specs: Vec<TypedSpec<()>>,
}

impl It {
    pub(crate) fn specs(self) -> Vec<TypedSpec<()>> {
        self.specs
    }

    pub fn should(&mut self, name: impl Into<String>, spec: impl Fn() -> MatchResult + 'static) {
        self.specs
            .push(TypedSpec::new(name, move |_: &mut ()| spec()));
    }

    pub fn only(&mut self, name: impl Into<String>, spec: impl Fn() -> MatchResult + 'static) {
        self.specs
            .push(TypedSpec::new(name, move |_: &mut ()| spec()).only());
    }

    pub fn skip(&mut self, name: impl Into<String>, spec: impl Fn() -> MatchResult + 'static) {
        self.specs
            .push(TypedSpec::new(name, move |_: &mut ()| spec()).skip());
    }
}

#[derive(new)]
pub struct TypedMutableIt<T>
where
    T: 'static,
{
    #[new(default)]
    specs: Vec<TypedSpec<WeakRef<T>>>,
}

impl<T> TypedMutableIt<T>
where
    T: 'static,
{
    pub(crate) fn specs(self) -> Vec<TypedSpec<WeakRef<T>>> {
        self.specs
    }

    fn callback(
        spec: impl Fn(&mut T) -> MatchResult + 'static,
    ) -> impl Fn(&mut WeakRef<T>) -> MatchResult {
        move |input: &mut WeakRef<T>| input.mut_ref(|v| spec(v))
    }

    fn spec(
        name: impl Into<String>,
        spec: impl Fn(&mut T) -> MatchResult + 'static,
    ) -> TypedSpec<WeakRef<T>> {
        TypedSpec::new(name, TypedMutableIt::callback(spec))
    }

    pub fn should(
        &mut self,
        name: impl Into<String>,
        spec: impl Fn(&mut T) -> MatchResult + 'static,
    ) {
        self.specs.push(TypedMutableIt::spec(name, spec));
    }

    pub fn only(
        &mut self,
        name: impl Into<String>,
        spec: impl Fn(&mut T) -> MatchResult + 'static,
    ) {
        self.specs.push(TypedMutableIt::spec(name, spec).only());
    }

    pub fn skip(
        &mut self,
        name: impl Into<String>,
        spec: impl Fn(&mut T) -> MatchResult + 'static,
    ) {
        self.specs.push(TypedMutableIt::spec(name, spec).skip());
    }
}

#[derive(new)]
pub struct TypedIt<T>
where
    T: Debug + Clone + 'static,
{
    #[new(default)]
    specs: Vec<TypedSpec<T>>,
}

impl<T> TypedIt<T>
where
    T: Debug + Clone + 'static,
{
    pub(crate) fn specs(self) -> Vec<TypedSpec<T>> {
        self.specs
    }

    pub fn should(
        &mut self,
        name: impl Into<String>,
        spec: impl Fn(&mut T) -> MatchResult + 'static,
    ) {
        self.specs.push(TypedSpec::new(name, spec));
    }

    pub fn only(
        &mut self,
        name: impl Into<String>,
        spec: impl Fn(&mut T) -> MatchResult + 'static,
    ) {
        self.specs.push(TypedSpec::new(name, spec));
    }

    pub fn skip(
        &mut self,
        name: impl Into<String>,
        spec: impl Fn(&mut T) -> MatchResult + 'static,
    ) {
        self.specs.push(TypedSpec::new(name, spec));
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ShouldRun {
    Always,
    Only,
    Never,
}

#[derive(Debug)]
pub struct Spec {
    pub name: String,
    callback: Function<'static, MatchResult>,
    running: ShouldRun,
}

impl Spec {
    pub(crate) fn run(self, suite_name: impl Into<String>) -> FinishedSpec {
        let Spec {
            name,
            running,
            mut callback,
        } = self;

        match running {
            ShouldRun::Never => FinishedSpec::skipped(suite_name, name),
            _ => {
                let result = callback.call();

                FinishedSpec::ran(suite_name, name, result)
            }
        }
    }

    pub(crate) fn in_only_suite(mut self) -> Spec {
        match self.running {
            ShouldRun::Always | ShouldRun::Never => {
                self.running = ShouldRun::Never;
            }
            ShouldRun::Only => self.running = ShouldRun::Only,
        }

        self
    }

    pub(crate) fn is_only(&self) -> bool {
        self.running == ShouldRun::Only
    }
}

pub struct TypedSpec<T>
where
    T: 'static,
{
    pub name: String,
    callback: Callback<T>,
    running: ShouldRun,
}

impl<T> Debug for TypedSpec<T>
where
    T: Clone + Debug + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedSpec")
            .field("name", &self.name)
            .field("running", &self.running)
            .finish()
    }
}

impl<T> TypedSpec<T>
where
    T: 'static,
{
    pub(crate) fn new(
        name: impl Into<String>,
        callback: impl Fn(&mut T) -> MatchResult + 'static,
    ) -> TypedSpec<T> {
        TypedSpec {
            name: name.into(),
            callback: TypedFunction::new(callback),
            running: ShouldRun::Always,
        }
    }

    pub fn skip(mut self) -> TypedSpec<T> {
        self.running = ShouldRun::Never;
        self
    }

    pub fn only(mut self) -> TypedSpec<T> {
        self.running = ShouldRun::Only;
        self
    }

    pub fn with_state(self, state: impl FnMut() -> T + 'static) -> Spec {
        Spec {
            name: self.name,
            callback: self.callback.hide_fn(state),
            running: self.running,
        }
    }
}

// pub struct Spec<T>
// where
//     T: Clone,
// {
//     pub name: String,
//     pub test: Box<dyn FnMut(&mut State<T>) -> Result<(), String>>,
//     pub pass: Option<bool>,
//     pub error_msg: Option<String>,
//     pub ignore: bool,
//     pub only_: bool,
//     pub time_started: Option<Instant>,
//     // pub time_ended: Option<Instant>,
//     pub duration: Option<Duration>,
// }
// impl<T> Spec<T>
// where
//     T: Clone,
// {
//     pub fn new<H>(name: String, handle: H) -> Spec<T>
//     where
//         H: FnMut(&mut State<T>) -> Result<(), String> + 'static,
//     {
//         Spec {
//             name,
//             test: Box::new(handle),
//             pass: None,
//             error_msg: None,
//             ignore: false,
//             only_: false,
//             time_started: None,
//             // time_ended: None,
//             duration: None,
//         }
//     }
//     pub fn skip(mut self) -> Self {
//         self.ignore = true;
//         self
//     }
//     pub fn only(mut self) -> Self {
//         self.only_ = true;
//         self
//     }

//     //noinspection RsMatchCheck
//     pub fn run(&mut self, state: &mut State<T>) {
//         let test: &mut dyn FnMut(&mut State<T>) -> Result<(), String> = self.test.borrow_mut();
//         if !self.ignore {
//             let start_time = Instant::now();
//             match (test)(state) {
//                 Ok(_) => {
//                     self.pass = Some(true);
//                 }
//                 Err(message) => {
//                     self.pass = Some(false);
//                     self.error_msg = Some(message);
//                 } /*,
//                   _ => {
//                       self.pass = Some(false);
//                       self.error_msg = Some("something happened".to_string());
//                   }*/
//             }
//             self.time_started = Some(start_time);
//             // self.time_ended = Some(Instant::now());
//             self.duration = Some(start_time.elapsed())
//         }
//     }
//     pub fn export_results(&self, suite_name: &str) -> SpecResult {
//         SpecResult::new(
//             suite_name,
//             &self.name,
//             self.pass,
//             &self.error_msg,
//             self.time_started,
//         )
//     }
// }
