import Config

config :order_service, OrderService.Repo,
  url: System.get_env("DATABASE_URL") || "postgres://bookmarket:bookmarket123@localhost:5432/bookmarket",
  pool_size: String.to_integer(System.get_env("POOL_SIZE") || "10"),
  ssl: true

config :order_service, OrderServiceWeb.Endpoint,
  http: [
    port: String.to_integer(System.get_env("PORT") || "4003"),
    transport_options: [socket_opts: [:inet6]]
  ],
  secret_key_base: System.get_env("SECRET_KEY_BASE") || "order_service_secret_key_base",
  server: true

config :logger, level: :info
