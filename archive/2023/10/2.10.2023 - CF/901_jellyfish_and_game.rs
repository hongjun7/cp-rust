//{"name":"A. Jellyfish and Game","group":"Codeforces - Codeforces Round 901 (Div. 1)","url":"https://codeforces.com/contest/1874/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2 2 1\n1 2\n3 4\n1 1 10000\n1\n2\n4 5 11037\n1 1 4 5\n1 9 1 9 8\n1 1 1\n2\n1\n","output":"6\n1\n19\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AJellyfishAndGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_int() as usize;
    let m = input.read_int() as usize;
    let mut k = input.read_int();
    let mut a = input.read_int_vec(n);
    let mut b = input.read_int_vec(m);
    k = (k - 1) % 2;
    let mut sum_a:i64 = a.iter().map(|x| *x as i64).sum();
    let mut sum_b:i64 = b.iter().map(|x| *x as i64).sum();
    if let Some((idx_a, &min_a)) = a.iter().enumerate().min_by_key(|(_, &x)| x) {
        if let Some((idx_b, &max_b)) = b.iter().enumerate().max_by_key(|(_, &x)| x) {
            if min_a < max_b {
                sum_a = sum_a - (min_a as i64) + (max_b as i64);
                sum_b = sum_b - (max_b as i64) + (min_a as i64);
                let tmp = a[idx_a];
                a[idx_a] = b[idx_b];
                b[idx_b] = tmp;
            }
        } else {
            return;
        }
    } else {
        return;
    }
    if k >= 1 {
        let mut min_b = b.iter().min().unwrap();
        let mut max_a = a.iter().max().unwrap();
        if *min_b < *max_a {
            sum_a = sum_a - (*max_a as i64) + (*min_b as i64);
            sum_b = sum_b - (*min_b as i64) + (*max_a as i64);
        }
    }
    out.print_line(sum_a);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
