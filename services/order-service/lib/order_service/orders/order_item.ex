defmodule OrderService.Orders.OrderItem do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id

  schema "order_items" do
    field :product_id, :binary_id
    field :product_title, :string
    field :product_isbn, :string
    field :price, :decimal
    field :quantity, :integer
    field :total_price, :decimal

    belongs_to :order, OrderService.Orders.Order

    timestamps()
  end

  @doc false
  def changeset(order_item, attrs) do
    order_item
    |> cast(attrs, [:product_id, :product_title, :product_isbn, :price, :quantity, :total_price])
    |> validate_required([:product_id, :product_title, :price, :quantity])
    |> validate_number(:price, greater_than: 0)
    |> validate_number(:quantity, greater_than: 0)
    |> calculate_total_price()
  end

  defp calculate_total_price(changeset) do
    case {get_field(changeset, :price), get_field(changeset, :quantity)} do
      {price, quantity} when is_number(price) and is_integer(quantity) ->
        total = Decimal.mult(Decimal.new(price), Decimal.new(quantity))
        put_change(changeset, :total_price, total)
      _ ->
        changeset
    end
  end
end
