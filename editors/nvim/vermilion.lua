-- SPDX-License-Identifier: BSD-3-Clause
---@brief
---
--- https://github.com/shrine-maiden-heavy-industries/Vermilion
---
--- Verilog/Verilog-AMS/SystemVerilog/VHDL Language Server, Linter, and Formatter
---

---@type vim.lsp.Config
return {
	cmd = { 'vermilion', 'lang-server' },
	filetypes = { 'verilog', 'system-verilog', 'vhdl' },
	root_markers = { 'vermilion.toml' }
}
