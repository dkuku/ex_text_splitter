defmodule ExTextSplitter.MixProject do
  use Mix.Project

  def project do
    [
      app: :ex_text_splitter,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      package: package(),
      description: description()
    ]
  end

  defp description do
    "ex_text_splitter wraps the rust crate text-splitter. It allows to divide text into chunks"
  end

  defp package do
    [
      # These are the default files included in the package
      files: [
        "lib",
        "mix.exs",
        "native/ex_text_splitter/.cargo",
        "native/ex_text_splitter/src",
        "native/ex_text_splitter/Cargo*",
        "README*"
      ],
      licenses: ["MIT"],
      links: %{
        "GitHub" => "https://github.com/dkuku/ex_text_splitter",
        "Crate GitHub" => "https://github.com/benbrandt/text-splitter",
        "Crate Documentation" => "https://crates.io/crates/text-splitter"
      }
    ]
  end

  def application do
    []
  end

  defp deps do
    [
      {:rustler, "~> 0.32.1", runtime: false},
      {:ex_doc, "~> 0.31", only: :dev, runtime: false}
    ]
  end
end
