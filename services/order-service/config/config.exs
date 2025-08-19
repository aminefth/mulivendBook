import Config

config :order_service,
  ecto_repos: [OrderService.Repo]

config :order_service_web,
  generators: [context_app: :order_service]

config :order_service, OrderServiceWeb.Endpoint,
  url: [host: "localhost"],
  render_errors: [view: OrderServiceWeb.ErrorView, accepts: ~w(json), layout: false],
  pubsub_server: OrderService.PubSub,
  live_view: [signing_salt: "order_service"]

config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

config :phoenix, :json_library, Jason

import_config "#{config_env()}.exs"
