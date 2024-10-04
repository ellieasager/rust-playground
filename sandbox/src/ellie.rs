/* 
 * A simple Breadth First Search (BSF)
 * implementation in Rust
 */

 use std::collections::VecDeque;

 fn trace(from: u32, to: u32, v: &Vec<Vec<u32>>) -> Vec<u32> {
     let mut frontier:   VecDeque<u32>   = VecDeque::new();
     let mut path:       Vec<u32>        = Vec::new();
     let mut visited:    Vec<u32>        = vec![0xffff; v.len()];
 
 
     frontier.push_back(from);
     visited[from as usize] = from;

     while !frontier.is_empty() {
        let current_node = frontier.pop_front().unwrap();
        // println!("current_node {}, to {}", current_node, to);
        println!("current_node {}", current_node);
        if current_node == to {
            break;
        }
        let nbrs = &v[current_node as usize];

        for nbr in nbrs {
            if visited[*nbr as usize] == 0xffff {
                visited[*nbr as usize] = current_node;
                frontier.push_back(*nbr);
            }
        }
     }

     let mut node = to;
     path.push(node);
     while node != from {
        node = visited[node as usize];
        path.push(node);
     }
     path.reverse();

     println!("path: {:?}", path);
     println!("visited: {:?}", visited);
     return path;
 }
 
 fn gen_field_graph(n: u32) -> Vec<Vec<u32>> {
     let mut v: Vec<Vec<u32>> = Vec::new();
 
     for y in 0..n {
         for x in 0..n {
             let mut row: Vec<u32> = Vec::new();
             let pos = x + y * n;
 
             if pos % n > 0 {
                 row.push(pos - 1);  // west
             }
 
             if pos < n * (n - 1) {
                 row.push(pos + n);  // south
             }
 
             if pos % n < (n - 1) {
                 row.push(pos + 1);  // east
             }
 
             if pos >= n {
                 row.push(pos - n);  // north
             }
            //  println!("row for x = {} is {:?}", x, row);
             v.push(row);
         }
        //  println!("v for y = {} is {:?}", y, v);
     }
 
     return v;
 }
 
 fn main() {
 
     let start_point = 7;
     let end_point   =  5;
 
     let g       = gen_field_graph(3);
     let path    = trace(start_point, end_point, &g);
 
     //-------
     println!("{:?} ", path);
 }