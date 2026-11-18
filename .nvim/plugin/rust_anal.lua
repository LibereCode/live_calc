---NOTE Alternative: put config in `../lsp/rust_analyzer.lua`
---You can pass LSP settings to the server:
vim.lsp.config("rust_analyzer", {
  settings = {
    ["rust-analyzer"] = {
      -- imports = {
      --   granularity = {
      --     group = "module",
      --   },
      --   prefix = "self",
      --   cargo = {
      --     buildScripts = {
      --       enable = true,
      --     },
      --   },
      --   procMacro = {
      --     enable = true,
      --   },
      -- },

      ---NOTE Need to enable rust (and clippy, is default) with devenv first!
      check = {
        command = "clippy",
        extraArgs = {
          "--",
          "-W",
          "clippy::pedantic",
        },
      },
    },
  },
})

vim.lsp.enable({ "rust_analyzer" })
