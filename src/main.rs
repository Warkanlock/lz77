#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct LZ77Token {
    length: usize,
    distance: usize,
    char: u8,
}

struct LZ77Algorithm {}

impl LZ77Algorithm {
    fn get_distance_to_match(data: &[u8], offset: usize, end: usize) -> u8 {
        let mut offset: usize = offset;
        let mut end: usize = end;
        let mut len: u8 = 0;

        // if item are equal, we increment the offset and the position
        while offset < end && end < data.len() && data[offset] == data[end] && len < 255 {
            offset += 1;
            end += 1;
            len += 1;
        }

        return len;
    }

    fn find_match(data: &[u8], end: usize) -> (u8, u8) {
        let mut best_offset = 0u8;
        let mut best_len = 0u8;

        for offset in 0..end {
            // end in this case is the end of the data stream
            let len = LZ77Algorithm::get_distance_to_match(data, offset, end);
            if len > best_len {
                best_offset = (end - offset) as u8; // distance from index (end)
                best_len = len; // length of the match
            }
        }
        return (best_offset, best_len);
    }

    fn encode(&self, input: &str) -> Vec<LZ77Token> {
        let input_as_byte = input.as_bytes();
        let mut output: Vec<LZ77Token> = Vec::new();
        let mut buffer: Vec<u8> = Vec::new();
        let mut index: usize = 0; // do not use a for since it will iterate over the entire set of
                                  // elements

        while index < input_as_byte.len() {
            let (offset, len) = LZ77Algorithm::find_match(&input_as_byte, index);

            let mut new_token = LZ77Token {
                distance: offset as usize,
                length: 0,
                char: 0,
            };

            if offset == 0 {
                new_token.char = input_as_byte[index];
                output.push(new_token);
                index += 1;
            } else {
                new_token.length = len as usize;
                output.push(new_token);
                index += len as usize;
            }
        }

        output
    }

    fn decode(&self, data: &Vec<LZ77Token>) -> String {
        let mut output = String::new();

        for (index, token) in data.iter().enumerate() {
            if token.length == 0 {
                output.push(token.char as char);
            } else {
                let start = output.len() - token.distance;
                let end = start + token.length;

                for i in start..end {
                    let c = output.chars().nth(i).unwrap();
                    output.push(c);
                }
            }
        }

        return output;
    }

    fn new() -> LZ77Algorithm {
        LZ77Algorithm {}
    }
}

fn main() {
    let lz77 = LZ77Algorithm::new();

    // input to compress
    let argv: Vec<String> = std::env::args().collect();
    let input = argv.get(1).unwrap();

    // encode input
    let encoded_result = lz77.encode(input);
    // decode input
    let decoded_result = lz77.decode(&encoded_result);

    // check if decoded result is equal to input
    println!(
        "Encoded: {} | Decoded: {} | Input: {}",
        encoded_result.len(),
        decoded_result.len(),
        input.len()
    );
    println!("> Encoded result: {:?}", encoded_result);
    println!("> Decoded result: {:?}", decoded_result);

    println!("Encoded bytes: {}", mem::size_of_val(&encoded_result));
    println!("Decoded bytes: {}", mem::size_of_val(&decoded_result));
}

// add test
#[cfg(test)]
mod test_suite {

    use super::LZ77Algorithm;

    #[test]
    fn encode_length_three_string() {
        let lz77 = LZ77Algorithm::new();

        // input to compress
        let input = "abc";

        // encode input
        let encoded_result = lz77.encode(input);
        let decoded_result = lz77.decode(&encoded_result);

        assert_eq!(decoded_result, input);
    }

    #[test]
    fn encode_length_complex_string() {
        let lz77 = LZ77Algorithm::new();

        // input to compress
        let input = "aabbcc";

        // encode input
        let encoded_result = lz77.encode(input);
        let decoded_result = lz77.decode(&encoded_result);

        assert_eq!(decoded_result, input);
    }
}
