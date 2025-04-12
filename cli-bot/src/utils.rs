pub fn generate_uuid_v4() -> String {
    use std::fmt::Write as _;

    let bytes = generate_uuid_v4_bytes();
    let mut str = String::with_capacity(36);

    for (i, b) in bytes.into_iter().enumerate() {
        if matches!(i, 4 | 6 | 8 | 10) {
            str.push('-');
        }

        write!(str, "{b:02x}").unwrap();
    }

    str
}

fn generate_uuid_v4_bytes() -> [u8; 16] {
    let mut bytes = [0u8; 16];
    fastrand::fill(&mut bytes);

    bytes[6] = (bytes[7] & 0x0F) | 0x40;
    bytes[8] = (bytes[9] & 0x0F) | 0x80;

    bytes
}
