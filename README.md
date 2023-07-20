
# Lights out solver

CLI program created in Rust to solve [Lights out puzzle](https://mathworld.wolfram.com/LightsOutPuzzle.html).

It finds the minimal solution and you aswell run in simulation mode to check that the board is going to look after a number of steps.
## Usage/Examples of flags

### Help
In order to read the documentation use `--help`
```sh
$ los --help
Lights Out Puzzle Solver 1.1.1
With the given input of on node it will output the order to toggle the lights to solve the puzzle

USAGE:
    los [OPTIONS] [--] [lights]...

ARGS:
    <lights>...    Indexes of the active lights (range from 1 to [cols]*[rows])

OPTIONS:
    -c, --cols <cols>            The number of columns [default: 3]
    -d, --display <mode>         Sets the way you display the results [default: draw] [possible
                                 values: simple, draw, all]
    -h, --help                   Print help information
    -i, --input <mode>           Changes where the first index is located in the matrix (eg: bl =
                                 bottom left) [default: bl] [possible values: tl, tr, bl, br]
    -r, --rows <rows>            The number of rows [default: 3]
    -s, --simulate <steps>...    Run a simulation with the given input
    -v, --verbose                Enable the debug logs
    -V, --version                Print version information
```

By default you just give the position of the lights that are on and in response you get what lights you need to toggle.
```cmd
$ los 7 9 1 3
#·#
·0·
#·#
```
### Simulation

In order to run a simulation just use `-s` in this case the '#' character represents a light on.
```cmd
$ los -s 1 2 3
···
###
·#·
```

### Size

To change the size of the board just set the number of columns or rows using the `-c` and `-r` flag

```cmd
$ los -c 4 -r 4

0123
4··5
6789
····
```

### Input mode

The input mode tells the origin of the indices so we can tell where to start counting. In this case there are 4 possible values: tl, tr, bl and br. Each one of them telling if the first index is at the top left, top right and so on.

Example of the indices for the different modes

```
tl
123
456
789 

tr
321 
654
987

bl
789
456
123

br
987
654
321

```

In this case the default value is `bl` (bottom left) it's done like that you can easily input a 3x3 board using just your num pad.

### Display

You can get the solution of the puzzle in 2 ways as a list of indices or as a drawn matrix where the numbers tell the order to trigger the lights and `#` is an on light and `·` an off light.

### Verbose

Note that you can also enable verbose mode with the `-v` flag
<details>
<summary>$ los -v 7 9 1 3</summary>
    
```cmd
2022-06-02T09:16:28.556Z INFO [los] Verbose mode enabled
2022-06-02T09:16:28.556Z DEBUG [lights_out_solver::program] Input mode: "bl"
2022-06-02T09:16:28.557Z DEBUG [lights_out_solver::program] Active indices: [6, 8, 0, 2]
2022-06-02T09:16:28.557Z DEBUG [lights_out_solver::program] Rows: 3
2022-06-02T09:16:28.558Z DEBUG [lights_out_solver::program] Cols: 3
2022-06-02T09:16:28.558Z DEBUG [lights_out_solver::program] Searching for solution ...
2022-06-02T09:16:28.559Z DEBUG [lights_out_solver::program] Final solution: Some([4])
2022-06-02T09:16:28.559Z DEBUG [lights_out_solver::program] Draw mode: draw

#·#
·0·
#·#
```
</details>


And you can aswell use `-v` to see the board after each step in the simulation
<details>
<summary>$ lights_out_solver -v -s 1 2 3</summary>

```cmd
2022-06-02T09:19:55.664Z INFO [los] Verbose mode enabled
2022-06-02T09:19:55.665Z DEBUG [lights_out_solver::program] Input mode: "bl"
2022-06-02T09:19:55.665Z DEBUG [lights_out_solver::program] Active indices: []
2022-06-02T09:19:55.666Z DEBUG [lights_out_solver::program] Rows: 3
2022-06-02T09:19:55.666Z DEBUG [lights_out_solver::program] Cols: 3
2022-06-02T09:19:55.667Z DEBUG [lights_out_solver::program] Board before the simulation:

···
···
···
2022-06-02T09:19:55.668Z DEBUG [lights_out_solver::program] Steps to simulate: [6, 7, 8]
2022-06-02T09:19:55.668Z DEBUG [lights_out_solver::program] Step 0:

···
#··
##·
2022-06-02T09:19:55.669Z DEBUG [lights_out_solver::program] Step 1:

···
##·
··#
2022-06-02T09:19:55.671Z DEBUG [lights_out_solver::program] Step 2:

···
###
·#·
2022-06-02T09:19:55.672Z DEBUG [lights_out_solver::program] Board after simulation:
···
###
·#·

···
###
·#·
```
</details>

## Run Locally

Clone the project

```bash
  git clone https://github.com/kikawet/lights_out_solver
```

Go to the project directory

```bash
  cd lights_out_solver
```

Install and run the project

```bash
  cargo run -- -h
```


## Running Tests

To run tests, run the following command

```bash
  cargo test
```


## License

[MIT](https://choosealicense.com/licenses/mit/)


## Related

Release using github actions - [rust-build.action](https://github.com/rust-build/rust-build.action)

Alternative method to solve the puzzle using polinomials instead of backtracking - [LightsOut.hh](https://www.keithschwarz.com/interesting/code/?dir=lights-out) by Keith Schwarz (htiek@cs.stanford.edu)
