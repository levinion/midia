pub fn number2vlv(number: u16) -> Vec<u8> {
    let n = number / 128;
    if n == 0 {
        return vec![number as u8];
    }
    let m = number % 128;
    vec![0x80 + n as u8, m as u8]
}

#[cfg(test)]
mod test {
    use crate::util::number2vlv;

    #[test]
    fn test_delta_time_to_bytes() {
        assert_eq!(number2vlv(128), vec![0x81, 0x00]);
    }
}
