use htmlescape::encode_minimal;
use tree_sitter_highlight::{Error, Highlight, HighlightEvent};

pub struct SvgRenderer<'a, AttrFn> {
    hl_stack: Vec<Highlight>,
    svg: String,
    source: &'a str,
    current_line: u32,
    attr_callback: AttrFn,
    max_line_width: usize,
    line_count: usize,
    font_size: usize,
    picture_width: usize,
    number_column_width: usize,
    font_aspect_ratio: f32,
}

impl<'a, AttrFn> SvgRenderer<'a, AttrFn>
where
    AttrFn: Fn(&Highlight) -> String,
{
    pub fn new(source: &'a str, attr_callback: AttrFn) -> Self {
        let mut result = Self {
            svg: String::with_capacity(10 * 1024),
            source,
            hl_stack: Vec::new(),
            current_line: 0,
            attr_callback,
            max_line_width: 0,
            line_count: 0,
            font_size: 20,
            picture_width: 512 << 1,
            number_column_width: 4,
            font_aspect_ratio: 3. / 5.,
        };
        result.calculate_max_line_width();
        result
    }

    fn calculate_max_line_width(&mut self) {
        let lines = self.source.split('\n');
        self.max_line_width =
            lines.clone().map(|line| line.len()).max().unwrap()
                + self.number_column_width;
        self.line_count = lines.count();
    }

    /// Renger highlight events to svg string
    pub fn render<I>(
        &mut self,
        events: I,
        stylesheet: String,
    ) -> Result<(), Error>
    where
        I: Iterator<Item = Result<HighlightEvent, Error>>,
    {
        self.prologue(stylesheet);
        for event in events {
            match event {
                Ok(HighlightEvent::HighlightStart(hl)) => {
                    self.start_highligt(&hl)
                }
                Ok(HighlightEvent::Source { start, end }) => {
                    self.highlight_source(start, end);
                }
                Ok(HighlightEvent::HighlightEnd) => {
                    self.end_highlight();
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        self.epilogue();
        Ok(())
    }

    fn start_highligt(&mut self, hl: &Highlight) {
        self.hl_stack.push(hl.clone());
        self.new_tspan(&hl);
    }

    fn highlight_source(&mut self, start: usize, end: usize) {
        let chunk = encode_minimal(&self.source[start..end].to_owned());
        for c in chunk.chars() {
            if c == '\n' {
                self.svg += &"</tspan>".repeat(self.hl_stack.len());
                self.svg += &"</text>\n";
                self.new_text();
                for hl in self.hl_stack.clone().iter() {
                    self.new_tspan(&hl);
                }
                continue;
            }
            self.svg.push(c);
        }
    }

    fn end_highlight(&mut self) {
        self.svg += "</tspan>";
        self.hl_stack.pop();
    }

    fn new_text(&mut self) {
        self.current_line += 1;
        self.svg += &format!(
            r#"<text y="{line}em" class="meta separator" xml:space="preserve">{:>3} </text><text x="48" y="{line}em" xml:space="preserve">"#,
            self.current_line,
            line = self.current_line,
        );
    }

    fn new_tspan(&mut self, highlight: &Highlight) {
        self.svg += "<tspan";
        let attributes = (self.attr_callback)(&highlight);
        if !attributes.is_empty() {
            self.svg.push(' ');
            self.svg += &attributes;
        }
        self.svg.push('>');
    }

    fn prologue(&mut self, stylesheet: String) {
        let font_size = (self.picture_width as f32
            / self.max_line_width as f32
            / self.font_aspect_ratio) as usize;
        let (width, height) = self.get_picture_size();
        self.svg += &format!(
            r#"
<svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">
    <style>
        text {{
          font-family: monospace;
          font-size: {}px;
          fill: #FFFFFF;
        }}
        {}
    </style>
    <rect width="100%" height="100%" class="background" />
            "#,
            width, height, font_size, stylesheet,
        )
        .trim();
        self.svg.push('\n');
        self.new_text();
    }

    fn epilogue(&mut self) {
        self.svg += "</text></svg>";
    }

    pub fn get_svg(&self) -> &str {
        &self.svg
    }

    pub fn get_picture_size(&self) -> (usize, usize) {
        (
            self.picture_width,
            (self.picture_width as f32 / self.get_aspect_ratio()) as usize + 10,
        )
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        let char_width = self.font_size as f32 * self.font_aspect_ratio;
        let width = char_width * self.max_line_width as f32;
        let height = self.font_size * self.line_count;
        width as f32 / height as f32
    }
}
