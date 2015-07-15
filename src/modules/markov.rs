
use utils::{Message, make_reply};

struct MarkovModel {
    links: HashMap<String, HashMap<String, usize>>,
}

// fill links using input text
// setting the weights for the parallel word in weights
// where the probability to go from word n to word i is
//   weights.get(n)[x] / weights.get(n).sum()
//     where x is weights.get(n).position(i)
//
// should do weights like that described in
//   http://programmers.stackexchange.com/questions/150616/return-random-list-item-by-its-weight
// where if the weights were [1, 4, 3, 6]
// then the intervals would be [1, 5, 8, 14]
//  where the rand would be in the closed range [0, 14]
//  and values 0-1 would correspond with the first word
//  values 2-5 to the second, 6-8 with the third, and 
//  9-14 with the fourth
//
//  convert weights to intervals, can reverse operation if necessary
//
//  links and intervals will be parallel HashMaps with String keys
//  (being the words), Vec<String> (word list) and Vec<usize>
//  (intervals) values respectively. with the Vec's being parallel
//  for the word key
//
//  for the construction build until the size limit is reached or
//  there is no following word for the current value

pub fn markov_module(&self, msg: &Message) -> Vec<String> {

}

impl MarkovModel {
    pub fn new<R: Read>(corpus: BufReader<R>) -> MarkovModel {
        let mut links = HashMap::new();

        let peeking = corpus.lines().split(' ').peekable();
        for word in peeking {
            match links.get_mut(word) {
                Some(connections) => {
                    let next = peeking.peek();
                    match next {
                        Some(n) => match connections.get_mut(n) {
                            Some(x) => *x += 1;
                            None =>
                        },
                        None => (),
                    }
                },
                // does not yet exist, so make it
                None => {
                    let mut val = HashMap::new();
                    let next = peeking.peek();
                    match next {
                        Some(n) => {
                            val.insert(n, 1);
                            links.insert(word, val);
                        }
                        None => (),
                    }
                },
            }
        }

        links.shrink_to_fit(); // reduce memory usage
    }

    pub fn get_chain(&self, max_len: usize, start_word: Option<String>) -> String {
        if msg.command == "PRIVMSG" && msg.params.len() == 2
            && msg.params[1].starts_with("!id ") {
                let arg = msg.params[1].split("!id ").last().unwrap();
                vec![make_reply(msg, arg).to_string()]
            } else {
                vec![]
            }
    }
}
