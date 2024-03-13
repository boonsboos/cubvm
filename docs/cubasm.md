# cubasm

## supported operations

| type            | notation                                                 |
| ---             | ---                                                      | 
| rotation        | `X X' X2, Y Y' Y2, Z Z' Z2`                              | 
| face turns      | `U U' U2, F F' F2, R R' R2, B B' B2, L L' L2, D D' D2`   |
| slice moves     | `M M' M2, E E' E2, S S' S2`                              | 
| nothing         | `*` does not do a move on the cube, just leaves it as is |

wide moves (like r') are not supported (yet)

## basic syntax

cubasm consists of line that are executed top to bottom.

a line of cubasm uses any of the above moves, separated by spaces. after a semicolon (`;`), the state of the cube is submitted, and you get a fresh new cube.

example (basic sexy move algorithm): `R U R' U' ;`
> note the space between `U'` and `;` - it is necessary.

cubasm also supports submitting two cubes at once. you can do this with a comma (`,`)

here, the first cube is the argument and the second cube is the opcode

example: (pushing a default cube to the stack): `* , D' R' D R ;` 
> again, note the space between `*`, `,` and `D'`

cubasm also supports comments

## labels

labels are defined at the start of a new line.

cubasm has two types of labels: a jump label and a conditional label.

while they function similarly to calls, you have to manually construct the return opcode.

### jump label

a jump label will immediately jump to its definition.

defining a jump label: `:label M2 U M2 U2 M2 ;`

calling to a jump label: `R U :label R' U' ;`

### conditional label

a conditional label will jump to its definition if the sum of the U face of the cube at the top of the stack is greater than zero.

defining a conditonal label: `=label F R' F' R;`

calling to a conditional label: `R U R' D =label ;` 
