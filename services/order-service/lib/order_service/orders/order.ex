defmodule OrderService.Orders.Order do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id

  schema "orders" do
    field :user_id, :binary_id
    field :vendor_id, :binary_id
    field :order_number, :string
    field :status, Ecto.Enum, values: [:pending, :confirmed, :paid, :processing, :shipped, :delivered, :cancelled, :refunded]
    field :subtotal, :decimal
    field :tax_amount, :decimal
    field :shipping_cost, :decimal
    field :total_amount, :decimal
    field :currency, :string, default: "USD"
    field :payment_method, :string
    field :payment_id, :string
    field :shipping_address, :map
    field :billing_address, :map
    field :notes, :string
    field :shipped_at, :utc_datetime
    field :delivered_at, :utc_datetime

    has_many :order_items, OrderService.Orders.OrderItem
    
    timestamps()
  end

  @doc false
  def changeset(order, attrs) do
    order
    |> cast(attrs, [
      :user_id, :vendor_id, :order_number, :status, :subtotal, :tax_amount,
      :shipping_cost, :total_amount, :currency, :payment_method, :payment_id,
      :shipping_address, :billing_address, :notes, :shipped_at, :delivered_at
    ])
    |> validate_required([:user_id, :order_number, :status, :total_amount])
    |> validate_inclusion(:status, [:pending, :confirmed, :paid, :processing, :shipped, :delivered, :cancelled, :refunded])
    |> validate_number(:total_amount, greater_than: 0)
    |> unique_constraint(:order_number)
    |> cast_assoc(:order_items, with: &OrderService.Orders.OrderItem.changeset/2)
  end

  def generate_order_number do
    timestamp = DateTime.utc_now() |> DateTime.to_unix()
    random = :rand.uniform(9999) |> Integer.to_string() |> String.pad_leading(4, "0")
    "ORD-#{timestamp}-#{random}"
  end
end
