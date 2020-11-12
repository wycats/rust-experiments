# Language

The concepts in Veritas are heavily inspired by Google's [truth][truth] library.

# Leaves

## Leaf Correspondence

A leaf correspondence determines whether a value `T` corresponds in some way to a `Pattern`.

There are two main kinds of correspondences:

- Exact: The `T` precisely corresponds to the `Pattern`. `Equal` and `Matches` are examples of Exact correspondences.
- Tolerance: The `T` corresponds to the `Pattern`, within some tolerance. `CloseTo` is an example of a Tolerance correspondence.

## Facts

A `Fact` is a key-value pair that can be presented to the user as a part of the explanation for a mismatch.

There are several built-in facts which are designed to help correspondence implementations present useful error messages for the supported reporters.

- Basic: a single string. For example, when the `Present` correspondence fails, it produces a Basic fact that renders as "is not present".
- Key-Value: a static key describing the fact and a value describing the fact in terms of the concrete `T`
- Diffable String: a string representation of the `T` and a string representation of the expected `T` that should be presented as a rich diff.

## Leaf Mismatches

If a `Correspondence` is not satisfied, it produces a `Mismatch`. A `Mismatch` contains a number of `Fact`s that, together, describe the reason for the mismatch.

A `Mismatch` `M2` can depend on another `Mismatch` `M1`, which means that `M2` doesn't make sense unless `M1` is fixed. Normal reporters will likely only present mismatches without dependencies, while verbose reporters could present all mismatches.

## Compound Mismatches

### Strings

Unlike many other expectation systems, mismatches in `String`s are modelled as compound mismatches in Veritas. Each `Fact` about a `String` specifies a `Range` into the `String` that it refers to.

Strings have a number of special kinds of `Fact`s:

- Ranged Basic: Like a `Basic` fact, but applied to a specific range in the string
- Ranged Key-Value: Like a `Key-Value` fact, but applied to a specific range in the string

Correspondences about `String` values can use the `Diffable String` fact to present differences between expected and actual values.

The collection of all `Fact`s about a `String` creates a set of all problematic ranges in the source string.

### Lists

### Records

### Maps

### Sets

## Localization

# Custom Correspondences

```rust

```

# Inspired By

This library is inspired by [rspec][rspec], [truth][truth], and many other test frameworks in many languages focusing on producing good error output.

[truth]: https://truth.dev/