// todo: This should be a series of classes in their own files, at least if the prgramming paradigm is close to c# in this regard. 
// After defining the below MethodTest type, I am more convinced that a single file would be appropriate. 

use http;

//
pub struct PieceInRepetoire {
    piece: Piece,
    guitar_configuration: GuitarConfiguration,
    reference_material: ReferenceMaterial,
    memorized: bool
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
pub struct Piece {
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