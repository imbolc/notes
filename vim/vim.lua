-- Lua in Neovim

-- Functions
local function greet(name)
	local greeting = "Hey"
	print(greeting, ", ", name)
end
greet("Lua")

-- Print table
vim.print({ 1, 2 })

-- Run Vimscript
vim.cmd([[
    echo "Hey from Vimscript ;)"
]])

print(vim.o.smarttab)

-- print a variable
-- print(vim.inspect(package.loaded))
