use htmlescape::encode_minimal;
use tree_sitter_highlight::{Error, Highlight, HighlightEvent};

pub struct SvgRenderer<AttrFn> {
    hl_stack: Vec<Highlight>,
    svg: String,
    source: String,
    current_line: u32,
    attr_callback: AttrFn,
}

impl<AttrFn> SvgRenderer<AttrFn>
where
    AttrFn: Fn(&Highlight) -> String,
{
    pub fn new(source: String, attr_callback: AttrFn) -> Self {
        Self {
            svg: String::with_capacity(10 * 1024),
            source,
            hl_stack: vec![],
            current_line: 0,
            attr_callback,
        }
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
            r#"<text y="{line}em" class="meta separator">{line}</text><text x="36" y="{line}em" xml:space="preserve">"#,
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
        let (width, height) = self.get_picture_size();
        self.svg += &format!(
            r#"
<svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">
    <style>
        text {{
          font-family: monospace;
          font-size: 20px;
          fill: #FFFFFF;
        }}
        {}
    </style>
    <rect width="100%" height="100%" class="background" />
            "#,
            width, height, stylesheet,
        )
        .trim();
        self.svg.push('\n');
        self.new_text();
    }

    fn epilogue(&mut self) {
        self.svg += "</text></svg>";
    }

    pub fn get_svg(&self) -> String {
        self.svg.clone()
    }

    pub fn get_picture_size(&self) -> (usize, usize) {
        let lines = self.source.split("\n");
        // Considering line numbers
        let max_line_width =
            lines.clone().map(|line| line.len()).max().unwrap() + 3;
        let font_size = 20;
        (
            font_size / 5 * 3 * max_line_width,
            font_size * lines.count(),
        )
    }
}
