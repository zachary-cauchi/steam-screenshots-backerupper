{ inputs, ... }:
{
  perSystem =
    { config
    , self'
    , pkgs
    , ...
    }:
    let
      u2c_pkg_name = "tool_u2c";
      u2c = pkgs.stdenv.mkDerivation {
        name = u2c_pkg_name;
        src = "${inputs.copyparty}/bin/u2c.py";
        dontUnpack = true;
        buildInputs = [ pkgs.python3 ];
        installPhase = ''
          mkdir -p $out/bin
          cp $src $out/bin/${u2c_pkg_name}
          chmod +x $out/bin/${u2c_pkg_name}
          patchShebangs $out/bin/${u2c_pkg_name}
        '';
      };
    in
    {
      devShells.default = pkgs.mkShell {
        name = "steam-screenshots-backerupper-shell";
        inputsFrom = [
          self'.devShells.rust
          config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
        ];
        packages = with pkgs; [
          just
          nixd # Nix language server
          haskell-language-server
          bacon
          self'.packages.tool_u2c
        ];
      };
    };
}
