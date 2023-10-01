//{"name":"대피소","group":"Baekjoon Online Judge","url":"https://www.acmicpc.net/problem/28215","interactive":false,"timeLimit":3000,"tests":[{"input":"5 2\n1 5\n3 0\n3 3\n6 12\n8 9\n","output":"5\n"},{"input":"4 2\n0 0\n0 5\n5 0\n5 5\n","output":"5\n"},{"input":"4 1\n1 0\n2 0\n3 0\n4 0\n","output":"2\n"},{"input":"2 1\n20 23\n5 14\n","output":"24\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Task"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_int() as usize;
    let k = input.read_int() as usize;
    let mut p:Vec<(i32, i32)> = Vec::new();
    for i in 0..n {
        let x = input.read_int();
        let y = input.read_int();
        p.push((x, y));
    }
    let get_dist:fn((i32, i32), (i32, i32)) -> i32 = |p, q| {
        i32::abs(p.0 - q.0) + i32::abs(p.1 - q.1)
    };
    let mut ans:i32 = std::i32::MAX;
    match k {
        1 => {
            ans = p.iter().map(|pi| {
                p.iter().map(|h| get_dist(*pi, *h)).max().unwrap_or(0)
            }).min().unwrap_or(std::i32::MAX);
        },
        2 => {
            ans = (0..n).map(|i| { let pi = &p[i];
                (i..n).map(|j| { let pj = &p[j];
                    p.iter().map(|h| get_dist(*pi, *h).min(get_dist(*pj, *h))).max().unwrap_or(0)
                }).min().unwrap_or(std::i32::MAX)
            }).min().unwrap();
        },
        3 => {
            ans = (0..n).map(|i| { let pi = &p[i];
                (i..n).map(|j| { let pj = &p[j];
                    (j..n).map(|k| { let pk = &p[k];
                        p.iter().map(|h| get_dist(*pi, *h).min(get_dist(*pj, *h)).min(get_dist(*pk, *h))).max().unwrap_or(0)
                    }).min().unwrap_or(std::i32::MAX)
                }).min().unwrap()
            }).min().unwrap();
        },
        _ => ()
    };
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
