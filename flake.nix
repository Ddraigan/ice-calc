{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    systems.url = "github:nix-systems/default";
  };

  outputs = {
    nixpkgs,
    systems,
    ...
  }: let
    eachSystem = nixpkgs.lib.genAttrs (import systems);
    pkgsFor = nixpkgs.legacyPackages;
  in {
    devShells = eachSystem (system: let
      pkgs = pkgsFor.${system};
      dlopenLibraries = with pkgs; [
        libxkbcommon

        # GPU backend
        vulkan-loader
        # libGL

        # Window system
        wayland
      ];
    in {
      default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
        ];

        env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${nixpkgs.lib.makeLibraryPath dlopenLibraries}";
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
  };
}
