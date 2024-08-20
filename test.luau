local ffi = require("@lune/ffi")
local addlib =
	ffi.load("./libtesting.so", { { name = "add", parameters = { "u64", "u64" }, result = "u64" } })
local add = addlib:symbol("add")
print(add(1, 2))
