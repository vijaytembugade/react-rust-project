defmodule ElixirServerWeb.ApiController do
  use ElixirServerWeb, :controller
  alias ElixirServer.Services.AiClient

  def hello(conn, params) do
    name = Map.get(params, "name", "World")
    json(conn, %{message: "Hello, #{name}!"})
  end

  def generate(conn, %{"query" => query}) when is_binary(query) and query != "" do
    case AiClient.generate_content(query) do
      {:ok, response} ->
        json(conn, response)

      {:error, message} ->
        conn
        |> put_status(:internal_server_error)
        |> json(%{error: message})
    end
  end
end
