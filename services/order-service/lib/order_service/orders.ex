defmodule OrderService.Orders do
  @moduledoc """
  The Orders context.
  """

  import Ecto.Query, warn: false
  alias OrderService.Repo
  alias OrderService.Orders.{Order, OrderItem, OrderStatus}

  @doc """
  Returns the list of orders.
  """
  def list_orders(user_id, opts \\ []) do
    page = Keyword.get(opts, :page, 1)
    per_page = Keyword.get(opts, :per_page, 20)
    status = Keyword.get(opts, :status)

    query = from o in Order,
      where: o.user_id == ^user_id,
      order_by: [desc: o.inserted_at],
      preload: [:order_items]

    query = if status do
      from o in query, where: o.status == ^status
    else
      query
    end

    Repo.paginate(query, page: page, page_size: per_page)
  end

  @doc """
  Gets a single order.
  """
  def get_order!(id), do: Repo.get!(Order, id) |> Repo.preload([:order_items])

  @doc """
  Gets a single order by user.
  """
  def get_user_order(id, user_id) do
    from(o in Order, where: o.id == ^id and o.user_id == ^user_id)
    |> Repo.one()
    |> case do
      nil -> {:error, :not_found}
      order -> {:ok, Repo.preload(order, [:order_items])}
    end
  end

  @doc """
  Creates an order.
  """
  def create_order(attrs \\ %{}) do
    %Order{}
    |> Order.changeset(attrs)
    |> Repo.insert()
    |> case do
      {:ok, order} ->
        # Publish order created event
        publish_event("order.created", order)
        {:ok, order}
      error -> error
    end
  end

  @doc """
  Updates an order.
  """
  def update_order(%Order{} = order, attrs) do
    old_status = order.status
    
    order
    |> Order.changeset(attrs)
    |> Repo.update()
    |> case do
      {:ok, updated_order} ->
        # Publish status change event if status changed
        if updated_order.status != old_status do
          publish_event("order.status_changed", %{
            order: updated_order,
            old_status: old_status,
            new_status: updated_order.status
          })
        end
        {:ok, updated_order}
      error -> error
    end
  end

  @doc """
  Deletes an order.
  """
  def delete_order(%Order{} = order) do
    Repo.delete(order)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking order changes.
  """
  def change_order(%Order{} = order, attrs \\ %{}) do
    Order.changeset(order, attrs)
  end

  @doc """
  Calculates order total including tax and shipping.
  """
  def calculate_total(order_items, shipping_cost \\ Decimal.new("0.00"), tax_rate \\ Decimal.new("0.10")) do
    subtotal = Enum.reduce(order_items, Decimal.new("0.00"), fn item, acc ->
      item_total = Decimal.mult(item.price, Decimal.new(item.quantity))
      Decimal.add(acc, item_total)
    end)
    
    tax_amount = Decimal.mult(subtotal, tax_rate)
    total = subtotal |> Decimal.add(tax_amount) |> Decimal.add(shipping_cost)
    
    %{
      subtotal: subtotal,
      tax_amount: tax_amount,
      shipping_cost: shipping_cost,
      total: total
    }
  end

  @doc """
  Processes payment for an order.
  """
  def process_payment(order_id, payment_method, payment_details) do
    with {:ok, order} <- get_user_order(order_id, order.user_id),
         {:ok, payment_result} <- call_payment_service(order, payment_method, payment_details),
         {:ok, updated_order} <- update_order(order, %{
           status: :paid,
           payment_id: payment_result.payment_id,
           payment_method: payment_method
         }) do
      {:ok, updated_order}
    else
      error -> error
    end
  end

  defp call_payment_service(order, payment_method, payment_details) do
    # Mock payment service call
    # In production, this would call the actual payment service
    payment_id = UUID.uuid4()
    
    # Simulate payment processing
    case payment_method do
      "credit_card" -> {:ok, %{payment_id: payment_id, status: "completed"}}
      "paypal" -> {:ok, %{payment_id: payment_id, status: "completed"}}
      _ -> {:error, "Unsupported payment method"}
    end
  end

  defp publish_event(event_type, data) do
    Phoenix.PubSub.broadcast(OrderService.PubSub, "orders", {event_type, data})
    
    # Also send to external message queue (Kafka/RabbitMQ)
    # This would be implemented with Broadway producers
  end
end
