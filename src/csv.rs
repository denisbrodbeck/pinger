use std::path::PathBuf;

pub struct CsvWriter {
    wf: Option<csv::Writer<std::fs::File>>,
    wo: Option<csv::Writer<std::io::Stdout>>,
}

// Tried to implement something like this
// (see https://www.reddit.com/r/rust/comments/3gtpy9/wrapping_around_generic_io/cu1ebi0)
//
// let mut wtr:  csv::Writer<Box<dyn std::io::Write>> = csv::WriterBuilder::new().from_writer(
//     if opt.output.is_some() {
//         let mut h = std::fs::File::create(opt.output.unwrap()).unwrap();
//         Box::new(h)
//     } else {
//         Box::new(std::io::stdout())
//     }
// );
// That failed with error message:
// 27  |     task::block_on(task::spawn(async {
//     |                    ^^^^^^^^^^^ future returned by `main` is not `Send`
//     |
//     = help: the trait `std::marker::Send` is not implemented for `dyn std::io::Write`
// Could not solve this and got this variant working.

impl CsvWriter {
    pub fn new(path: Option<PathBuf>) -> CsvWriter {
        if path.is_some() {
            let w = csv::WriterBuilder::new()
                .delimiter(b';')
                .quote_style(csv::QuoteStyle::Necessary)
                .from_path(path.as_ref().unwrap())
                .unwrap();
            CsvWriter {
                wf: Some(w),
                wo: None,
            }
        } else {
            let w = csv::WriterBuilder::new()
                .delimiter(b';')
                .quote_style(csv::QuoteStyle::Necessary)
                .from_writer(std::io::stdout());
            CsvWriter {
                wf: None,
                wo: Some(w),
            }
        }
    }

    pub fn write_record<I, T>(&mut self, record: I) -> Result<(), csv::Error>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<[u8]>,
    {
        if self.wf.as_ref().is_some() {
            let w = self.wf.as_mut().unwrap();
            w.write_record(record)?;
            w.flush()?;
            Ok(())
        } else {
            let w = self.wo.as_mut().unwrap();
            w.write_record(record)?;
            w.flush()?;
            Ok(())
        }
    }
}
