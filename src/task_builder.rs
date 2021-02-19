use chrono::Local;
use glob::glob;
use once_cell::sync::Lazy;
use printpdf::{indices, *};
use std::fs::File;
use std::io::BufWriter;
use std::{fs, ops::Deref};

pub struct TaskBuilder {
    doc: PdfDocumentReference,
    page_index: indices::PdfPageIndex,
    layer_index: indices::PdfLayerIndex,
}

impl TaskBuilder {
    const HONOKA_FONT: Lazy<File> =
        Lazy::new(|| File::open("assets/font_1_honokamin.ttf").unwrap());
    const A4_WIDTH: Mm = Mm(210.0);
    const A4_HEIGHT: Mm = Mm(297.0);
    const OFFSET_HORIZON: Mm = Mm(15.0);
    const OFFSET_VERTICAL: Mm = Mm(15.0);

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
        let text = "次の表をエクセルに入力してください。";
        let font = self
            .doc
            .add_external_font(Self::HONOKA_FONT.deref())
            .unwrap();
        // text, font size, x from left edge, y from bottom edge, font
        current_layer.use_text(
            text,
            18.0,
            Self::OFFSET_HORIZON,
            Self::A4_HEIGHT - Self::OFFSET_VERTICAL,
            &font,
        );
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
