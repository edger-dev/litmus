{
  description = "litmus — terminal color theme previewer";

  inputs = {
    jig.url = "github:edger-dev/jig";
  };

  outputs = { self, jig }:
    jig.lib.mkWorkspace
      {
        pname = "litmus";
        src = ./.;
        extraDevPackages = pkgs: [
          pkgs.dioxus-cli
          pkgs.wasm-bindgen-cli
          pkgs.cage             # Wayland kiosk compositor for headless capture
          pkgs.grim             # Wayland screenshot tool
          pkgs.wlr-randr        # Wayland output configuration (for setting headless resolution)
          pkgs.foot             # Wayland terminal (SHM-based, no OpenGL needed)
          pkgs.kitty            # Kitty terminal (requires real display with OpenGL)
          pkgs.fira-code
          pkgs.rclone           # S3-compatible sync for R2 screenshot uploads
        ];
      }
      {
        rust = { buildPackages = [ "litmus-cli" ]; wasm = true; };
        docs = { beans = true; };
      };
}
