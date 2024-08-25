/// A `DocComment`, representing a documentation comment in the source code.
///
/// # Overview
///
/// This struct is used to encapsulate a documentation comment, typically
/// used to describe the functionality, usage, or behavior of code elements
/// such as functions, classes, typealiases, or properties.
///
/// Documentation comments in Pkl start with `///` and
/// are essential for generating documentation.
///
/// # Fields
///
/// * `String` - Stores the content of the documentation comment as a `String`.
#[derive(Debug, Clone, PartialEq)]
pub struct DocComment(String);
