{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
  };
  outputs =
    {
      self,
      flake-utils,
      naersk,
      nixpkgs,
      fenix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        port = "8080";
        address = "0.0.0.0";
        pkgsOverlays = (final: prev: { });
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [
            fenix.overlays.default
            pkgsOverlays
          ];
          config = {
            allowUnfree = true;
            cudaSupport = true;
          };
        };
        naersk' = pkgs.callPackage naersk { };
        rustPackage = naersk'.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [
            pkg-config
            openssl
            cudaPackages.cuda_nvcc
            cudaPackages.cudatoolkit
          ];
          buildInputs = with pkgs; [ ];

          CUDA_ROOT = "${pkgs.cudaPackages.cudatoolkit}";
          CUDA_COMPUTE_CAP = "89";

          LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:$CUDA_HOME/lib64";
          CUDA_PATH = "${pkgs.cudaPackages.cudatoolkit}";
          EXTRA_LDFLAGS = "-L${pkgs.linuxPackages.nvidia_x11}/lib";
          EXTRA_CCFLAGS = "-I${pkgs.cudaPackages.cudatoolkit}/include";

          # CARGO_TARGET_DIR = "$out/target";
          # RUST_BACKTRACE = "1";
          # RUST_LOG = "debug";
        };
        drvInfo = builtins.parseDrvName rustPackage.name;
        packageName = drvInfo.name;
        packageVersion = drvInfo.version;
        dockerImage = pkgs.dockerTools.buildImage {
          name = packageName;
          tag = packageVersion;
          created = "now";
          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [ rustPackage ];
          };
          config = {
            Cmd = [
              ("/bin/" + packageName)
              ""
            ];
            volumes = {
              "/cache" = { };
            };
            Env = [
              "BIND_ADDRESS=${address}"
              "BIND_PORT=${port}"
              "RUST_LOG=info,cached_path=info,actix_server=warn,hyper=warn,reqwest=warn"
              "RUST_LOG_STYLE=always"
              "CACHE_DIR=/cache/"
            ];
            ExposedPorts = {
              "${port}/tcp" = { };
            };
          };
        };
      in
      {
        defaultPackage = rustPackage;
        packages = {
          inherit rustPackage;
          docker = dockerImage;
        };
        apps.docker =
          let
            script = pkgs.writeShellScriptBin "run-docker" (builtins.readFile ./scripts/run-docker.sh);
          in
          {
            type = "app";
            program = "${script}/bin/run-docker";
            args = [
              "${packageName}"
              "${packageVersion}"
              "${port}"
              "$CACHE_DIR"
            ];
          };
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            alejandra
            rust-analyzer
            pkg-config
            openssl
            cudaPackages.cuda_nvcc
            cudaPackages.cudatoolkit
            linuxPackages.nvidia_x11
            libGLU
            libGL
            marp-cli
            figlet
            onefetch
            (pkgs.fenix.stable.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
          ];
          shellHook = ''
            export CUDA_ROOT=${pkgs.cudaPackages.cudatoolkit}
            export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$CUDA_HOME/lib64
            export CUDA_PATH=${pkgs.cudaPackages.cudatoolkit}
            export EXTRA_LDFLAGS="-L${pkgs.linuxPackages.nvidia_x11}/lib"
            export EXTRA_CCFLAGS="-I${pkgs.cudaPackages.cudatoolkit}/include"
            figlet "Redactor"
            echo 
            echo "Development environment activated!"
            echo "=================================="
            make check-env
            echo 
            echo "start with make help"
          '';
        };
      }
    );
}
