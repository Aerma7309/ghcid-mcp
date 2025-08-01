{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    git-hooks-nix.url = "github:cachix/git-hooks.nix";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.git-hooks-nix.flakeModule
      ];
      systems = import inputs.systems;
      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        {

          pre-commit = {
            check.enable = true;
            settings = {
              hooks = {
                treefmt.enable = true;
                # Optional: Rust linting
                # clippy = {
                #   enable = true;
                #   description = "Lint Rust code.";
                #   entry = "${pkgs.clippy}/bin/cargo-clippy clippy -- -D warnings";
                #   files = "\\.rs$";
                #   pass_filenames = false;
                # };
              };
            };
          };

          devShells.default = pkgs.mkShell {
            name = "rpi-sms-reader";
            inputsFrom = [
              config.pre-commit.devShell
            ];
            packages =
              with pkgs;
              [
                rustup
                cargo-watch
                nixfmt-rfc-style
                pkg-config
                openssl
                libiconv
                treefmt
                taplo
              ]
              ++ lib.optionals pkgs.stdenv.isDarwin [
                pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
              ];
          };
        };
    };
}
