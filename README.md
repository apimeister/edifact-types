# Edifact-types

[![Latest Version](https://img.shields.io/crates/v/edifact-types.svg)](https://crates.io/crates/edifact-types)

This library provides bindings for the Edifact standard.

## Usage

The types should be used inconjuction with a Edifact serializer.

We do recommend the `serde_edifact` crate, since we are using this serializer for testing.

So far, there is also no validation attached to the structs.

### features

* logging: enables [log](https://crates.io/crates/log) library

### caveats

* repetition limitations are not implemented yet, we only differ
  * Mandatory (1), Optional (0/1), Vector (0-infinity)
* Not all lists are implemented for the types, validation is incomplete
* UNA, for changing control chars is ignored
* Grouped Messages with UNG/UNE are not implemented yet

## Supported Bindings

* d95b
  * coprar
* d00b
  * copran
  * iftmin
  * iftsta

Something missing? Please open an issue.

## Contributions

Since the Edifact is fairly huge, we only implement types on demand. So if you are missing some types, please open an issue or merge request.

### new type

This is only usable by copiing the content from UNECE, it will not work with anything else!

1. go to `src/util/mod.rs` and find `fs::write`, comment in all the lines
2. find your type on the UNECE Website
3. ctrl+a on the website and paste into `edi_desc/<version>/<your-type>`
4. run `cargo nextest run desc`
5. see new file under `src/<version>/message/`
