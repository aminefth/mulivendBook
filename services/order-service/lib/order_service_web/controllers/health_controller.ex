defmodule OrderServiceWeb.HealthController do
  use OrderServiceWeb, :controller

  def health(conn, _params) do
    json(conn, %{
      status: "healthy",
      service: "order-service",
      version: "1.0.0",
      timestamp: DateTime.utc_now()
    })
  end

  def ready(conn, _params) do
    # Check database connectivity
    case OrderService.Repo.query("SELECT 1") do
      {:ok, _} ->
        # Check Redis connectivity
        case Redix.command(:redix, ["PING"]) do
          {:ok, "PONG"} ->
            json(conn, %{
              status: "ready",
              checks: %{
                database: "healthy",
                redis: "healthy"
              },
              timestamp: DateTime.utc_now()
            })
          _ ->
            conn
            |> put_status(503)
            |> json(%{
              status: "not_ready",
              error: "Redis connection failed"
            })
        end
      _ ->
        conn
        |> put_status(503)
        |> json(%{
          status: "not_ready",
          error: "Database connection failed"
        })
    end
  end
end
