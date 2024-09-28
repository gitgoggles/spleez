{
  description = "Spleez";

  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust-default = pkgs.rust-bin.stable.latest.default;
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      in
        with pkgs; {
          packages.default = rustPlatform.buildRustPackage {
            pname = manifest.name;
            version = manifest.version;
            cargoLock.lockFile = ./Cargo.lock;
            src = lib.cleanSource ./.;

            nativeBuildInputs = [
              rust-default
              tailwindcss
            ];
          };

          devShells.default = mkShell {
            packages = [
              cargo-watch
              rust-default
              tailwindcss
            ];

            shellHook = ''
              alias watchrun='cargo watch -c -q -x run'

              echo "Default devShell: ${manifest.name}"

            '';
          };
        }
    );
}
