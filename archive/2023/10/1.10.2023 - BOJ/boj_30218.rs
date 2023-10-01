//{"name":"Weighted Window Sums","group":"Baekjoon Online Judge","url":"https://www.acmicpc.net/problem/30218","interactive":false,"timeLimit":5000,"tests":[{"input":"6 3\n3\n6\n2\n3\n5\n4\n","output":"2 19\n1 21\n3 23\n4 25\n"},{"input":"7 3\n2\n3\n2\n3\n2\n4\n1\n","output":"5 13\n1 14\n3 14\n2 16\n4 19\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"WeightedWindowSums"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::fmt::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_int() as usize;
    let k = input.read_int() as usize;
    let mut a:[i32; 200000] = [0; 200000];
    let mut s:[i64; 200000] = [0; 200000];
    for i in 0..n {
        a[i] = input.read_int();
        if i > 0 { s[i] = s[i - 1] + (a[i] as i64); }
        else { s[i] = a[i] as i64; }
    }
    let mut v:Vec<(i32, i64)> = Vec::new();
    let mut ws:i64 = 0;
    for i in 0..k {
        ws += (a[i] as i64) * ((i + 1) as i64);
    }
    v.push((1, ws));
    for i in k..n {
        ws -= s[i - 1];
        if (i as i32) - (k as i32) - 1 >= 0 {
            ws += s[i - k - 1];
        }
        ws += (k as i64) * (a[i] as i64);
        v.push(((i - k + 2) as i32, ws));
    }
    v.sort_by(|&(a_i32, a_i64), &(b_i32, b_i64)| {
        a_i64.cmp(&b_i64).then_with(|| a_i32.cmp(&b_i32))
    });
    let mut buf:String = String::new();
    for (x, y) in v {
        writeln!(buf, "{x} {y}").unwrap();
    }
    out.print(buf);
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
