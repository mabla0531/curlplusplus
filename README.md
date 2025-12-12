# curl++
## Description
curl++ has the goal of being a minimal but robust HTTP client with an immediate-mode terminal UI.

### Prio 1 (week or so):

- CTRL + Backspace for deleting the remainder of tabbing (i.e. 7 spaces -> delete 3 then 4 spaces)
- Stabilization of text editor (edge issues; current functionality seems acceptable)
- a guard against escape quitting the application or some sort of other quitting mechanism (maybe a menu similar to Btop)
- tertiary color level for editing (if/else hell!)
- syntax highlighting for JSON editor

### Prio 2:

- of course actual HTTP client functionality
- highly interested in moving to ropey for token editing but for syntax highlighting I still feel like I have to iter over chars and construct a Span set

### Prio 3:

- theme engine/framework
- funky animations for request processing
- ability to act as a mock server? via an openapi doc?
