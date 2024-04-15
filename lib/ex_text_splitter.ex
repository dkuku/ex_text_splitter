defmodule ExTextSplitter do
  @moduledoc """
  Documentation for `ExTextSplitter`.
  """
end

defmodule ExTextSplitter.Native do
  @moduledoc false
  use Rustler,
    otp_app: :ex_text_splitter,
    crate: "ex_text_splitter"

  def text_splitter(_arg1, _arg2), do: err()
  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
