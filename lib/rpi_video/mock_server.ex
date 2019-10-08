defmodule RpiVideo.MockServer do
  use GenServer

  def start do
    GenServer.start_link(__MODULE__, %{})
  end

  def stop do
    GenServer.stop(__MODULE__)
  end

  @impl true
  def init(%{}) do
    executable = :code.priv_dir(:rpi_video) ++ '/mock_rpi_video'

    port =
      Port.open({:spawn_executable, executable}, [
        {:packet, 2},
        :binary,
        :exit_status,
        :use_stdio
      ])

    state = %{port: port}

    {:ok, state}
  end

  @impl true
  def handle_info({_port, {:start, <<data::binary>>}}, state) do
    file_path = :erlang.binary_to_term(data)

    IO.puts("elixir_MOCK_server: Starts to record video - #{file_path}")

    {:noreply, state}
  end

  @impl true
  def handle_info({_port, {:end, <<data::binary>>}}, state) do
    file_path = :erlang.binary_to_term(data)

    IO.puts("elixir_MOCK_server: Finishes recording video - #{file_path}")

    {:noreply, state}
  end

  @impl true
  def handle_info({_, {:exit_status, status}}, state) do
    {:stop, {:exit, status}, state}
  end
end
