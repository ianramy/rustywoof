---
hide:
  - navigation
  - toc
  - md-header
  - logo
---

<style>
	/* 1. Transparent Nav */
	.md-header, .md-tabs {
		background-color: transparent !important;
		box-shadow: none !important;
		position: absolute !important;
		top: 0;
		width: 100%;
		z-index: 10;
		opacity: 0;
	}
	
	.md-search__form {
		background-color: rgba(255, 255, 255, 0.1) !important;
		backdrop-filter: blur(4px);
	}
	
	/* 2. Remove default MkDocs backgrounds so our fixed image shows through */
	.md-main__inner, .md-content {
		background-color: transparent !important;
	}
</style>

<div class="tx-hero full-screen-hero">
    <div class="tx-hero__bg" style="background-image: url('assets/hero.png');"></div>
    <div class="tx-hero__content">
        <h1>Enterprise Security Without Compromise</h1>
        <p>A blazing-fast, memory-safe CLI tool designed to detect exposed cryptographic credentials and compromised dependencies before they breach your perimeter.</p>
        <div class="tx-hero__actions">
            <a href="getting-started/" class="md-button md-button--primary">Get started</a>
            <a href="getting-started/what-is-rustywoof/" class="md-button">Learn more</a>
        </div>
    </div>
    <a href="#secure-your-ecosystem" class="scroll-down" aria-label="Scroll down to features">
        <span class="twemoji">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M19 14l-7 7-7-7z"/></svg>
        </span>
    </a>
</div>

<script>
  document.addEventListener("DOMContentLoaded", function() {
      const heroBg = document.querySelector('.tx-hero__bg');
      const heroContent = document.querySelector('.tx-hero__content');
      const scrollArrow = document.querySelector('.scroll-down');
      
      if (heroBg && heroContent) {
          window.addEventListener('scroll', function() {
              const scrollPos = window.scrollY;
              const windowHeight = window.innerHeight;
              
              // Calculate fade (vanishes completely when user scrolls 80% of the screen height)
              let fadeOpacity = 1 - (scrollPos / (windowHeight * 0.8)); 
              if (fadeOpacity < 0) fadeOpacity = 0; 
              
              // Apply fade to background, text, and arrow simultaneously
              heroBg.style.opacity = fadeOpacity.toString();
              heroContent.style.opacity = fadeOpacity.toString();
              if(scrollArrow) scrollArrow.style.opacity = fadeOpacity.toString();
          });
      }
  });
</script>

<div class="landing-content-wrapper" markdown>

## Secure Your Ecosystem {#secure-your-ecosystem}

<div class="grid cards" markdown>

-   :material-regex:{ .lg .middle } __High-Speed Regex__
    ---
    Simultaneous, multi-pattern literal prefix matching using an $O(n)$ Aho-Corasick automaton to scan massive monorepos in milliseconds without crashing CI runners.
-   :material-shield-search:{ .lg .middle } __Live Threat Intelligence__
    ---
    Automatically query the Open Source Vulnerability (OSV) API using JSON batching to audit Node, Rust, and Python lockfiles in under 500ms.
-   :material-wall:{ .lg .middle } __Perimeter Defense__
    ---
    Proactively detect un-tracked `.env` files and manage Git `pre-commit` hooks to stop leaked secrets locally before they ever reach your repository history.
</div>

<br>

## Blazing Fast Installation

Get started in seconds. RustyWoof is distributed as a single, statically linked binary.

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

</div>
