defmodule OrderService.Application do
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      # Start the Ecto repository
      OrderService.Repo,
      # Start the Telemetry supervisor
      OrderServiceWeb.Telemetry,
      # Start the PubSub system
      {Phoenix.PubSub, name: OrderService.PubSub},
      # Start the Endpoint (http/https)
      OrderServiceWeb.Endpoint,
      # Start Redis connection
      {Redix, host: redis_host(), port: redis_port(), name: :redix},
      # Start Broadway for event processing
      OrderService.EventProcessor
    ]

    opts = [strategy: :one_for_one, name: OrderService.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    OrderServiceWeb.Endpoint.config_change(changed, removed)
    :ok
  end

  defp redis_host do
    System.get_env("REDIS_HOST", "localhost")
  end

  defp redis_port do
    System.get_env("REDIS_PORT", "6379") |> String.to_integer()
  end
end
