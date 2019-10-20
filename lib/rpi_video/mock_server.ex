defmodule RpiVideo.MockServer do
  use GenServer

  def start do
    GenServer.start_link(__MODULE__, %{}, name: __MODULE__)
  end

  def stop do
    GenServer.stop(__MODULE__)
  end

  def record do
    __MODULE__
    |> GenServer.whereis()
    |> start_record_on()
  end

  @impl true
  def init(%{}) do
    executable = :code.priv_dir(:rpi_video) ++ '/mock_rpi_video'

    port =
      Port.open({:spawn_executable, executable}, [
        :binary,
        :exit_status,
        :use_stdio
      ])

    state = %{port: port}

    {:ok, state}
  end

  @impl true
  def handle_cast({:elixir_start_record, nil}, %{port: port} = state) do
    data = :erlang.term_to_binary("elixir_start_record")
    len = byte_size(data)

    buf = [<<len::big-unsigned-integer-size(64)>>, data]
    Port.command(port, buf)

    IO.puts(:stderr, "elixir_MOCK_server: Starts to record a new video")

    {:noreply, state}
  end

  @impl true
  def handle_info({_port, {:data, <<data::binary>>}}, state) do
    file_path = :erlang.binary_to_term(data)

    IO.puts(:stderr, "elixir_MOCK_server: Finishes recording the video - `#{file_path}`")

    {:noreply, state}
  end

  @impl true
  def handle_info({_, {:exit_status, status}}, state) do
    {:stop, {:exit, status}, state}
  end

  defp start_record_on(pid) when is_pid(pid),
  do: GenServer.cast(pid, {:elixir_start_record, nil})
  defp start_record_on(nil),
  do: raise("#{__MODULE__} has not been started")
end
