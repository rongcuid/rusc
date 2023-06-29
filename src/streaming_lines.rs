use miette::*;

use crate::{open_lines_input, open_lines_output, LineReader, LineWriter};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::io::Write;
use std::path::Path;

pub struct StreamingLines {
    /// Progress bars
    mp: MultiProgress,
    /// Input progress bar
    pb_in: ProgressBar,
    /// Output progress bar
    pb_out: ProgressBar,
    /// Input iterator
    line_reader: LineReader,
    /// Output iterator
    line_writer: LineWriter,
}

impl StreamingLines {
    pub fn new(
        input: Option<impl AsRef<Path>>,
        output: Option<impl AsRef<Path>>,
    ) -> Result<StreamingLines> {
        let mp = MultiProgress::new();
        let pb_in = mp.insert(
            0,
            ProgressBar::new_spinner().with_style(
                ProgressStyle::with_template(
                    "Lines read: [{elapsed_precise}] {human_pos} lines {per_sec} {msg}",
                )
                .into_diagnostic()?,
            ),
        );
        let pb_out = mp.insert(
            1,
            ProgressBar::new_spinner().with_style(
                ProgressStyle::with_template(
                    "Lines written: [{elapsed_precise}] {human_pos} lines {per_sec} {msg}",
                )
                .into_diagnostic()?,
            ),
        );
        let line_reader = open_lines_input(input)?;
        let line_writer = open_lines_output(output)?;
        Ok(Self {
            mp,
            pb_in,
            pb_out,
            line_reader,
            line_writer,
        })
    }

    pub fn for_each_line(&mut self, f: impl Fn(&str, &mut LineWriter) -> Result<()>) -> Result<()> {
        for line in &mut self.line_reader {
            let line = line.into_diagnostic()?;
            self.pb_in.inc(1);
            tracing::trace!("Line: {line}");
            f(&line, &mut self.line_writer)?;
            self.line_writer.flush().into_diagnostic()?;
            self.pb_out.inc(1);
        }
        Ok(())
    }
}
