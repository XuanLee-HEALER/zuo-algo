use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct TrieNode {
    pass: usize,
    end: usize,
    nxt: [Option<Rc<RefCell<TrieNode>>>; 3],
}

#[derive(Debug)]
struct Trie {
    root: Option<Rc<RefCell<TrieNode>>>,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(TrieNode {
                pass: 0,
                end: 0,
                nxt: [const { None }; 3],
            }))),
        }
    }

    fn insert(&mut self, str: &str) {
        let mut p_trie = self.root.clone();
        for c in str.chars() {
            let idx = (c as u8 - b'a') as usize;

            if let Some(node) = p_trie.clone() {
                if node.borrow().nxt[idx].is_none() {
                    node.borrow_mut().nxt[idx] = Some(Rc::new(RefCell::new(TrieNode {
                        pass: 0,
                        end: 0,
                        nxt: [const { None }; 3],
                    })))
                }
                node.borrow_mut().pass += 1;
                p_trie = node.borrow().nxt[idx].clone()
            }
        }

        if let Some(ref node) = p_trie {
            node.borrow_mut().pass += 1;
            node.borrow_mut().end += 1;
        }
    }

    fn delete(&mut self, str: &str) {
        if Self::search(self, str) > 0 {
            let mut p_trie = self.root.clone();
            for c in str.chars() {
                let idx = (c as u8 - b'a') as usize;

                if let Some(ref node) = p_trie.clone() {
                    node.borrow_mut().pass -= 1;
                    p_trie = node.borrow().nxt[idx].clone();
                }
            }

            if let Some(ref node) = p_trie {
                node.borrow_mut().pass -= 1;
                node.borrow_mut().end -= 1;
                if node.borrow().pass == 0 {
                    node.borrow_mut().nxt = [const { None }; 3];
                }
            }
        }
    }

    fn search(&self, str: &str) -> usize {
        let mut p_trie = self.root.clone();
        for c in str.chars() {
            let idx = (c as u8 - b'a') as usize;

            if let Some(ref node) = p_trie.clone() {
                if node.borrow().nxt[idx].is_none() {
                    return 0;
                }
                p_trie = node.borrow().nxt[idx].clone();
            }
        }

        if let Some(ref node) = p_trie {
            return node.borrow().end;
        }

        unreachable!()
    }

    fn prefix(&self, pre: &str) -> usize {
        let mut p_trie = self.root.clone();
        for c in pre.chars() {
            let idx = (c as u8 - b'a') as usize;

            if let Some(ref node) = p_trie.clone() {
                if node.borrow().nxt[idx].is_none() {
                    return 0;
                }
                p_trie = node.borrow().nxt[idx].clone();
            }
        }

        if let Some(ref node) = p_trie {
            return node.borrow().pass;
        }

        unreachable!()
    }
}

fn main() {
    let mut trie = Trie::new();
    // trie.insert("abc");
    // trie.insert("ab");
    // trie.delete("ab");
    // assert_eq!(trie.search("ab"), 0);
    // assert_eq!(trie.search("abc"), 1);
    // assert_eq!(trie.prefix("ab"), 1);
    // assert_eq!(trie.prefix("abc"), 1);
    // assert_eq!(trie.prefix("a"), 1);

    let mut rng = thread_rng();
    let mut random_word = ('a'..='c').collect::<Vec<char>>();
    let mut ct = HashMap::new();

    let mut before = Vec::new();
    let mut after = Vec::new();
    for i in 0..100 {
        random_word.shuffle(&mut rng);
        let c_str = random_word.iter().collect::<String>();
        if i < 50 {
            before.push(c_str.clone());
        } else {
            after.push(c_str.clone());
        }

        trie.insert(&c_str);
        *ct.entry(c_str.clone()).or_insert(0) += 1;
        // println!(
        //     "{} {} {}",
        //     c_str,
        //     *ct.get(&c_str).unwrap(),
        //     trie.search(&c_str)
        // );
        assert_eq!(*ct.get(&c_str).unwrap(), trie.search(&c_str));
        assert_eq!(mp_prefix(&ct, &c_str[..=1]), trie.prefix(&c_str[..=1]));
    }

    for del in before.iter() {
        if ct.contains_key(del) {
            if *ct.get(del).unwrap() == 0 {
                ct.remove(del);
            } else {
                ct.entry(del.clone()).and_modify(|v| {
                    if *v > 0 {
                        *v -= 1;
                    }
                });
            }
        }
        trie.delete(del);
    }

    before.append(&mut after);
    for r_str in before {
        // println!(
        //     "{} {} {}",
        //     r_str,
        //     *ct.get(&r_str).unwrap_or(&0),
        //     trie.search(&r_str)
        // );
        assert_eq!(*ct.get(&r_str).unwrap_or(&0), trie.search(&r_str))
    }
}

fn mp_prefix(mp: &HashMap<String, usize>, prefix: &str) -> usize {
    let mut res = 0;
    for k in mp.keys() {
        if k.starts_with(prefix) {
            res += mp[k];
        }
    }
    res
}
