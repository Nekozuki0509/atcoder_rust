FROM ubuntu:latest

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y build-essential curl wget git ripgrep fd-find \
    pkg-config libssl-dev unzip ca-certificates sudo nodejs npm mold && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold"

RUN useradd -m -s /bin/bash vscode && \
    echo "vscode ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER vscode
WORKDIR /home/vscode

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/home/vscode/.cargo/bin:${PATH}"

RUN cargo install --locked sccache && \
    cargo install cargo-compete
ENV RUSTC_WRAPPER=/home/vscode/.cargo/bin/sccache
ENV SCCACHE_DIR=/home/vscode/.cache/sccache

RUN rustup default 1.70.0 && rustup component add rust-analyzer

RUN curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim-linux-x86_64.tar.gz && \
    sudo tar -C /opt -xzf nvim-linux-x86_64.tar.gz
ENV PATH="/opt/nvim-linux-x86_64/bin:${PATH}"

RUN LAZYGIT_VERSION=$(curl -s https://api.github.com/repos/jesseduffield/lazygit/releases/latest | grep tag_name | cut -d '"' -f 4) && \
    wget -O lazygit.tar.gz "https://github.com/jesseduffield/lazygit/releases/download/${LAZYGIT_VERSION}/lazygit_${LAZYGIT_VERSION#v}_Linux_x86_64.tar.gz" && \
    tar xf lazygit.tar.gz lazygit && sudo install lazygit /usr/local/bin && \
    sudo npm install -g tree-sitter-cli
RUN git clone https://github.com/LazyVim/starter ~/.config/nvim && rm -rf ~/.config/nvim/.git

RUN mkdir -p ~/.config/nvim/lua/plugins && \
    echo 'return {\n\
        {\n\
            "mrcjkb/rustaceanvim",\n\
            version = "^5",\n\
            ft = { "rust" },\n\
            config = function()\n\
            vim.g.rustaceanvim = {\n\
                server = {\n\
                on_attach = function(_, bufnr)\n\
                    local opts = { buffer = bufnr }\n\
                    vim.keymap.set("n", "<leader>rr", function()\n\
                    vim.cmd.RustLsp("runnables")\n\
                    end, opts)\n\
                end,\n\
                },\n\
            }\n\
            end,\n\
        },\n\
    }\n' > ~/.config/nvim/lua/plugins/rust.lua

RUN nvim --headless "+Lazy! sync" +qa || true

ENV PATH="/workspaces/atcoder/commands:${PATH}"

WORKDIR /workspaces/atcoder
