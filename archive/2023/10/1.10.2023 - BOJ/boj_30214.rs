//{"name":"An Easy-Peasy Problem","group":"Baekjoon Online Judge","url":"https://www.acmicpc.net/problem/30214","interactive":false,"timeLimit":5000,"tests":[{"input":"10 20\n","output":"E\n"},{"input":"6 13\n","output":"H\n"},{"input":"15 22\n","output":"E\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AnEasyPeasyProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s1 = input.read_int();
    let s2 = input.read_int();
    if s2-s1 <= s1 {
        out.print("E");
    } else {
        out.print("H");
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
//    stress_test::stress_test(run, tester::check);
}
//END MAIN
