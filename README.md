# Attri  ✏️

A tiny rust crate for creating strongly-typed information, without specifying the information at compile time.

```rust
use attri::Attributes;

#[derive(Default)]
pub struct ImportantThing {
    important_number: i32, // Important!!
    attributes: Attributes,
}

#[derive(Debug, PartialEq, Clone)] 
pub struct ImportanceQuaifier(&'static str);
#[derive(Debug, PartialEq, Clone)] 
pub struct ImportanceLevel(f32);

let mut important_thing = ImportantThing::default();

important_thing.attributes.insert("now, this is VERY important.");
important_thing.attributes.insert(ImportanceQuaifier("immenesly"));
important_thing.attributes.insert(ImportanceLevel(450.43));

assert_eq!(
    important_thing.attributes.get::<ImportanceLevel>().unwrap(), 
    &ImportanceLevel(450.43)
);
assert_eq!(
    important_thing.attributes.get::<&'static str>().unwrap(),
    &"now, this is VERY important."
);
```

This is useful if you are creating an api where a user can modify some value, send it back into your code, might eventually see it come out of some other function, and the user needs to be aware of some information about that value.

## Notes 
* Forcing each of your attributes to implement `Debug` is optional. (set `default-features` to `false`)
* The attributes struct does *NOT* implement `Clone`, even if the objects you place into it implement `Clone`.
