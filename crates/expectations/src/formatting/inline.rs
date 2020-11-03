use std::fmt::Debug;

use derive_new::new;

use crate::traits::Described;

use super::traits::DescribeMatcher;
use super::Semantic;

#[derive(new)]
pub struct PatternMatcher<T>
where
    T: Clone + Debug,
{
    pattern: Described<T>,
    relationship: String,
}

impl<T> DescribeMatcher<T> for PatternMatcher<T>
where
    T: Clone + Debug + 'static,
{
    fn expected(&self, emitter: &mut StyledEmitter) -> std::io::Result<()> {
        use Semantic::*;

        let line = inline!(
            " should " [Relationship: self.relationship] " " [Expected: self.pattern.description]);

        line.emit_into(emitter)?;

        Ok(())
    }

    fn terse(&self, actual: &T, emit: &mut StyledEmitter) -> std::io::Result<()> {
        use Semantic::*;

        let actual = format!("{:?}", actual);

        let line = inline!("Expected " [Expected: self.pattern.value()]);
        // let line = inline!(
        //     "Expected " [Expected: self.pattern.value()] " to " [Relationship: self.relationship] " " [Actual: actual]);

        line.emit_into(emit)?;

        Ok(())
    }

    fn detailed(&self, _actual: Described<T>, _styled: &mut StyledEmitter) -> std::io::Result<()> {
        todo!()
    }

    fn clone_matcher(&self) -> Box<dyn DescribeMatcher<T>> {
        Box::new(PatternMatcher {
            pattern: self.pattern.clone(),
            relationship: self.relationship.clone(),
        })
    }
}
