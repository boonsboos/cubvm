/// standard 3x3 (Rubik's) Cube
///
/// layout of the faces array is as follows:
/// U, F, R, B, L, D,
///
/// with the correct facing of the colours as defined by the scrambling rules
/// set by the World Cubing Association in WCA regulation 4d1.
///
/// reference: https://www.worldcubeassociation.org/regulations#4d1
pub struct Cube {
    /// a face is 9 bytes.
    /// layout of a face by indices:
    ///
    /// `0 3 6`
    ///
    /// `1 4 7`
    ///
    /// `2 5 8`
    faces: [[u8; 9]; 6], // u8[9][6]
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            faces: [
                [0x00; 9], // white  9 * 0
                [0x01; 9], // green  9 * 1
                [0x02; 9], // red    9 * 2
                [0x04; 9], // blue   9 * 4
                [0x08; 9], // orange 9 * 8
                [0x10; 9], // yellow 9 * 16
            ]
        }
    }

    // NOTE: before you proceed!
    // the notation describing the mutations is
    // face -> face
    // read it as "old Face becomes new other Face"
    // eg: F -> U means old F is the new U

    /// rotates the cube on the X axis.
    ///
    /// ex. if white is facing U, it will now face B
    pub fn rotate_x(&mut self) {
        self.faces = [
            self.faces[1], // F -> U
            self.faces[5], // D -> F
            self.faces[2], // R
            self.faces[0], // U -> B
            self.faces[4], // L
            self.faces[3], // B -> D
        ]
    }

    /// returns a new cube rotated on the X' axis.
    ///
    /// ex. if white is facing U, it will now face F
    pub fn rotate_x_prime(&mut self){
        self.faces = [
            self.faces[3], // B -> U
            self.faces[0], // U -> F
            self.faces[2], // R
            self.faces[5], // D -> B
            self.faces[4], // L
            self.faces[1], // F -> D
        ]
    }

    /// rotates the cube on the Y axis.
    ///
    /// ex. if green is facing F, it will now face L
    pub fn rotate_y(&mut self) {
        self.faces = [
            self.faces[0], // U
            self.faces[2], // F -> R
            self.faces[3], // R -> B
            self.faces[4], // B -> L
            self.faces[1], // L -> F
            self.faces[5], // D
        ]
    }

    /// returns a new cube rotated on the Y' axis.
    ///
    /// ex. if green is facing F, it will now face R
    pub fn rotate_y_prime(&mut self){
        self.faces = [
            self.faces[0], // U
            self.faces[4], // L -> F
            self.faces[1], // F -> R
            self.faces[2], // R -> B
            self.faces[3], // B -> L
            self.faces[5], // D
        ]
    }

    /// rotates the cube on the Z axis.
    ///
    /// ex. if white is facing F, it will now face R
    pub fn rotate_z(&mut self) {
        self.faces = [
            self.faces[0], // U
            self.faces[2], // F -> R
            self.faces[3], // R -> B
            self.faces[4], // B -> L
            self.faces[1], // L -> F
            self.faces[5], // D
        ]
    }

    /// returns a new cube rotated on the Z' axis.
    ///
    /// ex. if white is facing U, it will now face L
    pub fn rotate_z_prime(&mut self){
        self.faces = [
            self.faces[0], // U
            self.faces[4], // L -> F
            self.faces[1], // F -> R
            self.faces[2], // R -> B
            self.faces[3], // B -> L
            self.faces[5], // D
        ]
    }
}