pub struct Node {
    pub value: i32,
    pub freq: f64,
}

pub struct OptimalBST {
    nodes: Vec<Node>,
    cost: Vec<Vec<f64>>,
    root: Vec<Vec<usize>>,
}

impl OptimalBST {
    pub fn new(nodes: Vec<Node>) -> Self {
        let n = nodes.len();
        OptimalBST {
            nodes,
            cost: vec![vec![0.0; n]; n],
            root: vec![vec![0; n]; n],
        }
    }

    pub fn compute(&mut self) {
        let n = self.nodes.len();

        for i in 0..n {
            self.cost[i][i] = self.nodes[i].freq;
            self.root[i][i] = i;
        }

        for len in 2..=n {
            for i in 0..=(n - len) {
                let j = i + len - 1;
                self.cost[i][j] = f64::INFINITY;

                let sum: f64 = (i..=j).map(|k| self.nodes[k].freq).sum();

                for r in i..=j {
                    let left_cost = if r > i { self.cost[i][r - 1] } else { 0.0 };
                    let right_cost = if r < j { self.cost[r + 1][j] } else { 0.0 };
                    let total_cost = left_cost + right_cost + sum;

                    if total_cost < self.cost[i][j] {
                        self.cost[i][j] = total_cost;
                        self.root[i][j] = r;
                    }
                }
            }
        }
    }

    pub fn get_optimal_cost(&self) -> f64 {
        self.cost[0][self.nodes.len() - 1]
    }

    pub fn show(&self) {
        if self.nodes.is_empty() {
            println!("The tree is empty.");
            return;
        }
        println!("Optimal Binary Search Tree:");
        self.print(0, self.nodes.len() - 1, -1, false);
    }

    fn print(&self, i: usize, j: usize, parent: i32, is_left: bool) {
        if i > j {
            return;
        }

        let r = self.root[i][j];
        let value = self.nodes[r].value;

        if parent == -1 {
            println!("{} is the root", value);
        } else if is_left {
            println!("{} is the left child of {}", value, parent);
        } else {
            println!("{} is the right child of {}", value, parent);
        }

        if r > i {
            self.print(i, r - 1, value, true);
        }

        if r < j {
            self.print(r + 1, j, value, false);
        }
    }
}
