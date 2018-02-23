pub mod common;

/*
struct Round {
    id:u64,
    hand:u8,
    start_time:u64,
    end_time:u64,
}

struct Limit {
    min: f64,
    max: f64,
}

struct BetLimit<T> {
    bets:T,
    limit:Limit,
}

struct PlayerBet<T:Hash>{
    uuid:String, 
    user_id:String, 
    bets:HashMap<T,f64>,
};

struct Table<Bets, Game> {
    id: u16,
    game: Game,
    round: Option<Round>,
    current_bets:Vec<PlayerBet<Self::Bets>>,
    previous_bets:Vec<PlayerBet<Self::Bets>>,
//   integration: BetActionDeps,
//    max_bet_offset:f64,
//    player_limits_cache:HashMap<String,Vec<Limit>
}




impl<Bets,Game> Table<Bets,Game> {
    fn bet(&mut self, user_id:&str,  min_limit:f64, max_limit:f64, round_id:u64, bets:HashMap<u16, f64>)->Result<f64> {
        let v:Result<HashMap<Bets,f64>> = self.game.from_raw_bets(bets);
        if let Ok(bets) = v {
            if(previous_bets.anybets.has_max || current_bets.anybets.has_max){
                return Err("limit changed not allowed");
            }
            if(game.bet_offset(bets))<self.max_bet_offset {

            }
            if(self.game.validate(bets)){
               
                put in the self.current_bets.push(PlayerBet{
                    uuid,
                    user_id,
                    bets
                });
                return Ok(total_bet_amount);
            }else {
                return Err("bet type not allowed");
            }
        }
        v
        
        //1. check round_id, if the time interval is allowed to place bet 
        //2. check the user_id is not banned for any reason.
        //3. check bet types name are allowed.
        //4. check bet amount on each bet types
        //5. operator total amount limit on the table.
    }

    fn next_round(&mut self, round:Round){
        self.round = Some(round);
        self.previous_player_bets = self.current_player_bets;
        self.current_player_bets = vec![];
    }

}

*/