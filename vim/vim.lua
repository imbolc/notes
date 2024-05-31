-- Lua in Neovim

-- Tables
local lang = {
	name = "Lua",
	greet = function(self)
		print(string.format("Hey, %s!", self.name))
	end,
}
lang:greet()

-- Print table
vim.print(lang)

-- Display error
vim.notify("An error", vim.log.levels.ERROR)

-- Run Vimscript
vim.cmd([[
    echo "Hey from Vimscript ;)"
]])

print(vim.fn.stdpath("data"))

-- print a variable
-- print(vim.inspect(package.loaded))
