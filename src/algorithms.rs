const KEY_POSITION: usize = 0;
const KEY_PLACE_END: bool = true;

pub struct Mul0;

impl Mul0 {
    pub fn hash(input: String) -> String {
        let bytes: Vec<u8> = input.bytes().collect();
        let key_byte = bytes[KEY_POSITION];
        let mut output_bytes: Vec<u16> = Vec::new();

        for byte in bytes {
            output_bytes.push(byte as u16 * key_byte as u16)
        }

        let output: String = output_bytes
            .iter()
            .map(|&number| {
                let hex_number = format!("{:x}", number);
                let mut hex_chars = hex_number.chars().collect::<Vec<char>>();

                let _ = hex_chars.reverse();
                let _ = hex_chars.resize(4, '0');
                let _ = hex_chars.reverse();

                return hex_chars.iter().collect::<String>();
            })
            .collect();

        let mut key_fmt = key_byte.to_string().chars().collect::<Vec<char>>();

        let _ = key_fmt.reverse();
        let _ = key_fmt.resize(4, '0');
        let _ = key_fmt.reverse();

        if KEY_PLACE_END {
            return format!("{}{}", output, key_fmt.iter().collect::<String>());
        }

        return format!("{}{}", key_fmt.iter().collect::<String>(), output);
    }

    pub fn dehash(input: String) -> String {
        let mut input_chars = input.chars().collect::<Vec<char>>();
        let mut _key = String::new();

        if KEY_PLACE_END {
            _key = input_chars.iter().rev().take(4).rev().collect::<String>();
            input_chars = input_chars[0..input_chars.len() - 4].to_vec();
        } else {
            _key = input_chars.iter().take(4).collect::<String>();
            input_chars = input_chars[4..input_chars.len()].to_vec();
        }

        let chunks = input_chars
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();

        let mut calculated_chars: Vec<u8> = Vec::new();
        let key_number = _key
            .parse::<u16>()
            .expect("An error occured while parsing key!");

        for chunk in chunks {
            let number = u16::from_str_radix(chunk.as_str(), 16)
                .expect(format!("Cannot parse chunk: {}", chunk).as_str());
            let result = number / key_number;

            calculated_chars.push(result as u8);
        }

        return String::from_utf8_lossy(&calculated_chars).to_string();
    }
}
