use crate::{TaskDataRow, TaskDataTable};
use chrono::Local;
use glob::glob;
use once_cell::sync::Lazy;
use printpdf::{indices, *};
use std::cell::Cell;
use std::fs::File;
use std::io::BufWriter;
use std::{fs, ops::Deref};

pub struct TaskBuilder {
    doc: PdfDocumentReference,
    page_index: indices::PdfPageIndex,
    layer_index: indices::PdfLayerIndex,
    current_layer: PdfLayerReference,
    used_offset: Cell<Mm>,
    font: IndirectFontRef,
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
        let current_layer = doc.get_page(page_index).get_layer(layer_index);
        let font = doc.add_external_font(Self::HONOKA_FONT.deref()).unwrap();
        Self {
            doc,
            page_index,
            layer_index,
            current_layer,
            used_offset: Cell::new(Mm(0.0)), // すでにコンテンツがある部分として足していく。
            font,
        }
    }

    pub fn add_used_offset(&self, offset: Mm) {
        let offset = self.used_offset.get() + offset;
        self.used_offset.set(offset);
    }

    pub fn preface(&self) {
        let text = "次の表をエクセルに入力してください。";
        // text, font size, x from left edge, y from bottom edge, font
        let font_size = 18.0;
        self.current_layer.use_text(
            text,
            font_size,
            Self::OFFSET_HORIZON,
            Self::A4_HEIGHT - Self::OFFSET_VERTICAL,
            &self.font,
        );
        self.add_used_offset(Self::OFFSET_VERTICAL + Mm(font_size));
    }

    pub fn table(&self, data: &TaskDataTable) {
        for data_row in data {
            self.row(&data_row);
        }
    }

    pub fn row(&self, data: &TaskDataRow) {
        let x = Self::OFFSET_HORIZON;
        let y = Self::A4_HEIGHT - self.used_offset.get();
        let width: Mm = Self::A4_WIDTH - Self::OFFSET_HORIZON * 2.0;
        let height = Mm(13.0);
        let outline = self.square(x, y, width, height);
        // Is the shape stroked? Is the shape closed? Is the shape filled?
        self.current_layer.add_shape(outline);
        // for i in 0..cell_num {
        for (index, &text) in data.iter().enumerate() {
            let cell_num = data.len() as f64;
            let width_ratio: Mm = width / cell_num;
            let x = Self::OFFSET_HORIZON + width_ratio * index as f64;
            let vertical_line = self.vertical_line(x, y, height);
            self.current_layer.add_shape(vertical_line);
            // フォントのアンカーポイントは左下っぽい
            let text_margin_left = Mm(5.0);
            self.current_layer.use_text(
                text,
                Pt::from(height).0 / 2.0,
                x + text_margin_left,
                y - height / 1.5,
                &self.font,
            )
        }
        self.add_used_offset(height);
    }

    pub fn square(&self, x: Mm, y: Mm, width: Mm, height: Mm) -> Line {
        //左上から時計回りに□を表現する
        let square_points = vec![
            (Point::new(x, y), false),
            (Point::new(x + width, y), false),
            (Point::new(x + width, y - height), false),
            (Point::new(x, y - height), false),
        ];
        Line {
            points: square_points,
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        }
    }

    pub fn vertical_line(&self, x: Mm, y: Mm, height: Mm) -> Line {
        let vertical_points = vec![
            (Point::new(x, y), false),
            (Point::new(x, y - height), false),
        ];
        Line {
            points: vertical_points,
            is_closed: false,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        }
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
