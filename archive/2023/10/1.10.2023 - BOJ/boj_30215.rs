//{"name":"Age Expression","group":"Baekjoon Online Judge","url":"https://www.acmicpc.net/problem/30215","interactive":false,"timeLimit":5000,"tests":[{"input":"69 9 1\n","output":"1\n"},{"input":"76 11 7\n","output":"1\n"},{"input":"50 9 3\n","output":"0\n"},{"input":"70 10 5\n","output":"1\n"},{"input":"10 7 2\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AgeExpression"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let o_age = input.read_int();
    let a_age = input.read_int();
    let k_age = input.read_int();
    let mut ans = 0;
    for a in 1..=150 {
        for k in 1..=150 {
            if o_age == a * a_age + k * k_age {
                ans = 1;
            }
        }
    }
    out.print(ans);
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
