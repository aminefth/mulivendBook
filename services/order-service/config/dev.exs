import Config

# Configure your database
config :order_service, OrderService.Repo,
  username: "bookmarket",
  password: "bookmarket_dev",
  hostname: "localhost",
  database: "bookmarket",
  stacktrace: true,
  show_sensitive_data_on_connection_error: true,
  pool_size: 10

# For development, we disable any cache and enable
# debugging and code reloading.
config :order_service, OrderServiceWeb.Endpoint,
  http: [ip: {0, 0, 0, 0}, port: 4003],
  check_origin: false,
  code_reloader: true,
  debug_errors: true,
  secret_key_base: "order_service_secret_key_base_for_development_only",
  watchers: []

# Do not include metadata nor timestamps in development logs
config :logger, :console, format: "[$level] $message\n"

# Set a higher stacktrace during development. Avoid configuring such
# in production as building large stacktraces may be expensive.
config :phoenix, :stacktrace_depth, 20

# Initialize plugs at runtime for faster development compilation
config :phoenix, :plug_init_mode, :runtime
