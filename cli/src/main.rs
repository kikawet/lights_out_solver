mod args;
mod handler;

use std::io::{stdout, Write};

use args::Args;
use clap::{Error, Parser};
use log::info;
#[cfg(not(test))]
use simple_logger::SimpleLogger;

use crate::handler::implementations::print::PrintHandler;
use crate::handler::implementations::sanitize_input::SanitizeHandler;
use crate::handler::implementations::simulator::SimulatorHandler;
use crate::handler::implementations::solver::SolverHandler;
use crate::handler::implementations::validate::ValidateHandler;
use crate::handler::r#trait::Handler;
use crate::handler::r#trait::HandlerMut;
use crate::handler::state::{SolvedState, State, ValidState};

fn main() {
    let input = Args::parse();

    let chain = run_program(input, &mut stdout());

    if let Some(err) = chain.err() {
        err.exit();
    }
}

fn run_program(input: Args, out: &mut impl Write) -> Result<SolvedState, Error> {
    set_up_logger(&input);

    let run_solver = input.simulation_steps.is_empty();
    let state = State::new(input);

    if run_solver {
        chain!(
            state,
            [
                ValidateHandler,
                SanitizeHandler,
                SolverHandler,
                PrintHandler { out }
            ]
        )
    } else {
        chain!(
            state,
            [
                ValidateHandler,
                SanitizeHandler,
                SimulatorHandler,
                MockSolverHandler
            ]
        )
    }
}

fn set_up_logger(args: &Args) {
    if args.verbose {
        #[cfg(not(test))]
        SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap();

        #[cfg(test)]
        cli_tests::set_up_logger();

        info!("Verbose mode enabled");
    }
}

#[derive(Default)]
struct MockSolverHandler;

impl Handler<ValidState, SolvedState> for MockSolverHandler {
    fn handle(&self, state: ValidState) -> Result<SolvedState, Error> {
        Ok(SolvedState {
            args: state.args,
            board: state.board,
            solution: None,
        })
    }
}

#[cfg(test)]
mod cli_tests {
    use crate::args::{Args, Display, Origin};
    use crate::handler::state::SolvedState;
    use crate::run_program;
    use clap::Parser;
    use log::{LevelFilter, Log, Metadata, Record};
    use solvers::Solution;
    use std::fmt::{Debug, Formatter};
    use std::sync::RwLock;

    struct TestArg<'a> {
        value: &'a str,
        assertion: fn(&AssertionContext),
    }

    struct AssertionContext<'a> {
        args: Args,
        options: &'a [&'a TestArg<'a>],
        state: SolvedState,
        output: String,
    }

    macro_rules! permutations {
        ($($it:ident),* | $last:expr) => {
          {
              $last.map(move |k| [$($it),*, k])
          }
        };
        ($($it:ident),* | $first:expr, $($rest:expr),+) => {
          {
             $first.flat_map(move |j| permutations!($($it),*, j | $($rest),+))
          }
        };
        [$first:expr, $($rest:expr),*] => {
            {
                $first.flat_map(|i| permutations!(i | $($rest),*))
            }
        };
    }

    #[test]
    fn test_run_program() {
        let lights: &[TestArg] = &[TestArg::new("", assert_lights)];
        let verbose: &[TestArg] = &[
            TestArg::new("", assert_verbose_empty),
            TestArg::new("-v", assert_verbose_enabled),
        ];
        let display_mode: &[TestArg] = &[
            TestArg::new("", assert_display),
            TestArg::new("-d simple", assert_display),
            TestArg::new("-d draw", assert_display),
            TestArg::new("-d all", assert_display),
        ];
        let origin_location: &[TestArg] = &[
            TestArg::new("", assert_origin),
            TestArg::new("-o bl", assert_origin),
            TestArg::new("-o br", assert_origin),
            TestArg::new("-o tl", assert_origin),
            TestArg::new("-o tr", assert_origin),
        ];

        for options in permutations![lights.iter(), verbose.iter(), display_mode.iter()]
            // display_mode + origin_location is too complex
            .chain(permutations![
                lights.iter(),
                verbose.iter(),
                origin_location.iter()
            ])
            //TODO: chain again to test simulation steps permutations![verbose.iter(), simulation_steps.iter()]
        {
            let options_format = format!("{:?}", options.map(|o| o.value));

            let args = Args::try_parse_from(select_test_args(&options)).unwrap_or_else(|err| {
                panic!("Unable to parse test arguments: {err} {options_format}")
            });

            LOGGER.reset_logs();
            let mut out_buffer = Vec::new();
            let state = run_program(args.clone(), &mut out_buffer)
                .unwrap_or_else(|err| panic!("Error running program: {err} {options_format}"));

            let output = String::from_utf8(out_buffer)
                .unwrap_or_else(|err| panic!("Error invalid string: {err} {options_format}"));

            let context = AssertionContext {
                args,
                state,
                output,
                options: &options,
            };

            options.iter().for_each(|o| (o.assertion)(&context));
        }
    }

    fn assert_lights(ctx: &AssertionContext) {
        assert_eq!(
            ctx.state.solution,
            AssertionHelper::lights_solution(&ctx.args.lights),
            "Unexpected output: {:?}",
            ctx.options
        );
    }

    fn assert_verbose_empty(ctx: &AssertionContext) {
        let logs = &LOGGER.logs.read().unwrap();
        assert!(
            logs.is_empty(),
            "Expected empty logs:[{logs:#?}]  {:?}",
            ctx.options
        );
    }

    fn assert_verbose_enabled(ctx: &AssertionContext) {
        let logs = &LOGGER.logs.read().unwrap();
        assert!(
            !logs.is_empty(),
            "Expected non empty logs:{logs:#?}  {:?}",
            ctx.options
        );
    }

    fn assert_display(ctx: &AssertionContext) {
        assert_eq!(
            ctx.output,
            AssertionHelper::display_output(&ctx.args),
            "Unexpected output: {:?}",
            ctx.options
        );
    }

    fn assert_origin(ctx: &AssertionContext) {
        assert_eq!(
            ctx.output,
            AssertionHelper::origin_output(&ctx.args),
            "Unexpected output: {:?}",
            ctx.options
        );
    }

    struct AssertionHelper;

    impl AssertionHelper {
        #[allow(clippy::unnecessary_wraps)]
        fn lights_solution(lights: &[usize]) -> Solution {
            match lights.len() {
                0 => Some(vec![0, 2, 4, 6, 8]),
                _ => {
                    unimplemented!("This implementation requires validation and rotation of input")
                }
            }
        }

        fn display_output(args: &Args) -> String {
            match args.display_mode {
                Display::Simple => Self::display_simple_output(),
                Display::Draw => Self::display_draw_output(),
                Display::All => Self::display_all_output(),
            }
        }

        fn display_simple_output() -> String {
            "[1, 3, 5, 7, 9]\n".to_owned()
        }

        fn display_draw_output() -> String {
            "\
            \n3·4\
            \n·2·\
            \n0·1\
            \n\
            "
            .to_owned()
        }

        fn display_all_output() -> String {
            format!(
                "{}{}",
                Self::display_simple_output(),
                Self::display_draw_output()
            )
        }

        fn origin_output(args: &Args) -> &str {
            match args.origin_location {
                Origin::BottomLeft => {
                    "\
                    \n3·4\
                    \n·2·\
                    \n0·1\
                    \n\
                    "
                }
                Origin::BottomRight => {
                    "\
                    \n4·3\
                    \n·2·\
                    \n1·0\
                    \n\
                    "
                }
                Origin::TopLeft => {
                    "\
                    \n0·1\
                    \n·2·\
                    \n3·4\
                    \n\
                    "
                }
                Origin::TopRight => {
                    "\
                    \n1·0\
                    \n·2·\
                    \n4·3\
                    \n\
                    "
                }
            }
        }
    }

    fn select_test_args<'a>(options: &'a [&'a TestArg]) -> Vec<&'a str> {
        let mut args = options
            .iter()
            .flat_map(|arg| arg.value.split(' '))
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();

        args.insert(0, "<PROGRAM>");

        args
    }

    impl<'a> TestArg<'a> {
        fn new(value: &'a str, assertion: fn(&AssertionContext)) -> Self {
            Self { value, assertion }
        }
    }

    impl Debug for TestArg<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TestArg")
                .field("value", &self.value)
                .finish_non_exhaustive()
        }
    }

    static LOGGER: BufferLogger = BufferLogger {
        logs: RwLock::new(Vec::new()),
    };

    struct BufferLogger {
        logs: RwLock<Vec<String>>, // Using Mutex to mutate inside log method
    }

    impl BufferLogger {
        fn reset_logs(&self) {
            self.logs.write().unwrap().clear();
            // Need to turn off LevelFilter or the dummy `log` will add logs anyway
            log::set_max_level(LevelFilter::Off);
        }
    }

    // This function will be called from `set_up_logger` only if verbose flag is on
    pub fn set_up_logger() {
        let _ = log::set_logger(&LOGGER); // Will error if logger was already set
        log::set_max_level(LevelFilter::Debug);
    }

    impl Log for BufferLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }

        fn log(&self, record: &Record) {
            let mut logs = self.logs.write().expect("Unable to get lock");
            logs.push(format!("{}", record.args()));
        }

        fn flush(&self) {
            panic!("Why flush?")
        }
    }
}
