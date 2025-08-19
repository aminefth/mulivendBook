defmodule OrderServiceWeb.OrderController do
  use OrderServiceWeb, :controller

  alias OrderService.Orders
  alias OrderService.Orders.Order

  action_fallback OrderServiceWeb.FallbackController

  def index(conn, params) do
    user_id = get_user_id(conn)
    page = Map.get(params, "page", "1") |> String.to_integer()
    per_page = Map.get(params, "per_page", "20") |> String.to_integer()
    status = Map.get(params, "status")

    orders = Orders.list_orders(user_id, page: page, per_page: per_page, status: status)
    render(conn, "index.json", orders: orders)
  end

  def create(conn, %{"order" => order_params}) do
    user_id = get_user_id(conn)
    
    order_params = Map.put(order_params, "user_id", user_id)
    |> Map.put("order_number", Order.generate_order_number())
    |> Map.put("status", "pending")

    with {:ok, %Order{} = order} <- Orders.create_order(order_params) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", Routes.order_path(conn, :show, order))
      |> render("show.json", order: order)
    end
  end

  def show(conn, %{"id" => id}) do
    user_id = get_user_id(conn)
    
    case Orders.get_user_order(id, user_id) do
      {:ok, order} -> render(conn, "show.json", order: order)
      {:error, :not_found} -> {:error, :not_found}
    end
  end

  def update(conn, %{"id" => id, "order" => order_params}) do
    user_id = get_user_id(conn)
    
    with {:ok, order} <- Orders.get_user_order(id, user_id),
         {:ok, %Order{} = order} <- Orders.update_order(order, order_params) do
      render(conn, "show.json", order: order)
    end
  end

  def delete(conn, %{"id" => id}) do
    user_id = get_user_id(conn)
    
    with {:ok, order} <- Orders.get_user_order(id, user_id),
         {:ok, %Order{}} <- Orders.delete_order(order) do
      send_resp(conn, :no_content, "")
    end
  end

  def process_payment(conn, %{"order_id" => order_id, "payment" => payment_params}) do
    user_id = get_user_id(conn)
    payment_method = Map.get(payment_params, "method")
    payment_details = Map.get(payment_params, "details", %{})

    with {:ok, order} <- Orders.get_user_order(order_id, user_id),
         {:ok, updated_order} <- Orders.process_payment(order_id, payment_method, payment_details) do
      render(conn, "show.json", order: updated_order)
    end
  end

  def cancel(conn, %{"order_id" => order_id}) do
    user_id = get_user_id(conn)
    
    with {:ok, order} <- Orders.get_user_order(order_id, user_id),
         {:ok, updated_order} <- Orders.update_order(order, %{status: "cancelled"}) do
      render(conn, "show.json", order: updated_order)
    end
  end

  def track(conn, %{"order_id" => order_id}) do
    user_id = get_user_id(conn)
    
    case Orders.get_user_order(order_id, user_id) do
      {:ok, order} -> 
        tracking_info = %{
          order_number: order.order_number,
          status: order.status,
          shipped_at: order.shipped_at,
          delivered_at: order.delivered_at,
          tracking_events: get_tracking_events(order)
        }
        json(conn, %{tracking: tracking_info})
      {:error, :not_found} -> {:error, :not_found}
    end
  end

  def user_orders(conn, %{"user_id" => requested_user_id} = params) do
    current_user_id = get_user_id(conn)
    
    # Only allow users to see their own orders unless admin
    if current_user_id == requested_user_id or is_admin?(conn) do
      page = Map.get(params, "page", "1") |> String.to_integer()
      per_page = Map.get(params, "per_page", "20") |> String.to_integer()
      
      orders = Orders.list_orders(requested_user_id, page: page, per_page: per_page)
      render(conn, "index.json", orders: orders)
    else
      conn
      |> put_status(:forbidden)
      |> json(%{error: "Access denied"})
    end
  end

  defp get_user_id(conn) do
    conn.assigns[:current_user_id]
  end

  defp is_admin?(conn) do
    conn.assigns[:user_role] == "admin"
  end

  defp get_tracking_events(order) do
    # Mock tracking events - in production this would come from shipping provider
    base_events = [
      %{
        status: "pending",
        description: "Order placed",
        timestamp: order.inserted_at
      }
    ]

    case order.status do
      :confirmed -> 
        base_events ++ [%{status: "confirmed", description: "Order confirmed", timestamp: order.updated_at}]
      :paid -> 
        base_events ++ [
          %{status: "confirmed", description: "Order confirmed", timestamp: order.updated_at},
          %{status: "paid", description: "Payment processed", timestamp: order.updated_at}
        ]
      :shipped -> 
        base_events ++ [
          %{status: "confirmed", description: "Order confirmed", timestamp: order.updated_at},
          %{status: "paid", description: "Payment processed", timestamp: order.updated_at},
          %{status: "shipped", description: "Order shipped", timestamp: order.shipped_at}
        ]
      :delivered -> 
        base_events ++ [
          %{status: "confirmed", description: "Order confirmed", timestamp: order.updated_at},
          %{status: "paid", description: "Payment processed", timestamp: order.updated_at},
          %{status: "shipped", description: "Order shipped", timestamp: order.shipped_at},
          %{status: "delivered", description: "Order delivered", timestamp: order.delivered_at}
        ]
      _ -> base_events
    end
  end
end
