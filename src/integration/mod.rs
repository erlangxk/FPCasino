// map bets into u16 before send to other system.
/*
trait BetActionDeps {
    fn load_player_limits(user_id: &str);
    fn persist_bets(user_id:&str, round_id:u64, min_limit:f64, max_limit:f64, bets:HashMap<u16,f64>);
    fn pay_bets(user_id:&str, round_id:u64, amount:f64);
    fn notify_bets(user_id:&str, round_id:u64, bets:HashMap<u16, f64>);
}
*/