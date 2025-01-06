// Minimum Operations to Move Balls to Each Box

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let n = boxes.len();
        let boxes: Vec<char> = boxes.chars().collect();

        let mut answer = vec![0; n]

        // left to right
        let mut count = 0;
        let mut operations = 0;
        for i in 0..n {
            answer[i] = operations;
            if boxes[i] == '1' {
                count += 1;
            }

            operations += count;
        }


        // right to left
        count = 0;
        operations = 0;

        for i in (0..n).rev() {
            answer[i] = operations;
            if boxes[i] == '1' {
                count += 1
            }

            operations += 1
        }

        answer
    }
}
