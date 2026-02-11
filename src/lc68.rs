struct Solution;

impl Solution {
    // TODO: 所有单词先贪心分行，有显示 words[i].len() <= max_width
    // 每一行的空格数就是 max_width-sum(words.total_length())
    // slot数就是 spaces.length / (words.len() - 1)，前 spaces%slot 个slot需要多加一个空格
    // 最后一行从左往右放单词，填空格即可
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut res = Vec::new();
        let mut i = 0;
        while i < words.len() {
            // 贪心分行：从 i 开始尽可能多塞单词，单词间至少1个空格
            let mut j = i;
            let mut line_len = words[j].len();
            j += 1;
            while j < words.len() && line_len + 1 + words[j].len() <= max_width {
                line_len += 1 + words[j].len();
                j += 1;
            }
            // words[i..j] 是当前行的单词
            let word_count = j - i;
            let char_len: usize = (i..j).map(|k| words[k].len()).sum();
            let total_spaces = max_width - char_len;

            let mut line = String::with_capacity(max_width);
            if j == words.len() || word_count == 1 {
                // 最后一行 或 只有一个单词：左对齐，单词间1个空格，右边补空格
                for k in i..j {
                    if k > i {
                        line.push(' ');
                    }
                    line.push_str(&words[k]);
                }
                while line.len() < max_width {
                    line.push(' ');
                }
            } else {
                // 中间行：空格均匀分配到 word_count-1 个槽位
                let slots = word_count - 1;
                let base = total_spaces / slots;
                let extra = total_spaces % slots; // 前 extra 个槽位多1个空格
                for k in i..j {
                    if k > i {
                        let slot_idx = k - i - 1;
                        let spaces = base + if slot_idx < extra { 1 } else { 0 };
                        for _ in 0..spaces {
                            line.push(' ');
                        }
                    }
                    line.push_str(&words[k]);
                }
            }
            res.push(line);
            i = j;
        }
        res
    }
}

#[cfg(test)]
mod test_lc68 {
    use super::*;

    #[test]
    fn test() {
        let words = vec!["This", "is", "an", "example", "of", "text", "justification."];
        let words: Vec<String> = words.into_iter().map(String::from).collect();
        assert_eq!(
            Solution::full_justify(words, 16),
            vec!["This    is    an", "example  of text", "justification.  "]
        );

        let words: Vec<String> = vec!["What", "must", "be", "acknowledgment", "shall", "be"]
            .into_iter().map(String::from).collect();
        assert_eq!(
            Solution::full_justify(words, 16),
            vec!["What   must   be", "acknowledgment  ", "shall be        "]
        );

        let words: Vec<String> = vec![
            "Science", "is", "what", "we", "understand", "well", "enough",
            "to", "explain", "to", "a", "computer.", "Art", "is",
            "everything", "else", "we", "do",
        ].into_iter().map(String::from).collect();
        assert_eq!(
            Solution::full_justify(words, 20),
            vec![
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  ",
            ]
        );
    }
}
