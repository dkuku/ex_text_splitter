defmodule ExTextSplitterTest do
  use ExUnit.Case
  doctest ExTextSplitter

  test "text_splitter" do
    assert ExTextSplitter.Native.text_splitter(
             """
             # Headers
             ## subheader
             1. Really long list item
               1. Some Indented Text
               2. More Indented Text
             """,
             max_tokens: 30,
             trim_chunks: true
           ) ==
             [
               "# Headers\n## subheader",
               "1. Really long list item",
               "1. Some Indented Text",
               "2. More Indented Text"
             ]
  end

  test "markdown_splitter" do
    assert ExTextSplitter.Native.markdown_splitter(
             """
             # Headers
             ## subheader
             1. Really long list item
               1. Some Indented Text
               2. More Indented Text
             """,
             max_tokens: 30,
             trim_chunks: true
           ) ==
             [
               "# Headers",
               "## subheader",
               "1. Really long list item",
               "1. Some Indented Text",
               "2. More Indented Text"
             ]
  end
end
