defmodule ExTextSplitterTest do
  use ExUnit.Case
  doctest ExTextSplitter

  test "add" do
    assert ExTextSplitter.Native.text_splitter(
             """
             defmodule ExTextSplitterTest do
              use ExUnit.Case
              doctest ExTextSplitter

              test "add" do
                assert ExTextSplitter.Native.add(1, 1) == 2
              end
             end
             """,
             max_tokens: 20,
             trim_chunks: true
           ) ==
             [
               "defmodule",
               "ExTextSplitterTest",
               "do\n use ExUnit.Case",
               "doctest",
               "ExTextSplitter",
               "test \"add\" do",
               "assert",
               "ExTextSplitter.Nativ",
               "e.add(1, 1) == 2",
               "end\nend"
             ]
  end
end
