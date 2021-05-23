/*

Functions to efficiently decode neutron comap headers

For docs see https://neutron.earlgrey.tech/spec/neutronabi

General note: Whenever it matters little-endian byte order is used, unless explicitly stated otherwise

TODO: Move to neutron-common, will at least be used in neutron-star too down the line

*/

// A long list of constants isn't pretty, the important thing is that it gets inlined by the compiler.
pub const HEADER_SIZE_MASK: u8 = 0b11000000;
pub const HEADER_SIZE_1: u8 = 0b00000000;
pub const HEADER_SIZE_2: u8 = 0b01000000;
pub const HEADER_SIZE_4: u8 = 0b10000000;
pub const HEADER_SIZE_RESERVED: u8 = 0b11000000;

// These following will be used in the full decoder

/*
const TYPE_CATEGORY_MASK: u8    = 0b00100000;
const TYPE_CATEGORY_NUMERIC: u8 = 0b00000000;
const TYPE_CATEGORY_SPECIAL: u8 = 0b00100000;

const HEX_OR_BIGNUM_MASK: u8    = 0b00010000;
const HEX_OR_BIGNUM_FALSE: u8   = 0b00000000;
const HEX_OR_BIGNUM_TRUE: u8    = 0b00010000;

const IS_ARRAY_MASK: u8         = 0b00001000;
const IS_ARRAY_FALSE: u8        = 0b00000000;
const IS_ARRAY_TRUE: u8         = 0b00001000;

const NUMERIC_TYPE_MASK: u8     = 0b00000111;
const NUMERIC_TYPE_U8: u8       = 0b00000000;
const NUMERIC_TYPE_I8: u8       = 0b00000100;
const NUMERIC_TYPE_U16: u8      = 0b00000010;
const NUMERIC_TYPE_I16: u8      = 0b00000110;
const NUMERIC_TYPE_U32: u8      = 0b00000001;
const NUMERIC_TYPE_I32: u8      = 0b00000101;
const NUMERIC_TYPE_U64: u8      = 0b00000011;
const NUMERIC_TYPE_I64: u8      = 0b00000111;

#[derive(PartialEq)]
pub enum ComapDataType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    UNDEFINED,
}

*/

// Returns: (header_size: usize, header_u32: u32)
pub fn comap_abi_header_to_u32(data: &[u8]) -> (usize, u32) {
    let mut header_size: usize = 0;
    let mut header_u32: u32 = 0;

    let first_byte = data[0];

    // Look at bit 8 and 7 of first byte to determine header size
    match first_byte & HEADER_SIZE_MASK {
        HEADER_SIZE_1 => {
            header_size = 1;
            header_u32 = u32::from_le_bytes([first_byte, 0_u8, 0_u8, 0_u8]);
        }
        HEADER_SIZE_2 => {
            header_size = 2;
            header_u32 = u32::from_le_bytes([first_byte, data[1], 0_u8, 0_u8]);
        }
        HEADER_SIZE_4 => {
            header_size = 4;
            header_u32 = u32::from_le_bytes([first_byte, data[1], data[2], data[3]]);
        }
        HEADER_SIZE_RESERVED => panic!("Reserved codata size type isn't implemented yet!"),
        _ => println!("Failed to match comap header data to a valid pattern (This should never happen)"),
    }

    (header_size, header_u32)
}

// Note: All 4 bytes from the provided u32 are always returned for simplicity. No need to explicitly clear the "extra" bytes,
// since the caller can use header_size to extract only the actual header bytes
// Returns: (header_size: usize, header_bytes: [u8;4])
pub fn comap_abi_header_from_u32(header_u32: u32) -> (usize, [u8; 4]) {
    let mut header_size: usize = 0;

    let header_bytes = header_u32.to_le_bytes();
    let first_byte = header_bytes[0];

    // Look at bit 8 and 7 of first byte to determine header size
    match first_byte & HEADER_SIZE_MASK {
        HEADER_SIZE_1 => {
            header_size = 1;
        }
        HEADER_SIZE_2 => {
            header_size = 2;
        }
        HEADER_SIZE_4 => {
            header_size = 4;
        }
        HEADER_SIZE_RESERVED => panic!("Reserved codata size type isn't implemented yet!"),
        _ => println!("Failed to match comap header data to a valid pattern (This should never happen)"),
    }

    (header_size, header_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    // These are just here to make the assignment patterns easier to compare
    const EMPTY_BYTE: u8 = 0x00;
    const EMPTY_U32_2: u32 = 0x0000_0000;
    const EMPTY_U32_3: u32 = 0x0000_0000;
    const EMPTY_U32_4: u32 = 0x0000_0000;

    const HEADER_BYTE: u8 = 0xAA;
    const HEADER_U32_2: u32 = 0x0000_AA00;
    const HEADER_U32_3: u32 = 0x00AA_0000;
    const HEADER_U32_4: u32 = 0xAA00_0000;

    const VALUE_BYTE: u8 = 0xDD;

    // Basic tests for comap_abi_header_to_u32
    #[test]
    fn test_get_header_u32_1() {
        let data: Vec<u8> = vec![HEADER_SIZE_1, VALUE_BYTE];
        let (header_size, header_u32) = comap_abi_header_to_u32(&data);
        assert_eq!(header_size, 1, "abi return value 'header_size' was wrong");
        let expected_header = (HEADER_SIZE_1 as u32) + EMPTY_U32_2 + EMPTY_U32_3 + EMPTY_U32_4;
        assert_eq!(header_u32, expected_header, "abi return value 'header_u32' was wrong");
    }
    #[test]
    fn test_get_header_u32_2() {
        let data: Vec<u8> = vec![HEADER_SIZE_2, HEADER_BYTE, VALUE_BYTE];
        let (header_size, header_u32) = comap_abi_header_to_u32(&data);
        assert_eq!(header_size, 2, "abi return value 'header_size' was wrong");
        let expected_header = (HEADER_SIZE_2 as u32) + HEADER_U32_2 + EMPTY_U32_3 + EMPTY_U32_4;
        assert_eq!(header_u32, expected_header, "abi return value 'header_u32' was wrong");
    }
    #[test]
    fn test_get_header_u32_4() {
        let data: Vec<u8> = vec![HEADER_SIZE_4, HEADER_BYTE, HEADER_BYTE, HEADER_BYTE, VALUE_BYTE];
        let (header_size, header_u32) = comap_abi_header_to_u32(&data);
        assert_eq!(header_size, 4, "abi return value 'header_size' was wrong");
        let expected_header = (HEADER_SIZE_4 as u32) + HEADER_U32_2 + HEADER_U32_3 + HEADER_U32_4;
        assert_eq!(header_u32, expected_header, "abi return value 'header_u32' was wrong");
    }
    #[test]
    #[should_panic]
    // The "reserved" data size is currently not implemented and should panic
    fn negtest_get_header_u32_reserved() {
        let data: Vec<u8> = vec![HEADER_SIZE_RESERVED, VALUE_BYTE];
        let (_header_size, _header_u32) = comap_abi_header_to_u32(&data);
    }

    // Basic tests for comap_abi_header_from_u32
    #[test]
    fn test_get_header_bytes_1() {
        let data: u32 = (HEADER_SIZE_1 as u32) + EMPTY_U32_2 + EMPTY_U32_3 + EMPTY_U32_4;
        let (header_size, header_bytes) = comap_abi_header_from_u32(data);
        assert_eq!(header_size, 1, "\nReturn value 'header_size' was wrong");
        assert_eq!(
            header_bytes,
            [HEADER_SIZE_1, EMPTY_BYTE, EMPTY_BYTE, EMPTY_BYTE],
            "\nReturn value 'header_bytes' was wrong"
        );
    }
    #[test]
    fn test_get_header_bytes_2() {
        let data: u32 = (HEADER_SIZE_2 as u32) + HEADER_U32_2 + EMPTY_U32_3 + EMPTY_U32_4;
        let (header_size, header_bytes) = comap_abi_header_from_u32(data);
        assert_eq!(header_size, 2, "\nReturn value 'header_size' was wrong");
        assert_eq!(
            header_bytes,
            [HEADER_SIZE_2, HEADER_BYTE, EMPTY_BYTE, EMPTY_BYTE],
            "\nReturn value 'header_bytes' was wrong"
        );
    }
    #[test]
    fn test_get_header_bytes_4() {
        let data: u32 = (HEADER_SIZE_4 as u32) + HEADER_U32_2 + HEADER_U32_3 + HEADER_U32_4;
        let (header_size, header_bytes) = comap_abi_header_from_u32(data);
        assert_eq!(header_size, 4, "\nReturn value 'header_size' was wrong");
        assert_eq!(
            header_bytes,
            [HEADER_SIZE_4, HEADER_BYTE, HEADER_BYTE, HEADER_BYTE],
            "\nReturn value 'header_bytes' was wrong"
        );
    }
    #[test]
    #[should_panic]
    // The "reserved" data size is currently not implemented and should panic
    fn negtest_get_header_bytes_4() {
        let data: u32 = HEADER_SIZE_RESERVED as u32;
        let (_header_size, _header_bytes) = comap_abi_header_from_u32(data);
    }
}
