use std::{collections::HashMap, fs::File, io::Read, iter};

struct Node {
    val: char, 
    next: Option<Box<Node>>
}

impl Node {
    fn new(val: char, next: Option<Box<Node>>) -> Node {
        Node { val, next }
    }

    fn construct_list_trie(pattern: &str) -> Node {
        let mut chars = pattern.chars();
        let first = chars.next().unwrap();

        let mut head = Node::new(first, None);
        let mut curr = &mut head; 

        for char in chars {
            curr.next = Some(Box::new(Node::new(char, None)));
            curr = curr.next.as_mut().unwrap();
        }

        head
    }
    
    fn string_match(pattern: Node, text: &str) -> Vec<(u64, u64)> {
        let mut result: Vec<(u64,u64)> = Vec::new();
        let mut chars = text.chars().peekable();
        let mut cur = &pattern;

        let mut temp_index = 0;
        let mut stuff: [String; 2] = [" ".to_string(), " ".to_string()];

        let mut c = chars.next().unwrap();
        let do_prefix = ['o', '(', ')'];
        let dont_prefix = ['o','n', '\'', 't', '(', ')'];
        let mut do_flag = true;

        while chars.peek().is_some() {
            
            if c == 'd'{
                let mut temp_chars = chars.clone();
                if temp_chars.by_ref().take(do_prefix.len()).eq(do_prefix.iter().copied()) {
                    println!("found do prefix");
                    chars.nth(do_prefix.len() - 1);
                    do_flag = true;
                    continue;
                }

                let mut temp_chars = chars.clone();
                if temp_chars.by_ref().take(dont_prefix.len()).eq(dont_prefix.iter().copied()) {
                    println!("found dont prefix");
                    chars.nth(dont_prefix.len() - 1);
                    do_flag = false;
                    continue;
                }

                if chars.peek().is_some(){
                    c = chars.next().unwrap();
                }

                cur = &pattern;
                temp_index = 0;
                continue
            }

            if cur.val == '*' {
                if !c.is_ascii_digit(){
                    cur = &pattern;
                    temp_index = 0;
                    c = chars.next().unwrap();

                    continue 
                }

                let mut m = String::new();

                while c.is_ascii_digit() && chars.peek().is_some() {
                    m.push(c);
                    c = chars.next().unwrap();
                }

                stuff[temp_index] = m;

                temp_index += 1;
                cur = cur.next.as_deref().unwrap();
            } else if cur.val == c {
                if cur.val == ')'{
                    if do_flag{ 
                        let m1 = stuff[0].parse::<u64>()
                            .expect(&format!("Unable to convert {} to usize.", stuff[0]));
                        let m2 = stuff[1].parse::<u64>()
                            .expect(&format!("Unable to convert {} to usize.", stuff[0]));
                        result.push((m1,m2));
                    }
                    c = chars.next().unwrap();
                    cur = &pattern; 
                    temp_index = 0;
                }else {
                    c = chars.next().unwrap();
                    cur = cur.next.as_deref().unwrap();
                }
                
            }else{
                c = chars.next().unwrap();
                cur = &pattern; 
                temp_index = 0;
            }
        }

        result
    }

}

fn open_file(path: &str) -> File {
    File::open(path).expect(&format!("Error opening file at path, {}.", path))
}


fn solve_silver(input: &str) -> u64 {
    let pattern = "mul(*,*)";
    let node_pattern = Node::construct_list_trie(pattern);
    let result = Node::string_match(node_pattern, input);
    let num = result.iter().map(|(a,b)| a*b).sum();
    //println!("{:?}", result);
    println!("{}", num);
    num
}

//alright todays solution is absolutely unapologetically awful.
//but it was this, just use re.match(), or build something like a full re engine
//no thanks...
fn main() {
    let mut file = open_file("src/data.txt");
    let mut data = String::new();

    let _ = file.read_to_string(&mut data);

    let result = solve_silver(&data);

}
