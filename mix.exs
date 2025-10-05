defmodule GmailStripper.MixProject do
  use Mix.Project

  def project do
    [
      app: :gmail_stripper,
      version: "0.1.2",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      description: description(),
      package: package(),
      deps: deps(),
      name: "GmailStripper",
      source_url: "https://github.com/your-username/gmail_stripper"
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      # {:ex_doc, "~> 0.27", only: :dev, runtime: false}
    ]
  end

  defp description() do
    "A library to normalize Gmail email addresses by removing dots and plus-suffixes to prevent duplicate signups."
  end

  defp package() do
    [
      licenses: ["BSD-3-Clause"],
      links: %{"GitHub" => "https://github.com/your-username/gmail_stripper"}
    ]
  end
end