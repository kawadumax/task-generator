use chrono::Local;
use glob::glob;
use printpdf::{indices, *};
use std::fs;
use std::fs::File;
use std::io::BufWriter;

pub struct TaskBuilder {
    doc: PdfDocumentReference,
    page_index: indices::PdfPageIndex,
    layer_index: indices::PdfLayerIndex,
}

impl TaskBuilder {
    pub fn new(mode: u8) -> Self {
        Self::mkdir_pdf();
        Self::rm_pdf();
        let (doc, page_index, layer_index) =
            PdfDocument::new("Task", Mm(210.0), Mm(297.0), "Layer 1");
        Self {
            doc,
            page_index,
            layer_index,
        }
    }

    pub fn preface(&self) {
        let current_layer = self
            .doc
            .get_page(self.page_index)
            .get_layer(self.layer_index);
        let text = "Lorem ipsum";
        let text2 = "unicode: стуфхfцчшщъыьэюя";

        let font = self
            .doc
            .add_external_font(File::open("assets/font_1_honokamin.ttf").unwrap())
            .unwrap();

        // text, font size, x from left edge, y from bottom edge, font
        current_layer.use_text(text, 48.0, Mm(20.0), Mm(100.0), &font);

        // For more complex layout of text, you can use functions
        // defined on the PdfLayerReference
        // Make sure to wrap your commands
        // in a `begin_text_section()` and `end_text_section()` wrapper
        current_layer.begin_text_section();

        // setup the general fonts.
        // see the docs for these functions for details
        current_layer.set_font(&font, 33.0);
        current_layer.set_text_cursor(Mm(10.0), Mm(10.0));
        current_layer.set_line_height(33.0);
        current_layer.set_word_spacing(3000.0);
        current_layer.set_character_spacing(10.0);
        current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);

        // write two lines (one line break)
        current_layer.write_text(text.clone(), &font);
        current_layer.add_line_break();
        current_layer.write_text(text2.clone(), &font);
        current_layer.add_line_break();

        // write one line, but write text2 in superscript
        current_layer.write_text(text.clone(), &font);
        current_layer.set_line_offset(10.0);
        current_layer.write_text(text2.clone(), &font);

        current_layer.end_text_section();
    }

    pub fn export(self) {
        let t = Local::now().format("%Y%m%d-%H%M%S-").to_string();
        let path = "pdf/".to_string() + t.as_str() + "task.pdf";
        self.doc
            .save(&mut BufWriter::new(File::create(path).unwrap()))
            .unwrap();
    }

    fn rm_pdf() {
        for entry in glob("pdf/*.pdf").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    println!("{:?}", path.display());
                    fs::remove_file(path);
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }

    fn mkdir_pdf() {
        match fs::create_dir("pdf") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {}
        }
    }
}
