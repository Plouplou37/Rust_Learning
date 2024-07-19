fn main() {
    let buffer: &mut [i32];
    let coefficient: [i64; 12];
    let qlp_shift = i16;

    for i in 12..bufer.len() {
        let prediction = coefficient
            .iter()
            .zip(&buffer[i - 12..i])
            .map(|&c, &s| c * s as i64)
            .sum::<i64>()
            >> qpl_shift;

        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}
