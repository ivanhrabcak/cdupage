Thank you for investing your time in contributing to my project! I really value your work and help.

# Code - general rules
I try to write code as cleanly as possible, I think readability is the most important thing when wrtiting code.
 
- Functionality that is provided to the user is in the form of a trait (a file in the `traits` module) that is implemented for the `Edupage` struct
- Use the `Edupage::request` method if possible when requesting data from edupage instead of using the client directly
- Try not to use `Map<String, Value>` - create types for json that is returned from edupage
- Use the `EdupageError` error type for all errors
- Do not use `.unwrap()` or code that can crash unless it is certain that it will not crash (check with `is_ok`/`is_some`/`if let`)
- ... and others, you can read the code for more information, or read the next section that describes the structure of this project, which also includes additional instructions.

# Code - structure
In the following section, I am providing a guide to the structure of my code and some instructions for implementing new features.

### `cdupage::edupage`
Contains the `Edupage` struct. This is the struct that is used by the users of this library to interact with the library.
 
Also contains the `EdupageError` enum, which is used for all errors - most methods of the `Edupage` struct should return `Result<_, EdupageError>`

### `cdupage::traits`
All new functionality is added as a trait, which is then implemented for the `Edupage` struct.
The trait and its implementation and relevant code should be in the same file. Please see other trait implementations for examples.

### `cdupage::types`
When implementing new features that require interaction with edupage, try to not use raw json (`Map<String, Value>`) when parsing data, but give the responses types.

For now, the project only has support for node.js bindings, which need some additional code for each type that is exposed to the users (__only__ types that are exposed for the users):

- Each type has to implement `serde::Serialize` and `serde::Deserialize`
- Each type has to have the following attributes: `#[derive(TS)]`, `#[ts(export)]` and `#[ts(rename_all = "camelCase")]` when the `node-types` feature is enabled. Here is a helpful snippet:
```rust
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
```
- Each attribute that uses `serde(rename = "...")` has to be guarded and only rename when the `node-types` attribute is NOT enabled. Here is a helpful snippet:
```rust
#[cfg_attr(not(feature = "node-types"), serde(rename = "starttime"))]
pub start_time: NaiveDateTime
```
- Any file that exposes types to users has to import `ts_rs::TS` when the `node-types` feature is enabled. Here is a helpful snippet:
```rust
#[cfg(feature = "node-types")]
use ts_rs::TS;
```

### `edupage::deserializers`
Because of how dynamic some of the types in edupage's responses are, sometimes it is required to write custom serialization and deserialization code.

When implementing custom (de)serializers, it is recommended to use `mod` blocks with the following structure:
```rust
// This custom serializer code is used to parse edupage's ids, which are in the json as strings.
// Because we want ids as `i64` in the code, we have to implement a custom serializer that checks
// if the id is not null and convert the string to i64.
pub mod string_i64_option {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(id: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if id.is_none() {
            serializer.serialize_none()
        } else {
            serializer.serialize_i64(id.unwrap())
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {

        let s: Option<String> = match Deserialize::deserialize(deserializer) {
            Ok(x) => x,
            
            // missing field? we want to return None, not fail.
            Err(_) => return Ok(None),
        };

        // the id is `null` in the json
        if s.is_none() {
            return Ok(None);
        }

        let s = &s.unwrap();

        if s.is_empty() {
            return Ok(None);
        }
        
        // convert to i64
        match s.parse() {
            Ok(n) => Ok(Some(n)),
            Err(_) => Ok(None),
        }
    }
}
```

You can then use it in your types:
```rust
#[serde(with = string_i64_option)]
pub id: Option<i64>
```

### `cdupage::nodes`
Boilerplate code that is used to support the generation of node types and bindings.
