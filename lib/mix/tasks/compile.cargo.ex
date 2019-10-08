defmodule Mix.Tasks.Compile.Cargo do
  use Mix.Task

  @shortdoc "Builds Rust part."

  def run(args) do
    Mix.shell().print_app()

    target = parse_target(args)
    build(target)
    copy(target)
  end

  defp bin_filename(:mac), do: "mock_rpi_video"
  defp bin_filename(:rpi), do: "real_rpi_video"

  defp build(target) do
    System.cmd("cargo", build_cmd_args(target),
      stderr_to_stdout: true,
      into: IO.stream(:stdio, :line)
    )
  end

  defp build_cmd_args(target) do
    [
      "build",
      "--bin",
      bin_filename(target),
      "--release",
      "--target-dir",
      priv_path()
    ]
  end

  defp copy(target) do
    System.cmd("cp", copy_cmd_args(target),
      stderr_to_stdout: true,
      into: IO.stream(:stdio, :line)
    )
  end

  defp copy_cmd_args(target) do
    priv = priv_path()

    [
      Path.join([priv, "release", bin_filename(target)]),
      priv
    ]
  end

  defp current_env, do: Application.get_env(:rpi_video, :env)

  defp os_target({:unix, :darwin}), do: :mac
  defp os_target({:unix, :linux}), do: :rpi

  defp os_target(_) do
    raise(
      "Specifies target via `mix.compile.cargo --target=mac` or `mix.compile.cargo --target=rpi`"
    )
  end

  defp parse_target(args) do
    arg_target =
      OptionParser.parse(args, strict: [target: :string])
      |> elem(0)
      |> Keyword.get(:target)

    if arg_target do
      String.to_atom(String.downcase(arg_target))
    else
      os_target(:os.type())
    end
  end

  defp priv_path do
    "_build/#{current_env()}/lib/rpi_video/priv"
  end
end
