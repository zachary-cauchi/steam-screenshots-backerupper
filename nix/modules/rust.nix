{ inputs, ... }:
{
  imports = [
    inputs.rust-flake.flakeModules.default
    inputs.rust-flake.flakeModules.nixpkgs
  ];
  perSystem =
    { config
    , self'
    , pkgs
    , lib
    , ...
    }:
    let
      bin_name = "steam-screenshots-backerupper";
    in
    {
      rust-project.crates."${bin_name}".crane.args = {
        # On darwin, you may need framework dependencies like IOKit.
        # The default SDK now provides these automatically - no need to specify them.
        # buildInputs = lib.optionals pkgs.stdenv.isDarwin [ pkgs.apple-sdk ];
      };
      packages = {
        # The actual `steam-screenshots-backerupper` tool requires some runtime packages (`u2c`).
        # Symlink the runtime requirements in into the final package.
        default = pkgs.symlinkJoin {
          name = "${bin_name}";
          paths = [ self'.packages."${bin_name}" ];
          nativeBuildInputs = [ pkgs.makeWrapper ];
          postBuild = ''
            ls -lah $out
            wrapProgram $out/bin/${bin_name} \
              --prefix PATH : ${pkgs.lib.makeBinPath [ self'.packages.tool_u2c ]}
          '';
        };
      };
    };
}
