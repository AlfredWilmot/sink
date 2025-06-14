use colored::Colorize;

///  bit-pattern of the three components encoded into the f32 type:
///
/// [] [<----- EXPONENT_MASK ---->] [<---- MANTISSA_MASK ---->]
/// 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 .. 0
/// ^
/// (sign)
///
/// (assumes val is BigEndian)
///
pub struct DeconstructedFloat32<'a> {
    // reference to the original float this deconstruction is based on.
    float: &'a f32,

    // IEEE 754-XXXX standards define:
    //
    // RADIX  = 2 (base)
    // BIAS = 127 (exponent offset)
    //
    sign_bit: u8,
    exponent_byte: u8,
    mantissa_bytes: [u8; 3],
}

impl<'a> DeconstructedFloat32<'a> {
    /// create a deconstructed float from an input f32
    pub fn new(val: &'a f32) -> DeconstructedFloat32<'a> {
        // convert the input to u32 for bit-manipuation
        let bits: u32 = val.to_bits();

        // define some masks
        const SIGN_MASK: u32 = 0b10000000_00000000_00000000_00000000; // sign-bit
        const EXPO_MASK: u32 = 0b01111111_10000000_00000000_00000000; // exponent-byte
        const MANT_MASK: u32 = 0b00000000_01111111_11111111_11111111; // mantissa-bytes

        // apply masks to bits and shift to extract relevant bytes for each component:
        // (NOTE: masking sign_bit is redundant (it's the MSB) but done for consistency)
        let sign_bit = ((bits & SIGN_MASK) >> 31) as u8;
        let exponent_byte = ((bits & EXPO_MASK) >> 23) as u8;
        let mantissa_bytes = [
            (bits & MANT_MASK) >> 16,
            (bits & MANT_MASK) >> 8,
            (bits & MANT_MASK),
        ]
        .map(|v| v as u8);

        DeconstructedFloat32 {
            float: val,
            sign_bit,
            exponent_byte,
            mantissa_bytes,
        }
    }

    /// display the contents of the deconstructed float.
    pub fn print(&self) {
        let sign_bit_txt = format!("{:b}", self.sign_bit).on_red();
        let exponent_txt = format!("{:08b}", self.exponent_byte).on_red();

        let m_ = self.mantissa_bytes;
        let mantissa_txt = format!("{:07b}{:08b}{:08b}", m_[0], m_[1], m_[2]).on_red();

        println!("\nInput: {:?}\n", self.float);
        println!("| input (bits) | {:032b} |", self.float.to_bits());
        println!("| sign         | {}{:031b} |", sign_bit_txt, 0);
        println!("| exponent     | {:01b}{}{:023b} |", 0, exponent_txt, 0);
        println!("| mantissa     | {:09b}{} |", 0, mantissa_txt);
        println!();
    }
}
