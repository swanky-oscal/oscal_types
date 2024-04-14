# OSCAL Types
OSCAL metaschema data types for Rust.
This lib implements the metaschema data type defined in [Metaschema](https://github.com/usnistgov/metaschema).

For guidance on data types, read [Data Types Used in Metaschema](https://pages.nist.gov/metaschema/specification/datatypes/)
## URI
URIDataType and URIReferenceDatatype leverage [fluent-uri](https://docs.rs/fluent_uri) for validation.

## Dates
The date based types leverage [chrono](https://docs.rs/chrono) for validation.