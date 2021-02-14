use htmlescape::encode_attribute;
use tree_sitter_highlight::{Error, Highlight, HighlightEvent};

pub struct SvgRenderer {
    hl_stack: Vec<Highlight>,
    svg: String,
}

type HlEvent = Result<HighlightEvent, Error>;

impl SvgRenderer {
    pub fn new() -> Self {
        Self {
            svg: String::with_capacity(10 * 1024),
            hl_stack: vec![],
        }
    }

    /// Renger highlight events to svg string
    pub fn render(
        &mut self,
        source: &str,
        events: impl Iterator<Item = HlEvent>,
        attr_fn: &impl Fn(Highlight) -> String,
        stylesheet: String,
    ) -> Result<(), Error> {
        self.prologue(stylesheet);
        for event in events {
            match event {
                Ok(HighlightEvent::HighlightStart(hl)) => {
                    self.start_hl(hl, attr_fn);
                }
                Ok(HighlightEvent::Source { start, end }) => {
                    // TODO: wrap text longer than 80 chars
                    self.svg += &encode_attribute(&source[start..end]);
                }
                Ok(HighlightEvent::HighlightEnd) => {
                    self.end_hl();
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        self.epilogue();
        Ok(())
    }

    fn start_hl(
        &mut self,
        hl: Highlight,
        attr_fn: &impl Fn(Highlight) -> String,
    ) {
        self.svg += r#"<tspan"#;
        let attributes = attr_fn(hl);
        if !attributes.is_empty() {
            self.svg.push(' ');
            self.svg += &attributes;
        }
        self.svg.push('>');
    }

    fn end_hl(&mut self) {
        self.svg += "</tspan>";
    }

    fn prologue(&mut self, stylesheet: String) {
        let mut styles = r#"
            text {
              font-family: monospace;
              white-space: pre-wrap;
              font-size: 10px;
            }
        "#
        .to_string();
        styles += &stylesheet;
        self.svg += &format!(
            r#"
            <svg width="100%" height="100%" xmlns="http://www.w3.org/2000/svg">
                <style>{}</style>
                <rect width="100%" height="100%" class="background-rect" />
                <text x="0" y="1em">
            "#,
            styles,
        )
        .trim();
    }

    fn epilogue(&mut self) {
        self.svg += "</text></svg>";
    }

    pub fn svg(&self) -> String {
        self.svg.clone()
    }
}
