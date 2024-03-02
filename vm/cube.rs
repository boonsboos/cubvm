/// standard 3x3 (Rubik's) Cube
///
/// layout of the faces array is as follows:
/// U, F, R, B, L, D,
///
/// with the correct facing of the colours as defined by the scrambling rules
/// set by the World Cubing Association in WCA regulation 4d1.
///
/// reference: https://www.worldcubeassociation.org/regulations#4d1
const U: usize = 0;
const F: usize = 1;
const R: usize = 2;
const B: usize = 3;
const L: usize = 4;
const D: usize = 5;

#[derive(Debug)]
pub struct Cube {
    /// a face is 9 bytes.
    /// layout of a face by indices:
    ///
    /// `0 3 6`
    ///
    /// `1 4 7`
    ///
    /// `2 5 8`
    /// Where 0 is top left, and 8 is bottom right.
    faces: [[u8; 9]; 6], // u8[9][6]
}

/// rotates a face 90 degrees clockwise
fn rotate_face(r: &mut[u8; 9]) {
    *r = [r[2], r[5], r[8], r[1], r[4], r[7], r[0], r[3], r[6]];
}

/// rotates a face 90 degrees counter clockwise
fn rotate_face_prime(r: &mut[u8; 9]) {
    *r = [r[6], r[3], r[0], r[7], r[4], r[1], r[8], r[5], r[2]];
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
    // X -> Y
    // read it as "old Face becomes new other Face"
    // eg: F -> U means old F is the new U

    /// rotates the cube on the X axis.
    ///
    /// ex. if white is facing U, it will now face B
    pub fn rotate_x(&mut self) {
        
        rotate_face(&mut self.faces[R]);
        rotate_face_prime(&mut self.faces[L]);
 
        self.faces = [
            self.faces[F], // F -> U
            self.faces[D], // D -> F
            self.faces[R], // R
            self.faces[U], // U -> B
            self.faces[L], // L
            self.faces[B], // B -> D
        ]
    }

    /// rotates the cube on the X' axis.
    ///
    /// ex. if white is facing U, it will now face F
    pub fn rotate_x_prime(&mut self) {
        rotate_face_prime(&mut self.faces[R]);
        rotate_face(&mut self.faces[L]);
 
        self.faces = [
            self.faces[B], // B -> U
            self.faces[U], // U -> F
            self.faces[R], // R
            self.faces[D], // D -> B
            self.faces[L], // L
            self.faces[F], // F -> D
        ]
    }

    /// rotates the cube twice on the X axis.
    ///
    /// ex. if white is facing U, it will now face D
    pub fn rotate_x2(&mut self) {        
        self.faces[R].reverse();
        self.faces[L].reverse();
 
        self.faces = [
            self.faces[D], // D -> U
            self.faces[B], // B -> F
            self.faces[R], // R
            self.faces[F], // F -> B
            self.faces[L], // L
            self.faces[U], // U -> D
        ]
    }

    /// rotates the cube on the Y axis.
    ///
    /// ex. if green is facing F, it will now face L
    pub fn rotate_y(&mut self) {
        rotate_face(&mut self.faces[U]);
        rotate_face_prime(&mut self.faces[D]);

        self.faces = [
            self.faces[U], // U
            self.faces[R], // R -> F
            self.faces[B], // B -> R
            self.faces[L], // L -> B
            self.faces[F], // F -> L
            self.faces[D], // D
        ]
    }

    /// rotates the cube twice on the Y' axis.
    ///
    /// ex. if green is facing F, it will now face R
    pub fn rotate_y_prime(&mut self) {
        rotate_face_prime(&mut self.faces[U]);
        rotate_face(&mut self.faces[D]);

        self.faces = [
            self.faces[U], // U
            self.faces[L], // L -> F
            self.faces[F], // F -> R
            self.faces[R], // R -> B
            self.faces[B], // B -> L
            self.faces[D], // D
        ]
    }

    /// rotates the cube twice on the Y axis.
    ///
    /// ex. if green is facing F, it will now face B
    pub fn rotate_y2(&mut self) {        
        self.faces[U].reverse();
        self.faces[D].reverse();
 
        self.faces = [
            self.faces[U], // U
            self.faces[B], // B -> F
            self.faces[L], // L -> R
            self.faces[F], // F -> B
            self.faces[R], // R -> L
            self.faces[D], // D
        ]
    }

    /// rotates the cube on the Z axis.
    ///
    /// ex. if white is facing U, it will now face R
    pub fn rotate_z(&mut self) {
        rotate_face(&mut self.faces[F]);
        rotate_face_prime(&mut self.faces[B]);

        self.faces = [
            self.faces[L], // L -> U
            self.faces[F], // F
            self.faces[U], // U -> R
            self.faces[B], // B
            self.faces[D], // D -> L
            self.faces[R], // R -> D
        ]
    }

    /// rotates the cube on the Z' axis.
    ///
    /// ex. if white is facing U, it will now face L
    pub fn rotate_z_prime(&mut self) {
        
        rotate_face_prime(&mut self.faces[F]);
        rotate_face(&mut self.faces[B]);

        // change facings
        self.faces = [
            self.faces[R], // R -> U
            self.faces[F], // F
            self.faces[D], // D -> R
            self.faces[B], // B
            self.faces[U], // U -> L
            self.faces[L], // L -> D
        ]
    }

    /// rotates the cube twice on the Z axis.
    ///
    /// ex. if white is facing U, it will now face D
    pub fn rotate_z2(&mut self) {        
        self.faces[F].reverse();
        self.faces[B].reverse();
 
        self.faces = [
            self.faces[D], // D -> U
            self.faces[F], // F
            self.faces[L], // L -> R
            self.faces[B], // B
            self.faces[R], // R -> L
            self.faces[U], // U -> D
        ]
    }

    /*
     * Dear traveller, before you enter this section,
     * just know, this section makes no sense unless you use a physical cube.
     * I could probably explain it someday.
     */

    /// twists the U side clockwise (U)
    pub fn twist_u(&mut self) {
        let b = self.faces[B];
        let l = self.faces[L];
        let f = self.faces[F];
        let r = self.faces[R];
        
        rotate_face(&mut self.faces[U]);
        
        self.faces[B] = [l[0], b[1], b[2], l[3], b[4], b[5], l[6], b[7], b[8]];
        self.faces[F] = [r[0], f[1], f[2], r[3], f[4], f[5], r[6], f[7], f[8]];
        self.faces[L] = [f[0], l[1], l[2], f[3], l[4], l[5], f[6], l[7], l[8]];
        self.faces[R] = [b[0], r[1], r[2], b[3], r[4], r[5], b[6], r[7], r[8]];
    }

    /// twists the U side counter clockwise (U')
    pub fn twist_u_prime(&mut self) {
        let b = self.faces[B];
        let l = self.faces[L];
        let f = self.faces[F];
        let r = self.faces[R];
        
        rotate_face_prime(&mut self.faces[U]);
        
        self.faces[B] = [r[0], b[1], b[2], r[3], b[4], b[5], r[6], b[7], b[8]];
        self.faces[F] = [l[0], f[1], f[2], l[3], f[4], f[5], l[6], f[7], f[8]];
        self.faces[L] = [b[0], l[1], l[2], b[3], l[4], l[5], b[6], l[7], l[8]];
        self.faces[R] = [f[0], r[1], r[2], f[3], r[4], r[5], f[6], r[7], r[8]];
    }

    /// twists the U side twice (U2)
    pub fn twist_u2(&mut self) {
        let b = self.faces[B];
        let l = self.faces[L];
        let f = self.faces[F];
        let r = self.faces[R];

        self.faces[U].reverse();

        self.faces[B] = [f[0], b[1], b[2], f[3], b[4], b[5], f[6], b[7], b[8]];
        self.faces[F] = [b[0], f[1], f[2], b[3], f[4], f[5], b[6], f[7], f[8]];
        self.faces[L] = [r[0], l[1], l[2], r[3], l[4], l[5], r[6], l[7], l[8]];
        self.faces[R] = [l[0], r[1], r[2], l[3], r[4], r[5], l[6], r[7], r[8]];
    }

    /// twists the F side clockwise (F)
    pub fn twist_f(&mut self) {
        let u = self.faces[U];
        let d = self.faces[D];
        let l = self.faces[L];
        let r = self.faces[R];

        rotate_face(&mut self.faces[F]);

        self.faces[R] = [u[2], u[5], u[8], r[3], r[4], r[5], r[6], r[7], r[8]];
        self.faces[L] = [l[0], l[1], l[2], l[3], l[4], l[5], d[0], d[3], d[6]];
        self.faces[U] = [u[0], u[1], l[8], u[3], u[4], l[7], u[6], u[7], l[6]];
        self.faces[D] = [r[0], d[1], d[2], r[1], d[4], d[5], r[2], d[7], d[8]];
    }

    /// twists the F side counter-clockwise (F')
    pub fn twist_f_prime(&mut self) {
        let u = self.faces[U];
        let d = self.faces[D];
        let l = self.faces[L];
        let r = self.faces[R];

        rotate_face_prime(&mut self.faces[F]);

        self.faces[R] = [d[0], d[3], d[6], r[3], r[4], r[5], r[6], r[7], r[8]];
        self.faces[L] = [l[0], l[1], l[2], l[3], l[4], l[5], d[2], d[5], d[8]];
        self.faces[U] = [u[0], u[1], r[0], u[3], u[4], r[1], u[6], u[7], r[2]];
        self.faces[D] = [d[0], d[1], l[6], d[3], d[4], l[7], d[6], d[7], l[8]];
    }

    /// twists the F side twice (F2)
    pub fn twist_f2(&mut self) {
        let u = self.faces[U];
        let d = self.faces[D];
        let l = self.faces[L];
        let r = self.faces[R];

        self.faces[F].reverse();

        self.faces[R] = [l[8], l[7], l[6], r[3], r[4], r[5], r[6], r[7], r[8]];
        self.faces[L] = [l[0], l[1], l[2], l[3], l[4], r[5], r[2], l[1], l[0]];
        self.faces[U] = [u[0], u[1], d[0], u[3], u[4], d[3], u[6], u[7], d[6]];
        self.faces[D] = [u[8], d[1], d[2], u[5], d[4], d[5], u[2], d[7], d[8]];
    }

    /// twists the R side clockwise (R)
    pub fn twist_r(&mut self) {
        let b = self.faces[B];
        let u = self.faces[U];
        let f = self.faces[F];
        let d = self.faces[D];
        
        rotate_face(&mut self.faces[R]);

        self.faces[F] = [f[0], f[1], f[2], f[3], f[4], f[5], d[6], d[7], d[8]];
        self.faces[U] = [u[0], u[1], u[2], u[3], u[4], u[5], f[6], f[7], f[8]];
        self.faces[B] = [u[6], u[7], u[8], b[3], b[4], b[5], b[6], b[7], b[8]];
        self.faces[D] = [d[0], d[1], d[2], d[3], d[4], d[5], b[0], b[1], b[2]];
    }

    /// twists the R side counter clockwise (R')
    pub fn twist_r_prime(&mut self) {
        let b = self.faces[B];
        let u = self.faces[U];
        let f = self.faces[F];
        let d = self.faces[D];
        
        rotate_face_prime(&mut self.faces[R]);

        self.faces[F] = [f[0], f[1], f[2], f[3], f[4], f[5], u[6], u[7], u[8]];
        self.faces[B] = [d[6], d[7], d[8], b[3], b[4], b[5], b[6], b[7], b[8]];
        self.faces[D] = [d[0], d[1], d[2], d[3], d[4], d[5], f[6], f[7], f[8]];
        self.faces[U] = [u[0], u[1], u[2], u[3], u[4], u[5], b[0], b[1], b[2]];
    }

    /// twists the R side twice (R2)
    pub fn twist_r2(&mut self) {
        let b = self.faces[B];
        let u = self.faces[U];
        let f = self.faces[F];
        let d = self.faces[D];
        
        // rotate
        self.faces[R].reverse();
        
        self.faces[D] = [d[0], d[1], d[2], d[3], d[4], d[5], u[6], u[7], u[8]];
        self.faces[U] = [u[0], u[1], u[2], u[3], u[4], u[5], d[0], d[1], d[2]];
        self.faces[F] = [f[0], f[1], f[2], f[3], f[4], f[5], b[0], u[1], u[2]];
        self.faces[B] = [f[6], f[7], f[8], b[3], b[4], b[5], b[6], b[7], b[8]];
    }

    /// twists the B side clockwise (B)
    pub fn twist_b(&mut self) {
        let u = self.faces[U];
        let d = self.faces[D];
        let l = self.faces[L];
        let r = self.faces[R];

        rotate_face(&mut self.faces[B]);

        self.faces[D] = [d[0], d[1], l[0], d[3], d[4], l[1], d[6], d[7], l[2]];
        self.faces[U] = [r[6], u[1], u[2], r[7], u[4], u[5], r[8], u[7], u[8]];
        self.faces[L] = [u[6], u[3], u[0], l[3], l[4], l[5], l[6], l[7], l[8]];
        self.faces[R] = [r[0], r[1], r[2], r[3], r[4], r[5], d[8], d[5], d[2]];
    }

    /// twists the B side counter clockwise (B')
    pub fn twist_b_prime(&mut self) {
        let u = self.faces[U];
        let d = self.faces[D];
        let l = self.faces[L];
        let r = self.faces[R];

        rotate_face_prime(&mut self.faces[B]);

        self.faces[D] = [d[0], d[1], r[8], d[3], d[4], r[7], d[6], d[7], r[6]];
        self.faces[U] = [l[2], u[1], u[2], l[1], u[4], u[5], l[0], u[7], u[8]];
        self.faces[L] = [d[2], d[5], d[8], l[3], l[4], l[5], l[6], l[7], l[8]];
        self.faces[R] = [r[0], r[1], r[2], r[3], r[4], r[5], u[0], u[2], u[5]];
    }

    /// twists the B side twice (B2)
    pub fn twist_b2(&mut self) {
        let u = self.faces[U];
        let d = self.faces[D];
        let l = self.faces[L];
        let r = self.faces[R];

        self.faces[B].reverse();

        self.faces[D] = [d[0], d[1], u[0], d[3], d[4], u[3], d[6], d[7], u[6]];
        self.faces[U] = [d[6], u[1], u[2], d[3], u[4], u[5], d[0], u[7], u[8]];
        self.faces[L] = [r[8], r[7], r[6], l[3], l[4], l[5], l[6], l[7], l[8]];
        self.faces[R] = [r[0], r[1], r[2], r[3], r[4], r[5], l[2], l[1], l[0]];
    }

    pub fn twist_l(&mut self) {
        let b = self.faces[B];
        let u = self.faces[U];
        let f = self.faces[F];
        let d = self.faces[D];

        rotate_face(&mut self.faces[L]);
    }

    pub fn twist_l_prime(&mut self) {
        let b = self.faces[B];
        let u = self.faces[U];
        let f = self.faces[F];
        let d = self.faces[D];

        rotate_face_prime(&mut self.faces[L]);
    }

    pub fn twist_l2(&mut self) {
        let b = self.faces[B];
        let u = self.faces[U];
        let f = self.faces[F];
        let d = self.faces[D];

        self.faces[L].reverse();
    }

    pub fn twist_d(&mut self) {
        let b = self.faces[B];
        let l = self.faces[L];
        let f = self.faces[F];
        let r = self.faces[R];
        
        rotate_face(&mut self.faces[D]);
    }

    pub fn twist_d_prime(&mut self) {
        let b = self.faces[B];
        let l = self.faces[L];
        let f = self.faces[F];
        let r = self.faces[R];
        
        rotate_face_prime(&mut self.faces[D]);
    }

    pub fn twist_d2(&mut self) {
        let b = self.faces[B];
        let l = self.faces[L];
        let f = self.faces[F];
        let r = self.faces[R];
        
        self.faces[D].reverse();
    }
}