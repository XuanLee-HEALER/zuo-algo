use std::collections::{HashMap, HashSet};

struct Solution;

// You are given a string s and an array of strings words. All the strings of words are of the same length.

// A concatenated string is a string that exactly contains all the strings of any permutation of words concatenated.

// For example, if words = ["ab","cd","ef"], then "abcdef", "abefcd", "cdabef", "cdefab", "efabcd", and "efcdab" are all concatenated strings. "acdbef" is not a concatenated string because it is not the concatenation of any permutation of words.
// Return an array of the starting indices of all the concatenated substrings in s. You can return the answer in any order.

impl Solution {
    // dumb
    pub fn find_substring_1(s: String, words: Vec<String>) -> Vec<i32> {
        let mut hp = HashMap::new();
        let n = s.len();
        let m = words.len();
        let word_l = words[0].len();
        let window_size = m * word_l;
        for word in &words {
            hp.entry(word.as_str()).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut l = 0;
        let mut r = window_size - 1;
        if r >= n {
            vec![]
        } else {
            let mut res = vec![];
            let mut statis = HashMap::new();
            while r < n {
                let mut ok = true;
                for i in 0..m {
                    let cur_str = &s[l + i * word_l..l + (i + 1) * word_l];
                    statis.entry(cur_str).and_modify(|v| *v += 1).or_insert(1);
                }
                for (k, v) in statis.iter() {
                    if !hp.contains_key(k) || hp.get(k).unwrap() != v {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    res.push(l as i32);
                }
                l += 1;
                r += 1;
                statis.clear();
            }
            res
        }
    }

    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut hp = HashMap::new();
        let n = s.len();
        let m = words.len();
        let word_l = words[0].len();
        let window_size = m * word_l;
        for word in &words {
            hp.entry(word.as_str()).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut res = vec![];
        for i in 0..word_l {
            let mut l = i;
            let mut r = l + window_size - 1;
            let mut statis = HashMap::new();
            if r < n {
                for i in 0..m {
                    let cur_str = &s[l + i * word_l..l + (i + 1) * word_l];
                    statis.entry(cur_str).and_modify(|v| *v += 1).or_insert(1);
                }
                if Self::match_counter(&hp, &statis) {
                    res.push(l as i32);
                }
                l += word_l;
                r += word_l;
            } else {
                break;
            }
            while r < n {
                // println!(
                //     "old {} new {} {} {} {} {}",
                //     &s[l - word_l..l],
                //     &s[r - word_l + 1..=r],
                //     l - word_l,
                //     l,
                //     r - word_l + 1,
                //     r
                // );
                statis.entry(&s[l - word_l..l]).and_modify(|v| *v -= 1);
                statis
                    .entry(&s[r - word_l + 1..=r])
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
                if Self::match_counter(&hp, &statis) {
                    res.push(l as i32);
                }
                l += word_l;
                r += word_l;
            }
        }

        res
    }

    fn match_counter(ori: &HashMap<&str, i32>, target: &HashMap<&str, i32>) -> bool {
        for (k, v) in ori {
            if !target.contains_key(k) || *target.get(k).unwrap() != *v {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test_solution {
    #[test]
    fn test_find_substring() {
        // ["ab","cd","ef"]"acdbef"
        // "cdefab"
        let sample_s = "barfoothefoobarman";
        let sample_arr = vec!["foo".to_string(), "bar".to_string()];
        assert_eq!(
            super::Solution::find_substring(sample_s.into(), sample_arr),
            vec![0, 9]
        );
        let sample_s = "wordgoodgoodgoodbestword";
        let sample_arr = vec![
            "word".to_string(),
            "good".to_string(),
            "best".to_string(),
            "word".to_string(),
        ];
        assert_eq!(
            super::Solution::find_substring(sample_s.into(), sample_arr),
            vec![]
        );
        let sample_s = "wordgoodgoodgoodbestword";
        let sample_arr = vec![
            "word".to_string(),
            "good".to_string(),
            "best".to_string(),
            "good".to_string(),
        ];
        assert_eq!(
            super::Solution::find_substring(sample_s.into(), sample_arr),
            vec![8]
        )
    }
}
