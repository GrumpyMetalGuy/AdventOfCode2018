#[derive(Debug, Eq, PartialEq, Clone)]
struct TreeNode {
    children: Vec<TreeNode>,
    metadata: Vec<i32>,
    value: i32
}


impl TreeNode {
    fn collect_metadata_from_tokens(tokens: &mut Vec<String>, metadata: &mut Vec<i32>) {
        let node_count = tokens.remove(0).parse::<i32>().unwrap();
        let metadata_count = tokens.remove(0).parse::<usize>().unwrap();

        for _ in 0..node_count {
            Self::collect_metadata_from_tokens(tokens, metadata);
        }

        let metadata_tokens: Vec<String> = tokens.drain(..metadata_count).collect();
        let converted_tokens: Vec<i32> = metadata_tokens.iter().map(|item| item.parse::<i32>().unwrap()).collect();
        
        metadata.extend(converted_tokens);
    }

    fn from_tokens(tokens: &mut Vec<String>) -> TreeNode {
        let node_count = tokens.remove(0).parse::<i32>().unwrap();
        let metadata_count = tokens.remove(0).parse::<usize>().unwrap();

        let mut children: Vec<TreeNode> = Vec::new();

        for _ in 0..node_count {
            children.push(Self::from_tokens(tokens));
        }

        let metadata_tokens: Vec<String> = tokens.drain(..metadata_count).collect();
        let metadata: Vec<i32> = metadata_tokens.iter().map(|item| item.parse::<i32>().unwrap()).collect();

        let mut value = 0;

        if children.is_empty() {
            value = metadata.iter().sum::<i32>();
        } else {
            for&metadata_entry in &metadata {
                if metadata_entry > 0 && metadata_entry <= children.len() as i32 {
                    value += children[(metadata_entry - 1) as usize].value;
                }
            }
        }

        TreeNode{
            children,
            metadata,
            value
        }
    }
}


fn part1() {
    let file_input = utils::read_file(String::from("day8.txt"));
    let mut input_tokens = file_input.split(' ').collect::<Vec<_>>().iter().map(|token| String::from(*token)).collect();
    let mut metadata: Vec<i32> = Vec::new();

    TreeNode::collect_metadata_from_tokens(&mut input_tokens, &mut metadata);

    let total_metadata_sum = metadata.iter().sum::<i32>();

    println!("Node sum: {}", total_metadata_sum);
}


fn part2() {
    let file_input = utils::read_file(String::from("day8.txt"));
    let mut input_tokens = file_input.split(' ').collect::<Vec<_>>().iter().map(|token| String::from(*token)).collect();

    let tree = TreeNode::from_tokens(&mut input_tokens);

    println!("Value: {:?}", tree.value);
}


fn main() {
    part1();
    part2();
}
