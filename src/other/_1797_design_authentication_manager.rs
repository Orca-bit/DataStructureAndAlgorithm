use std::collections::HashMap;

struct AuthenticationManager {
    tokens: HashMap<String, i32>,
    time_to_live: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AuthenticationManager {
    fn new(time_to_live: i32) -> Self {
        let tokens = HashMap::new();
        Self {
            tokens,
            time_to_live,
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        self.tokens.insert(token_id, current_time);
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        if let Some(time) = self.tokens.get_mut(&token_id) {
            if *time + self.time_to_live > current_time {
                *time = current_time;
            }
        }
    }

    fn count_unexpired_tokens(&self, current_time: i32) -> i32 {
        let mut count = 0;
        for time in self.tokens.values() {
            if *time + self.time_to_live > current_time {
                count += 1;
            }
        }
        count
    }
}

/**
 * Your AuthenticationManager object will be instantiated and called as such:
 * let obj = AuthenticationManager::new(timeToLive);
 * obj.generate(tokenId, currentTime);
 * obj.renew(tokenId, currentTime);
 * let ret_3: i32 = obj.count_unexpired_tokens(currentTime);
 */

#[test]
fn test() {
    let mut obj = AuthenticationManager::new(5);
    obj.renew("aaa".to_string(), 1);
    obj.generate("aaa".to_string(), 2);
    assert_eq!(obj.count_unexpired_tokens(6), 1);
    obj.generate("bbb".to_string(), 7);
    obj.renew("aaa".to_string(), 8);
    obj.renew("bbb".to_string(), 10);
    assert_eq!(obj.count_unexpired_tokens(15), 0);
}
