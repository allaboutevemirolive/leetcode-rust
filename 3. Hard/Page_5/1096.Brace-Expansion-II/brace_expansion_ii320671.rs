// https://leetcode.com/problems/brace-expansion-ii/solutions/320671/rust-solution/
pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        use std::collections::HashSet;
        let mut group : Vec<Vec<Vec<String>>> = Vec::new();
        group.push(Vec::new());
        let mut res = Vec::new();
        let mut level = 0;
        let mut start = 0;
        for (i,c) in expression.chars().enumerate() {
            if c=='{' {
                if level == 0 {
                   start = i+1;
                }
                level = level + 1;
            } else if c=='}' {
                level = level -1;
                if level == 0 {
                    let last = group.len()-1;
                    group[last].push(Solution::brace_expansion_ii(expression[start..i].to_string()));
                }
            } else if level == 0 && c == ',' {
                let last = group.len()-1;
                group.push(Vec::new());
            } else if level == 0 {
                let last = group.len()-1;
                let mut v = String::new();
                let mut vv = Vec::new();
                v.push(c);
                vv.push(v);
                group[last].push(vv);
            }
        }
        let mut result_set = HashSet::new();
        for item in group {  //item is vec<vec<string>>> 
            let mut cur = item[0].clone();
            let mut prev = item[0].clone();
            for thing in item[1..].iter() {
               cur = Vec::new();
               for p in prev {
                   for q in thing {
                        let r = p.clone()+&q.clone();
                        cur.push(r);
                   }
               }
               prev = cur;
            }
            for ss in prev {
                result_set.insert(ss);
            }
        }
        for ele in result_set {
            res.push(ele);
        }
        res.sort();
        res
    }