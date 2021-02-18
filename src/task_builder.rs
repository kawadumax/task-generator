use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::fs;
use glob::glob;

#[derive(Debug)]
struct TaskBuilder {
    doc: PdfDocumentReference,
    page: PdfPageIndex,
    layer: PdfLayerIndex,
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

    fn delete_pdf() {
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
    
    pub fn export(&self){
        self.doc.save(&mut BufWriter::new(File::create("pdf/test_working.pdf").unwrap())).unwrap();
    }
       
}