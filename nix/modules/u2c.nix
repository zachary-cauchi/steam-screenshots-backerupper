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
    in
    {
      # A package wrapping the copyparty-supplied `u2c` cli tool for uploading files to a copyparty server.
      packages.tool_u2c = pkgs.stdenv.mkDerivation {
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
    };
}
