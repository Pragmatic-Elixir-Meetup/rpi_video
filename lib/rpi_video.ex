defmodule RpiVideo do
  def start(mock \\ false), do: impl_mod(mock).start

  defp impl_mod(false), do: RpiVideo.RealServer
  defp impl_mod(true), do: RpiVideo.MockServer
end
