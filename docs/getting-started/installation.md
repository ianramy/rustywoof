# Install the Tool
    
Rustywoof is distributed as a standalone binary and via popular package managers. Select your preferred environment below.

## Choose Your Package Manager

=== ":fontawesome-brands-rust: Cargo"

    Install directly from crates.io. This is recommended if you already have a Rust toolchain configured.

    ```bash {.mac-terminal}
    cargo install rustywoof
    ```

=== ":fontawesome-brands-node-js: Node Ecosystem"

    Rustywoof provides wrappers for all major Node package managers. 
    
    === "npm"
        ```bash {.mac-terminal}
        npm install -g @ianramy/rustywoof
        ```
    === "yarn"
        ```bash {.mac-terminal}
        yarn global add @ianramy/rustywoof
        ```
    === "pnpm"
        ```bash {.mac-terminal}
        pnpm add -g @ianramy/rustywoof
        ```
    === "bun"
        ```bash {.mac-terminal}
        bun install -g @ianramy/rustywoof
        ```
        
## Choose Your Operating System

If you prefer to download pre-compiled binaries, you can use standard OS package managers or scripts.

=== ":fontawesome-brands-apple: macOS"

    Download and execute the installation script.

    ```bash {.mac-terminal}
    curl -sSL https://ianramy.co.ke/rustywoof/installer.sh | sh
    ```

=== ":fontawesome-brands-linux: Linux"

    Download and execute the installation script.

    ```bash {.mac-terminal}
    curl -sSL https://ianramy.co.ke/rustywoof/installer.sh | sh
    ```

=== ":fontawesome-brands-windows: Windows"

    Install via PowerShell.

    ```powershell {.mac-terminal}
    irm https://ianramy.co.ke/rustywoof/installer.ps1 | iex
    ```
    
???+ note "Binary download"

	Alternatively, the installer or binaries can be downloaded directly from [GitHub](https://github.com/ianramy/rustywoof/releases).

???+ example "Verify Installation"

    Once installed, verify the binary is accessible in your path by checking the version.
    ```bash {.mac-terminal}
    rustywoof --version
    ```
