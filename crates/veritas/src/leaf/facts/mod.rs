trait Reportable {
    type Representation;
}

trait BasicFact: Reportable {}

trait KeyValueFact: Reportable {}

trait DiffableFact: Reportable {}

trait StringFact: Fact {}
