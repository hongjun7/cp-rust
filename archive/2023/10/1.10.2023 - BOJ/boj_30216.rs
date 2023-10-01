//{"name":"Increasing Sublist","group":"Baekjoon Online Judge","url":"https://www.acmicpc.net/problem/30216","interactive":false,"timeLimit":5000,"tests":[{"input":"6\n5 7 2 4 6 3\n","output":"3\n"},{"input":"15\n10 70 80 5 5 5 15 20 30 40 60 9 8 70 80\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IncreasingSublist"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_int() as usize;
    let a = input.read_int_vec(n);
    let mut ans = 1;
    for i in 1..n {
        let mut cnt = 1;
        for j in 1..=i {
            if a[i - j] < a[i - j + 1] {
                cnt += 1;
            } else {
                break;
            }
        }
        println!("{i} {cnt}");
        if ans < cnt { ans = cnt; }
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
