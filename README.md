# ExTextSplitter

This package provides bindings to [text-splitter](https://github.com/benbrandt/text-splitter) crate
By default only the text_splitter function is available but you can configure the available features:

```elixir
# this will enable all features
config :ex_text_splitter,
  features: ["markdown", "tiktoken-rs"]
```

This can be also configured using Mix.installed
```elixir
Mix.install(
    [:ex_text_splitter],
    config: [ex_text_splitter: [features: ["markdown"]]]
)
```
<!-- MDOC -->
## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `ex_text_splitter` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:ex_text_splitter, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/ex_text_splitter>.

