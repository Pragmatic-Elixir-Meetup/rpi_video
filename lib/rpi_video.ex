defmodule RpiVideo do
  def start(mock \\ false), do: impl_mod(mock).start
  def stop(mock \\ false), do: impl_mod(mock).stop

  def record(mock \\ false), do: impl_mod(mock).record

  defp impl_mod(false), do: RpiVideo.RealServer
  defp impl_mod(true), do: RpiVideo.MockServer
end
