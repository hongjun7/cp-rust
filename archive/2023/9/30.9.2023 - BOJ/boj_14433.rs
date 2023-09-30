//{"name":"task","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"task"}}}

use std::io::Read;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::max_flow::MaxFlow;

type PreCalc = ();

fn max_matching(input: &mut Input, n: usize, m: usize, k: usize) -> i32 {
    let edges = input.read_size_pair_vec(k);
    let mut graph = Graph::new(n + m + 2);
    for i in 0..n {
        graph.add_edge(0, FlowEdge::new(i + 1, 1));
    }
    for i in 0..m {
        graph.add_edge(n + i + 1, FlowEdge::new(n + m + 1, 1));
    }
    for edge in &edges {
        let (u, v) = edge;
        graph.add_edge(*u, FlowEdge::new(n + *v + 1, 1));
    }
    graph.max_flow(0, n + m + 1)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k1 = input.read_size();
    let k2 = input.read_size();
    let m1 = max_matching(input, n, m, k1);
    let m2 = max_matching(input, n, m, k2);
    if m1 >= m2 { out.print("그만 알아보자"); }
    else { out.print("네 다음 힐딱이"); }
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
