# TestError

`TestError` provides a way of encapsulating test failures that still have enough information to be presented in various different ways:

- tersely, in a single-line presentation
- in detail, with diffs
- in serialized format

`TestError` builds on `Emit` so that each error format can be emitted with style or in plain text, and to any supported `Emit` backend.