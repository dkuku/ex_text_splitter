#[cfg(feature = "markdown")]
pub use ::text_splitter::MarkdownSplitter;

#[cfg(feature = "tiktoken-rs")]
use tiktoken_rs::CoreBPE;

use ::text_splitter::TextSplitter;
use rustler::NifTaggedEnum;

#[derive(NifTaggedEnum, Debug)]
pub enum SearchOptionPub {
    MaxTokens(usize),
    MinTokens(usize),
    TrimChunks(bool),
    Tokenizer(String),
}
#[derive(Debug)]
struct SearchOption {
    max_tokens: usize,
    min_tokens: usize,
    trim_chunks: bool,
    tokenizer: String,
}
fn get_options(options: Vec<SearchOptionPub>) -> SearchOption {
    let mut opts = SearchOption {
        max_tokens: 1000,
        min_tokens: 0,
        trim_chunks: false,
        tokenizer: "cl100k_base".to_string(),
    };

    options.iter().for_each(|option| match option {
        SearchOptionPub::MaxTokens(val) => opts.max_tokens = *val,
        SearchOptionPub::MinTokens(val) => opts.min_tokens = *val,
        SearchOptionPub::TrimChunks(val) => opts.trim_chunks = *val,
        SearchOptionPub::Tokenizer(val) => opts.tokenizer = val.clone(),
    });
    opts
}
#[rustler::nif]
fn text_splitter(query: &str, options: Vec<SearchOptionPub>) -> Vec<&str> {
    let opts = get_options(options);
    let splitter = TextSplitter::default().with_trim_chunks(opts.trim_chunks);
    let chunks = splitter.chunks(query, opts.max_tokens);
    chunks.into_iter().collect::<Vec<&str>>()
}
#[cfg(feature = "markdown")]
#[rustler::nif]
fn markdown_splitter(query: &str, options: Vec<SearchOptionPub>) -> Vec<&str> {
    let opts = get_options(options);
    let splitter = MarkdownSplitter::default().with_trim_chunks(opts.trim_chunks);
    let chunks = splitter.chunks(query, opts.max_tokens);
    chunks.into_iter().collect::<Vec<&str>>()
}

#[cfg(feature = "tiktoken-rs")]
#[rustler::nif]
fn tokenizer_text_splitter<'a>(query: &'a str, options: Vec<SearchOptionPub>) -> Vec<&'a str> {
    let opts = get_options(options);
    match get_tokenizer(&opts.tokenizer) {
        Some(tokenizer) => {
            let splitter = TextSplitter::new(&tokenizer).with_trim_chunks(opts.trim_chunks);
            let chunks = splitter.chunks(query, opts.max_tokens);
            chunks.into_iter().collect::<Vec<&str>>()
        }
        None => Vec::new(),
    }
}
#[cfg(all(feature = "tiktoken-rs", feature = "markdown"))]
#[rustler::nif]
fn tokenizer_markdown_splitter<'a>(query: &'a str, options: Vec<SearchOptionPub>) -> Vec<&'a str> {
    let opts = get_options(options);
    match get_tokenizer(&opts.tokenizer) {
        Some(tokenizer) => {
            let splitter = MarkdownSplitter::new(&tokenizer).with_trim_chunks(opts.trim_chunks);
            let chunks = splitter.chunks(query, opts.max_tokens);
            chunks.into_iter().collect::<Vec<&str>>()
        }
        None => Vec::new(),
    }
}
#[cfg(feature = "tiktoken-rs")]
fn get_tokenizer(tokenizer: &str) -> Option<CoreBPE> {
    match tokenizer {
        "cl100k_base" => tiktoken_rs::cl100k_base(),
        "p50k_base" => tiktoken_rs::p50k_base(),
        "r50k_base" => tiktoken_rs::r50k_base(),
        "p50k_edit" => tiktoken_rs::p50k_edit(),
        // todo check singleton speed
        // "cl100k_base" => tiktoken_rs::cl100k_base_singleton(),
        // "p50k_base" => tiktoken_rs::p50k_base_singleton(),
        // "r50k_base" => tiktoken_rs::r50k_base_singleton(),
        // "p50k_edit" => tiktoken_rs::p50k_edit_singleton(),
        _other => todo!("other not implemented"),
    }
    .ok()
}
#[cfg(all(feature = "tiktoken-rs", feature = "markdown"))]
rustler::init!(
    "Elixir.ExTextSplitter.Native",
    [
        markdown_splitter,
        text_splitter,
        tokenizer_markdown_splitter,
        tokenizer_text_splitter
    ]
);
#[cfg(all(feature = "tiktoken-rs", not(feature = "markdown")))]
rustler::init!(
    "Elixir.ExTextSplitter.Native",
    [text_splitter, tokenizer_text_splitter]
);
#[cfg(all(not(feature = "tiktoken-rs"), feature = "markdown"))]
rustler::init!(
    "Elixir.ExTextSplitter.Native",
    [markdown_splitter, text_splitter]
);
#[cfg(all(not(feature = "tiktoken-rs"), not(feature = "markdown")))]
rustler::init!("Elixir.ExTextSplitter.Native", [text_splitter]);
