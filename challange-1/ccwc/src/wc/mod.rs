use std::{error::Error, fs::File, io::Read};

pub struct Counts {
    pub byte_count: u64,
    pub char_count: u64,
    pub word_count: u64,
    pub line_count: u64,
}

pub fn process_file(filepath: String) -> Result<Counts, Box<dyn Error>> {
    let file_open_result = File::open(filepath);
    if file_open_result.is_err() {
        return Err(Box::new(file_open_result.err().unwrap()));
    }
    let result = process_reader(&file_open_result.unwrap());
    result
}

pub fn process_reader<R: Read>(mut reader: R) -> Result<Counts, Box<dyn Error>> {
    const BUFFER_SIZE: usize = 1024;
    let mut buffer: [u8; 1024] = [0; BUFFER_SIZE];
    let mut line_count: u64 = 0;
    let mut word_count: u64 = 0;
    let mut char_count: u64 = 0;
    let mut byte_count: u64 = 0;

    // let mut is_previous_byte_whitespace: bool = true;
    let mut previous_byte: u8 = b' ';
    loop {
        let file_read_result = reader.read(&mut buffer);
        if file_read_result.is_err() {
            return Err(Box::new(file_read_result.err().unwrap()));
        }
        let read_count: usize = file_read_result.unwrap();
        if read_count == 0 {
            break;
        }
        byte_count += read_count as u64;

        let result = process_buffer(buffer[0..read_count].as_ref(), previous_byte);

        line_count += result.0;
        word_count += result.1;
        char_count += result.2;
        previous_byte = result.3;
    }

    Ok(Counts {
        byte_count,
        char_count,
        line_count,
        word_count,
    })
}

fn process_buffer(buffer: &[u8], previous_buffer_end_byte: u8) -> (u64, u64, u64, u8) {
    let mut line_count: u64 = 0;
    let mut word_count: u64 = 0;
    let mut char_count: u64 = 0;

    let mut previous_byte: u8 = previous_buffer_end_byte.clone();
    for i in 0..buffer.len() {
        if is_new_line(buffer[i]) {
            line_count += 1;
        }

        if is_new_utf8_char(buffer[i]) {
            char_count += 1;
        }

        if is_new_word(previous_byte, buffer[i]) {
            word_count += 1;
        }
        
        previous_byte = buffer[i];
    }

    (line_count, word_count, char_count, buffer[buffer.len() - 1]) // is_previous_byte_whitespace)
}

fn is_new_word(previous_byte: u8, current_byte: u8) -> bool {
    is_whitespace(previous_byte) && !is_whitespace(current_byte)
}

fn is_whitespace(byte: u8) -> bool {
    match byte {
        b' ' | b'\t' | b'\r' | b'\n' => true,
        _ => false,
    }
}

fn is_new_line(byte: u8) -> bool {
    match byte {
        b'\n' => true,
        _ => false,
    }
}

fn is_new_utf8_char(byte: u8) -> bool {
    (byte & 192) == 192 || (byte & 128) != 128
}

#[cfg(test)]
mod process_buffer_tests {
    use crate::wc::process_buffer;

    #[test]
    fn process_buffer_single_word_no_whitespace() {
        // Arrange
        let input = b"hello";

        //Act
        let (line_count, word_count, char_count, last_byte) = process_buffer(input, b' ');

        //Assert
        assert_eq!(line_count, 0);
        assert_eq!(word_count, 1);
        assert_eq!(char_count, 5);
        assert_eq!(last_byte, b'o');
    }

    #[test]
    fn process_buffer_single_word_leading_whitespace() {
        // Arrange
        let input = b" hello";

        //Act
        let (line_count, word_count, char_count, last_byte) = process_buffer(input, b' ');

        //Assert
        assert_eq!(line_count, 0);
        assert_eq!(word_count, 1);
        assert_eq!(char_count, 6);
        assert_eq!(last_byte, b'o');
    }

    #[test]
    fn process_buffer_single_word_trailing_whitespace() {
        // Arrange
        let input = b"hello ";

        //Act
        let (line_count, word_count, char_count, last_byte) = process_buffer(input, b' ');

        //Assert
        assert_eq!(line_count, 0);
        assert_eq!(word_count, 1);
        assert_eq!(char_count, 6);
        assert_eq!(last_byte, b' ');
    }

    #[test]
    fn process_buffer_two_words_separated_by_space() {
        // Arrange
        let input = b"hello world";

        //Act
        let (line_count, word_count, char_count, last_byte) = process_buffer(input, b' ');

        //Assert
        assert_eq!(line_count, 0);
        assert_eq!(word_count, 2);
        assert_eq!(char_count, 11);
        assert_eq!(last_byte, b'd');
    }

    #[test]
    fn process_buffer_two_words_separated_by_tab() {
        // Arrange
        let input = b"hello\tworld";

        //Act
        let (line_count, word_count, char_count, last_byte) = process_buffer(input, b' ');

        //Assert
        assert_eq!(line_count, 0);
        assert_eq!(word_count, 2);
        assert_eq!(char_count, 11);
        assert_eq!(last_byte, b'd');
    }

    #[test]
    fn process_buffer_two_lines() {
        // Arrange
        let input = b"hello\nworld";

        //Act
        let (line_count, word_count, char_count, last_byte) = process_buffer(input, b' ');

        //Assert
        assert_eq!(line_count, 1);
        assert_eq!(word_count, 2);
        assert_eq!(char_count, 11);
        assert_eq!(last_byte, b'd');
    }

    #[test]
    fn process_multi_byte_characters() {
        // Arrange
        let input = b"h\xE9llo w\xF8rld";

        //Act
        let (line_count, word_count, char_count, last_byte) = process_buffer(input, b' ');

        //Assert
        assert_eq!(line_count, 0);
        assert_eq!(word_count, 2);
        assert_eq!(char_count, 11);
        assert_eq!(last_byte, b'd');
    }
}

#[cfg(test)]
mod reader_tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn process_reader_single_word_no_whitespace() {
        // Arrange
        let data = b"hello";
        let cursor = Cursor::new(data.to_vec());

        //Act
        let counts = process_reader(cursor).unwrap();

        //Assert
        assert_eq!(counts.byte_count, 5);
        assert_eq!(counts.char_count, 5);
        assert_eq!(counts.line_count, 0);
        assert_eq!(counts.word_count, 1);
    }

    #[test]
    fn process_reader_multiple_words_and_lines() {
        // Arrange
        let data = b"hello world\nthis is a test";
        let cursor = Cursor::new(data.to_vec());

        //Act
        let counts = process_reader(cursor).unwrap();

        //Assert
        assert_eq!(counts.byte_count, 26);
        assert_eq!(counts.char_count, 26);
        assert_eq!(counts.line_count, 1);
        assert_eq!(counts.word_count, 6);
    }

    #[test]
    fn process_reader_empty() {
        // Arrange
        let data = b"";
        let cursor = Cursor::new(data.to_vec());

        //Act
        let counts = process_reader(cursor).unwrap();

        //Assert
        assert_eq!(counts.byte_count, 0);
        assert_eq!(counts.char_count, 0);
        assert_eq!(counts.line_count, 0);
        assert_eq!(counts.word_count, 0);
    }

    #[test]
    fn process_reader_whitespace_only() {
        // Arrange
        let data = b" \t \n \t";
        let cursor = Cursor::new(data.to_vec());

        //Act
        let counts = process_reader(cursor).unwrap();

        //Assert
        assert_eq!(counts.byte_count, 6);
        assert_eq!(counts.char_count, 6);
        assert_eq!(counts.line_count, 1);
        assert_eq!(counts.word_count, 0);
    }
}
