defmodule ExTextSplitter do
  @moduledoc """
  Documentation for `ExTextSplitter`.
  """
end

defmodule ExTextSplitter.Native do
  @moduledoc false
  @features Application.compile_env(:ex_text_splitter, :features, []) |> List.wrap()

  use Rustler,
    otp_app: :ex_text_splitter,
    features: @features,
    crate: "ex_text_splitter"

  def text_splitter(_arg1, _arg2), do: err()

  if "markdown" in @features do
    def markdown_splitter(_arg1, _arg2), do: err()
  end

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
