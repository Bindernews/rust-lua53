#include "src/lua.h"
#include "src/lauxlib.h"
#include "src/lualib.h"
#include "src/llimits.h"
#include "src/luaconf.h"

#undef LUA_EXTRASPACE
#define LUA_EXTRASPACE  sizeof(void *)

#undef LUAL_NUMSIZES
#define LUAL_NUMSIZES   sizeof(lua_Integer) * 16 + sizeof(lua_Number)
