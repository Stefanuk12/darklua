local process = require("@lune/process")

local exitCode = require("@self/lune_packages/toolchainlib")("Stefanuk12/darklua", _G.PESDE_ROOT)

return process.exit(exitCode)