// Mul0 Hashing Algorithm
// =======================================
// https://github.com/mealet/mul0
// Project licensed under the MIT License.
// See more in LICENSE file.

const KEY_POSITION: usize = 0;
const KEY_PLACE_END: bool = true;

pub struct Mul0;

impl Mul0 {
    pub fn hash(input: String) -> String {
        let input_bytes: Vec<u8> = input.bytes().collect();
        let key_byte = input_bytes[KEY_POSITION];
        let output_bytes: Vec<u16> = input_bytes
            .iter()
            .map(|&byte| byte as u16 * key_byte as u16)
            .collect();

        let output_string: String = output_bytes
            .iter()
            .map(|&number| {
                let hex_number = format!("{:x}", number);
                let mut hex_chars = hex_number.chars().collect::<Vec<char>>();

                let _ = hex_chars.reverse();
                let _ = hex_chars.resize(4, '0');
                let _ = hex_chars.reverse();

                return hex_chars.iter().collect::<String>();
            })
            .collect::<String>();

        let mut key_fmt = key_byte.to_string().chars().collect::<Vec<char>>();

        let _ = key_fmt.reverse();
        let _ = key_fmt.resize(4, '0');
        let _ = key_fmt.reverse();

        return if KEY_PLACE_END {
            format!("{}{}", output_string, key_fmt.iter().collect::<String>())
        } else {
            format!("{}{}", key_fmt.iter().collect::<String>(), output_string)
        };
    }

    pub fn dehash(input: String) -> String {
        let mut input_chars = input.chars().collect::<Vec<char>>();
        let mut _key = String::new();

        (_key, input_chars) = if KEY_PLACE_END {
            (
                input_chars.iter().rev().take(4).rev().collect::<String>(),
                input_chars[0..input_chars.len() - 4].to_vec(),
            )
        } else {
            (
                input_chars.iter().take(4).collect::<String>(),
                input_chars[4..input_chars.len()].to_vec(),
            )
        };

        let chunks = input_chars
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();

        let key_number = _key
            .parse::<u16>()
            .expect("An error occured while parsing key!");
        let calculated_chars: Vec<u8> = chunks
            .iter()
            .map(|chunk| {
                let number = u16::from_str_radix(chunk.as_str(), 16)
                    .expect(format!("Cannot parse chunk: {}", chunk).as_str());
                return (number / key_number) as u8;
            })
            .collect();

        return String::from_utf8_lossy(&calculated_chars).to_string();
    }
}
