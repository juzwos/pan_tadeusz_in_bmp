pub(crate) mod poem {
    use std::collections::HashMap;
    use std::{cmp, fs};
    use bmp::{Image, Pixel};
    #[derive(Debug, Clone)]
    pub struct CharPixelsMapper {
        char_pixels: HashMap<char, Vec<u8>>
    }

    #[derive(Debug, Clone)]
    pub struct TextProcessor {
        text_lines: Vec<String>
    }

    #[derive(Debug, Clone)]
    pub struct Canvas {
        margin_y: i32,
        margin_x: i32,
        letter_height: i32,
        bmp_image: Image
    }

    impl Canvas {
        pub(crate) fn new() -> Canvas {
            Canvas {
                margin_y: 8,
                margin_x: 45,
                letter_height: 3,
                bmp_image: Image::new(3840, 2160),
            }
        }
    }

    impl CharPixelsMapper {
        pub(crate) fn new() -> CharPixelsMapper {

            let mut char_to_pixels : HashMap<char, Vec<u8>> = HashMap::new();

            char_to_pixels.insert('a', vec![0, 1, 0, 1, 1, 1, 1, 0, 1 ]);
            char_to_pixels.insert('b', vec![1, 1, 0, 1, 1, 1, 1, 1, 1 ]);
            char_to_pixels.insert('c', vec![1, 1, 1, 0, 1, 1 ]);
            char_to_pixels.insert('d', vec![1, 1, 0, 1, 0, 1, 1, 1, 0 ]);
            char_to_pixels.insert('e', vec![1, 1, 1, 1, 1, 0, 1, 1, 1 ]);
            char_to_pixels.insert('f', vec![1, 1, 1, 1, 1, 0, 1, 0, 0 ]);
            char_to_pixels.insert('g', vec![1, 0, 0, 1, 0, 1, 1, 1, 1 ]);
            char_to_pixels.insert('h', vec![1, 0, 1, 1, 1, 1, 1, 0, 1 ]);
            char_to_pixels.insert('i', vec![1, 1, 1 ]);
            char_to_pixels.insert('j', vec![0, 0, 1, 1, 0, 1, 1, 1, 1 ]);
            char_to_pixels.insert('k', vec![1, 0, 1, 1, 1, 0, 1, 0, 1 ]);
            char_to_pixels.insert('l', vec![1, 0, 1, 0, 1, 1 ]);
            char_to_pixels.insert('m', vec![1, 1, 0, 0, 1, 1, 0, 0, 1 ]);
            char_to_pixels.insert('n', vec![1, 1, 1, 1, 0, 1, 1, 0, 1 ]);
            char_to_pixels.insert('o', vec![1, 1, 1, 1, 0, 1, 1, 1, 1 ]);
            char_to_pixels.insert('p', vec![1, 1, 1, 1, 1, 1, 1, 0, 0 ]);
            char_to_pixels.insert('q', vec![1, 1, 1, 1, 1, 1, 0, 0, 1 ]);
            char_to_pixels.insert('r', vec![1, 1, 1, 0, 1, 0 ]);
            char_to_pixels.insert('s', vec![0, 1, 1, 0, 1, 0, 1, 1, 0 ]);
            char_to_pixels.insert('t', vec![1, 1, 1, 0, 1, 0, 0, 1, 0 ]);
            char_to_pixels.insert('u', vec![1, 0, 1, 1, 0, 1, 1, 1, 1 ]);
            char_to_pixels.insert('v', vec![1, 0, 1, 1, 0, 1, 0, 1, 0 ]);
            char_to_pixels.insert('w', vec![1, 0, 0, 1, 1, 0, 0, 1, 1 ]);
            char_to_pixels.insert('x', vec![1, 0, 1, 0, 1, 0, 1, 0, 1 ]);
            char_to_pixels.insert('y', vec![1, 0, 1, 1, 1, 1, 0, 1, 0 ]);
            char_to_pixels.insert('z', vec![1, 1, 0, 0, 1, 0, 0, 1, 1 ]);
            char_to_pixels.insert('.', vec![0, 0, 1 ]);
            char_to_pixels.insert(',', vec![0, 1, 1 ]);
            char_to_pixels.insert('"', vec![1, 0, 1, 0, 0, 0, 0, 0, 0 ]);
            char_to_pixels.insert('(', vec![0, 1, 1, 0, 0, 1 ]);
            char_to_pixels.insert(')', vec![1, 0, 0, 1, 1, 0 ]);
            char_to_pixels.insert('!', vec![1, 1, 0 ]);
            char_to_pixels.insert('?', vec![1, 1, 0, 0, 0, 1 ]);
            char_to_pixels.insert(':', vec![1, 0, 1 ]);
            char_to_pixels.insert('-', vec![0, 0, 1, 1, 0, 0 ]);
            char_to_pixels.insert('1', vec![1, 1, 0, 1, 0, 1 ]);
            char_to_pixels.insert('2', vec![1, 1, 0, 0, 1, 0, 0, 1, 1 ]);
            char_to_pixels.insert('8', vec![0, 1, 1, 1, 1, 1, 1, 1, 0 ]);

            CharPixelsMapper {
                char_pixels : char_to_pixels
            }
        }
    }

    impl TextProcessor {
        pub(crate) fn new() -> TextProcessor {
            TextProcessor{
                text_lines: vec![],
            }
        }

        pub fn process(&mut self, file_path: &str, canvas: &mut Canvas, char_pixels_mapper : &CharPixelsMapper) -> std::io::Result<()> {

            self.text_lines = TextProcessor::read_content(file_path);
            self.clear_canvas(canvas);

            let mut current_x = canvas.margin_x;
            let mut current_y = canvas.letter_height + 3;
            let mut chapter_record_width = 0;

            for line in &self.text_lines {
                if line.len() == 0 {
                    continue
                }

                if (current_y + canvas.letter_height) as u32 > (canvas.bmp_image.get_height() - canvas.margin_y as u32)
                {
                    current_x =  2 + chapter_record_width;
                    current_y = canvas.margin_y;
                    chapter_record_width = 0;
                }

                let line_width = self.draw_text_line(canvas, char_pixels_mapper, line, current_x, current_y as i32);
                chapter_record_width = cmp::max(chapter_record_width, line_width);

                current_y += canvas.letter_height + 1;
            }

            canvas.bmp_image.save("pan_tadeusz.bmp")
        }

        fn read_content(file_path: &str) -> Vec<String> {
            let contents = fs::read_to_string(file_path).
                expect("Unable to read file");

            let contents : String = contents.chars()
                .map(|x|
                    match x {
                        '…' => '.',
                        ';' => ',',
                        '«' => '\"',
                        '»' => '\"',
                        '—' => '-',
                        'à' => 'a',
                        'ę' => 'e',
                        'æ' => 'a',
                        'é' => 'e',
                        'ó' => 'o',
                        'ł' => 'l',
                        'ś' => 's',
                        'ą' => 'a',
                        'ż' => 'z',
                        'ź' => 'z',
                        'ć' => 'c',
                        'ń' => 'n',
                        '*' => '\0',
                        _ => x
                    })
                .collect();

           contents.lines()
                    .map(|line| line.trim().to_lowercase().to_string())
                    .collect()
        }

        fn clear_canvas(&self, canvas : &mut Canvas) {
            for (x, y) in canvas.bmp_image.coordinates() {
                canvas.bmp_image.set_pixel(x, y, bmp::consts::WHITE);
            }
        }

        fn draw_text_line(&self, canvas : &mut Canvas,
                          char_pixels_mapper : &CharPixelsMapper,
                          line : &str,
                          start_x : i32,
                          start_y :i32) -> i32 {
            let mut current_x = start_x;
            let current_y = start_y;

            for i in line.chars() {
                if i == ' ' {
                    current_x += 2;
                    continue
                }

                if i == '\n' || i == '\r' {
                    continue
                }

                if char_pixels_mapper.char_pixels.contains_key(&i) == false {
                    continue
                }

                current_x = self.draw_pixels( canvas, char_pixels_mapper, i, current_x, current_y);

            }

            current_x
        }

        fn draw_pixels (&self, canvas : &mut Canvas,
                        char_pixels_mapper : &CharPixelsMapper,
                        ch : char,
                        start_x : i32,
                        start_y :i32) -> i32 {
            let bits_len = char_pixels_mapper.char_pixels.get(&ch).unwrap().len();

            for x in 0..bits_len / 3 {
                for y in 0..canvas.letter_height {
                     let idx = (bits_len / 3) * y as usize + x;
                     let pixel = match char_pixels_mapper.char_pixels.get(&ch).unwrap().get(idx).unwrap() {
                         1 => Pixel::new(0,0,0),
                         _ => Pixel::new(255,255,225)
                     };

                    canvas.bmp_image.set_pixel(start_x as u32 + x as u32, start_y as u32 + y as u32, pixel)
                }
            }

            start_x + (bits_len as i32 / 3) + 1
        }
    }
}