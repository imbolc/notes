-- Lua in Neovim

-- Tables
local lang = {
	name = "Lua",
	greet = function(self)
		print(self:greeting())
	end,
	greeting = function(self)
		return string.format("Hey, %s!", self.name)
	end,
}
lang:greet()

-- Print table
vim.print(lang)

-- Display error
vim.notify("Error notification example", vim.log.levels.ERROR)

-- Run Vimscript
vim.cmd([[
    echo "Hey from Vimscript ;)"
]])

-- print a variable
-- print(vim.inspect(package.loaded))

local cur_dir = vim.fn.fnamemodify(vim.fn.expand("%:p"), ":h")
print(cur_dir)
