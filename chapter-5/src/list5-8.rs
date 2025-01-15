fn main() {
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let _exponent = n_bits >> 23;
    let _exponent = _exponent & 0xff;
    let exponent = _exponent - 127;
}
