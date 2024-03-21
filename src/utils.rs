#[macro_export]
macro_rules! to_u16_be {
    ($x:expr) => {{
        if $x.len() != 2 {
            panic!("Invalid byte length");
        }

        u16::from_be_bytes([$x[0], $x[1]])
    }};
}

#[macro_export]
macro_rules! to_u32_be {
    ($x:expr) => {{
        if $x.len() != 4 {
            panic!("Invalid byte length");
        }

        u32::from_be_bytes([$x[0], $x[1], $x[2], $x[3]])
    }};
}
