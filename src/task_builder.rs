use printpdf::{*, indices};
use std::fs::File;
use std::io::BufWriter;
use std::fs;
use glob::glob;
use chrono::{Local, DateTime};

pub struct TaskBuilder {
    doc: PdfDocumentReference,
    page: indices::PdfPageIndex,
    layer: indices::PdfLayerIndex,
}


impl TaskBuilder {

    pub fn new() -> Self {
        let (doc, page, layer) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
        Self {
            doc,
            page,
            layer
        }
    }

    pub fn delete_pdf(&self) {
        for entry in glob("pdf/*.pdf").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    println!("{:?}", path.display());
                    fs::remove_file(path);
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }
    
    pub fn export(self){
        let t = Local::now().format("%Y%m%d%H%M%S").to_string();
        let path = "pdf/".to_string() + t.as_str() + "task.pdf";
        self.doc.save(&mut BufWriter::new(File::create(path).unwrap())).unwrap();
    }
       
}