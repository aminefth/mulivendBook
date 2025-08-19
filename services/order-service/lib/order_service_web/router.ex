defmodule OrderServiceWeb.Router do
  use OrderServiceWeb, :router

  pipeline :api do
    plug :accepts, ["json"]
    plug OrderServiceWeb.Auth
  end

  pipeline :public do
    plug :accepts, ["json"]
  end

  scope "/", OrderServiceWeb do
    pipe_through :public

    get "/health", HealthController, :health
    get "/ready", HealthController, :ready
    get "/metrics", MetricsController, :metrics
  end

  scope "/api/v1", OrderServiceWeb do
    pipe_through :api

    resources "/orders", OrderController, except: [:new, :edit] do
      post "/payment", OrderController, :process_payment
      post "/cancel", OrderController, :cancel
      get "/track", OrderController, :track
    end

    get "/orders/user/:user_id", OrderController, :user_orders
  end

  # Enable LiveDashboard in development
  if Mix.env() in [:dev, :test] do
    import Phoenix.LiveDashboard.Router

    scope "/" do
      pipe_through [:fetch_session, :protect_from_forgery]
      live_dashboard "/dashboard", metrics: OrderServiceWeb.Telemetry
    end
  end
end
