#[cfg(feature = "markdown")]
pub use ::text_splitter::MarkdownSplitter;
use ::text_splitter::TextSplitter;
use rustler::NifTaggedEnum;

#[derive(NifTaggedEnum, Debug)]
pub enum SearchOptionPub {
    MaxTokens(usize),
    MinTokens(usize),
    TrimChunks(bool),
}
#[derive(Debug)]
struct SearchOption {
    max_tokens: usize,
    min_tokens: usize,
    trim_chunks: bool,
}
fn get_options(options: Vec<SearchOptionPub>) -> SearchOption {
    let mut opts = SearchOption {
        max_tokens: 1000,
        min_tokens: 0,
        trim_chunks: false,
    };

    options.iter().for_each(|option| match option {
        SearchOptionPub::MaxTokens(val) => opts.max_tokens = *val,
        SearchOptionPub::MinTokens(val) => opts.min_tokens = *val,
        SearchOptionPub::TrimChunks(val) => opts.trim_chunks = *val,
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
#[cfg(not(feature = "markdown"))]
#[rustler::nif]
fn markdown_splitter(_query: &str, _options: Vec<SearchOptionPub>) -> Vec<&str> {
    vec![]
}

rustler::init!(
    "Elixir.ExTextSplitter.Native",
    [markdown_splitter, text_splitter]
);
