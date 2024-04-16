defmodule ExTextSplitter do
  @moduledoc """
  Documentation for `ExTextSplitter`.
  """
end

defmodule ExTextSplitter.Native do
  @moduledoc "README.md"
             |> File.read!()
             |> String.split("<!-- MDOC !-->")
             |> Enum.fetch!(0)
  @features Application.compile_env(:ex_text_splitter, :features, []) |> List.wrap()

  use Rustler,
    otp_app: :ex_text_splitter,
    features: @features,
    crate: "ex_text_splitter"

  @doc """
  text_splitter("your text here", options)
  where options is a keyword list with these optional params:

  max_tokens: integer,
  min_tokens: integer,
  trim_chunks: bool,
  tokenizer:
  "cl100k_base"
  | "p50k_base"
  | "r50k_base"
  | "p50k_edit"

  using text_splitter a token is a single letter, when using
  tokenizer_text_splitter then it's a real token
  """
  def text_splitter(_arg1, _arg2), do: err()

  if "tiktoken-rs" in @features do
    def tokenizer_text_splitter(_arg1, _arg2), do: err()
  end

  if "markdown" in @features do
    def markdown_splitter(_arg1, _arg2), do: err()
  end

  if "tiktoken-rs" in @features and "markdown" in @features do
    def tokenizer_markdown_splitter(_arg1, _arg2), do: err()
  end

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
