
# Lights out solver

CLI program created in Rust to solve [Lights out puzzle](https://mathworld.wolfram.com/LightsOutPuzzle.html).

It finds the minimal solution and you aswell run in simulation mode to check that the board is going to look after a number of steps.
## Usage/Examples

In order to read the documentation use `--help`
```cmd
$ lights_out_solver --help
Puzzle Solver 0.1.0
With the given input of on node it will output the order to toggle the lights to solve the puzzle  

USAGE:
    lights_out_solver.exe [OPTIONS] [--] [NODES]...

ARGS:
    <NODES>...    Indexes of the active nodes starting at 0 on the top left

OPTIONS:
    -c <cols>                          The number of columns [default: 3]
    -h, --help                         Print help information
    -r <rows>                          The number of rows [default: 3]
    -s <postions_to_trigger>...        Run a simulation of the puzzle
    -v                                 Use verbose output
    -V, --version                      Print version information
```

By default you just give the position of the lights that are on and in response you get what lights you need to toggle.
```cmd
$ lights_out_solver 0 2 6 8
[4]
```

Note that you can also enable verbose mode with the `-v` flag
<details>
<summary>$ lights_out_solver -v 0 2 6 8</summary>
    
```cmd
2022-05-24T22:51:16.582Z INFO [lights_out_solver] Verbose mode enabled
2022-05-24T22:51:16.582Z DEBUG [lights_out_solver] Active indices: [0, 2, 6, 8]
2022-05-24T22:51:16.583Z DEBUG [lights_out_solver] Rows: 3
2022-05-24T22:51:16.583Z DEBUG [lights_out_solver] Cols: 3
2022-05-24T22:51:16.584Z DEBUG [lights_out_solver] Board:
#·#
···
#·#
2022-05-24T22:51:16.584Z DEBUG [lights_out_solver] Searching for solution ...
2022-05-24T22:51:17.166Z DEBUG [lights_out_solver] Final solution: Some([4])
[4]
```
</details>

---
In order to run a simulation just use `-s` in this case the '#' character represents a light on.
```cmd
$ lights_out_solver -s 1 2 3
#··
#·#
#··
```

And you can aswell use `-v` to see the board after each step in the simulation
<details>
<summary>$ lights_out_solver -v -s 1 2 3</summary>

```cmd
2022-05-24T22:59:16.065Z INFO [lights_out_solver] Verbose mode enabled
2022-05-24T22:59:16.066Z DEBUG [lights_out_solver] Active indices: []
2022-05-24T22:59:16.066Z DEBUG [lights_out_solver] Rows: 3
2022-05-24T22:59:16.066Z DEBUG [lights_out_solver] Cols: 3
2022-05-24T22:59:16.067Z DEBUG [lights_out_solver] Board:
···
···
···
2022-05-24T22:59:16.068Z DEBUG [lights_out_solver] Board before the simulation:

···
···
···
2022-05-24T22:59:16.069Z DEBUG [lights_out_solver] Steps to simulate: [1, 2, 3]
2022-05-24T22:59:16.069Z DEBUG [lights_out_solver] Step 0:

###
·#·
···
2022-05-24T22:59:16.071Z DEBUG [lights_out_solver] Step 1:

#··
·##
···
2022-05-24T22:59:16.074Z DEBUG [lights_out_solver] Step 2:

#··
#·#
#··
2022-05-24T22:59:16.075Z DEBUG [lights_out_solver] Board after simulation:
#··
#·#
#··

#··
#·#
#··
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
