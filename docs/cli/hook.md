# Manage Git Hooks (hook)

The `woof hook` command manages Rustywoof's native Git integration, providing proactive perimeter defense directly at the developer's workstation.

## Deploy or Remove the Guard

=== "Install Hook"

    ```bash {.mac-terminal}
    woof hook install
    ```
    
    Creates or modifies the `.git/hooks/pre-commit` file in your local repository. Once installed, every `git commit` command triggers a lightning-fast micro-sweep of your staged files.

=== "Remove Hook"

    ```bash {.mac-terminal}
    woof hook remove
    ```
    
    Safely detaches the Rustywoof guard from your Git lifecycle without affecting other hooks you may have installed.

???+ warning "Local Scope Limitation"
    This command only installs the hook in your *local* repository clone. To enforce this across your entire team automatically, you must use a centralized tool like Husky (Node) or the Python `pre-commit` framework to orchestrate the hooks upon repository cloning.
