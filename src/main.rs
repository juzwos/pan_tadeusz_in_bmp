mod poem;

fn main() -> std::io::Result<()> {

    let mut char_pixels_mapper = poem::poem::CharPixelsMapper::new();
    let mut canvas = poem::poem::Canvas::new();
    let mut text_processor = poem::poem::TextProcessor::new();

    text_processor.process("pantadeusz.txt", &mut canvas, &mut char_pixels_mapper)
}
