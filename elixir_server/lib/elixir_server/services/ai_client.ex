defmodule ElixirServer.Services.AiClient do
  def generate_content(prompt) do
    url =
      "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-lite:generateContent"

    api_key = "API_KEY_GOES_HERE"

    # Construct the request URL with query parameter
    request_url = "#{url}?key=#{api_key}"

    # Prepare headers
    headers = [{"Content-Type", "application/json"}]

    # Prepare the request body
    body =
      Jason.encode!(%{
        contents: [
          %{
            parts: [
              %{text: "generate a shader code for #{prompt}"}
            ]
          }
        ]
      })

    # Make the POST request
    case HTTPoison.post(request_url, body, headers, timeout: 50_000, recv_timeout: 50_000) do
      {:ok, %HTTPoison.Response{status_code: 200, body: response_body}} ->
        {:ok, Jason.decode!(response_body)}

      {:ok, %HTTPoison.Response{status_code: status_code, body: response_body}} ->
        {:error, "API error: #{status_code}, #{response_body}"}

      {:error, %HTTPoison.Error{reason: reason}} ->
        {:error, "Request failed: #{reason}"}
    end
  end
end
