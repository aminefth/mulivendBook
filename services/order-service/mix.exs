defmodule OrderService.MixProject do
  use Mix.Project

  def project do
    [
      app: :order_service,
      version: "0.1.0",
      elixir: "~> 1.12",
      elixirc_paths: elixirc_paths(Mix.env()),
      compilers: [:phoenix] ++ Mix.compilers(),
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      mod: {OrderService.Application, []},
      extra_applications: [:logger, :runtime_tools]
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp deps do
    [
      {:phoenix, "~> 1.6.0"},
      {:phoenix_ecto, "~> 4.4"},
      {:ecto_sql, "~> 3.6"},
      {:postgrex, ">= 0.0.0"},
      {:phoenix_live_dashboard, "~> 0.6"},
      {:telemetry_metrics, "~> 0.6"},
      {:telemetry_poller, "~> 1.0"},
      {:gettext, "~> 0.18"},
      {:jason, "~> 1.2"},
      {:plug_cowboy, "~> 2.5"},
      {:redix, "~> 1.1"},
      {:uuid, "~> 1.1"},
      {:decimal, "~> 2.0"},
      {:httpoison, "~> 1.8"},
      {:joken, "~> 2.4"},
      {:cors_plug, "~> 3.0"},
      {:broadway, "~> 1.0"},
      {:broadway_kafka, "~> 0.3"},
      {:prometheus_ex, "~> 3.0"},
      {:prometheus_plugs, "~> 1.1"}
    ]
  end
end
