mod svg_renderer;
#[cfg(test)]
mod tests;
mod utils;

use crate::svg_renderer::SvgRenderer;
use hyper::service::{make_service_fn, service_fn};
use hyper::{body, http, Body, Method, Request, Response, Server};
use std::{collections::HashMap, env, fs};
use tree_sitter_highlight::{Highlight, HighlightConfiguration, Highlighter};
use utils::{get_language, USVG_TREE_OPTIONS};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let addr = format!("127.0.0.1:{}", port)
        .parse()
        .expect("Invalid port value");
    let make_srv =
        make_service_fn(|_| async { Ok::<_, http::Error>(service_fn(serve)) });
    let server = Server::bind(&addr).serve(make_srv);
    log::info!("Server listening on :{}", port);
    if let Err(err) = server.await {
        log::error!("{:?}", err);
    }
}

fn parse_query_string(query: &str) -> HashMap<String, String> {
    query.split('&').fold(HashMap::new(), |mut acc, cur| {
        let kv: Vec<_> = cur.splitn(2, '=').collect();
        acc.insert(kv[0].to_string(), kv[1].to_string());
        acc
    })
}

async fn highlight(req: Request<Body>) -> Option<Body> {
    let qs = req.uri().query().map(parse_query_string);
    let language_name = qs.map(|qs| qs.get("lang").cloned()).flatten()?;
    let bytes = body::to_bytes(req.into_body())
        .await
        .ok()?
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    let source_code = String::from_utf8(bytes).ok()?;

    let mut hl_cfg = {
        let cfg = get_language(&language_name)?;
        HighlightConfiguration::new(
            cfg.language,
            &cfg.highlight_query,
            &cfg.injections_query,
            &cfg.locals_query,
        )
        .ok()?
    };
    let mut highlighter = Highlighter::new();
    let hl_names = hl_cfg.names().to_vec();
    let svg_attributes: Vec<_> = hl_names
        .iter()
        .map(|name| format!(r#"class="{}""#, name.replace(".", " ")))
        .collect();
    hl_cfg.configure(&hl_names.to_vec());
    log::info!("Highlighting...");
    let events = highlighter
        .highlight(&hl_cfg, source_code.as_bytes(), None, |_| None)
        .ok()?;

    log::info!("Creating renderer...");
    let attribute_callback = |hl: &Highlight| svg_attributes[hl.0].clone();
    let mut svg_renderer = SvgRenderer::new(&source_code, &attribute_callback);

    let stylesheet =
        fs::read_to_string("assets/stylesheets/ayu-vim.css").ok()?;
    log::info!("Rendering SVG...");
    svg_renderer.render(events, stylesheet).ok()?;
    let tree =
        usvg::Tree::from_str(&svg_renderer.get_svg(), &USVG_TREE_OPTIONS)
            .ok()?;
    let (width, height) = svg_renderer.get_picture_size();
    let mut pixmap = tiny_skia::Pixmap::new(width as u32, height as u32)?;
    log::info!("Rendering PNG...");
    resvg::render(&tree, usvg::FitTo::Original, pixmap.as_mut())?;
    Some(Body::from(pixmap.encode_png().ok()?))
}

async fn serve(req: Request<Body>) -> http::Result<Response<Body>> {
    match (req.uri().path(), req.method()) {
        ("/", &Method::POST) => {
            if let Some(body) = highlight(req).await {
                Response::builder()
                    .status(200)
                    .header("Content-Type", "image/png")
                    .body(body)
            } else {
                Response::builder().status(400).body(Body::empty())
            }
        }
        _ => Response::builder().status(404).body(Body::empty()),
    }
}
