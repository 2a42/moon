# Not maintained
This project is based on an old version of rust and is no longer maintained.


# Moon

A lua interpter written in rust. Does not support evey functionality of Lua, here is an example of program of supported features:
```
x = 1
y = 3
while x < 2 do
  local y = 2
  print(x, y)
  x = x - 1
end
print(y)
```

## Currently supported
* Parser supporting most Lua statements
* Variable declaration, local and global
* Numbers, strings, arithmetic operations
* While and if statements
* Test operators (==, <=, ~=, etc.)
* Calling functions defined in rust

## TODO
* Currently user defined functions are not supported. They require capturing the lua environment at the moment they were created, which means considerably changing the design of the interpretor.
* Support of several variables declaration statement e.g. x, y = 2, 3. Not too complicated.
