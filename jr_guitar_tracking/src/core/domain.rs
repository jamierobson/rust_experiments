// todo: This should be a series of classes in their own files, at least if the prgramming paradigm is close to c# in this regard. 
// After defining the below MethodTest type, I am more convinced that a single file would be appropriate. 

use http;
//

pub struct MethodTest();
impl MethodTest {
    pub fn message(self: &Self) -> String {
        return String::from("message from Tune");
    }
}

//
pub struct Piece {
    number: Number,
    guitar_configuration: GuitarConfiguration,
    reference_material: ReferenceMaterial
}

//
pub struct GuitarConfiguration {
    tuning: Tuning,
    capo: Option::<i32>,
}

//
#[allow(non_snake_case)] // guitar convention recognises the default turning as EADGBe
pub struct Tuning {
    string_E: String,
    string_A: String,
    string_D: String,
    string_G: String,
    string_B: String,
    string_e: String,
}
//

pub struct Number {
    artist: String,
    name: String,
    album: Option<String>,
    track_number: Option<i32>
}

//
pub struct ReferenceMaterial {
    tab_link: http::Uri,
    playback_link: http::Uri,
}

// Learned:
// A class seems to be a struct with methods implemented on it, according to https://medium.com/@jimmco/classes-in-rust-c5b72c0f0a4c
// To defining a method reminds me a little of describing extension methods: passing in the first argument as (self: &Self).
// I dislike the shorthand "pub", "fn", "mod" shorthand tendencies for keywords. I hope that these are front-loaded.
// There are two means to return a string from a literal, by the seems of things. "something" seems to be a &String type.Use "something".to_string(), or better String::from("something").
// I'm guessing that the definition of MethodTest(); hints to the compiler that there are no fields on the type, or that it allows the definition of a parameterless constructor
// There's a couple of ways to use types from another file. https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project