use std::collections::HashMap;
use super::{corner, line, split, straight, street, simple, BetKind};

struct Roulette {
    all_bets: HashMap<u16, Box<BetKind>>,
}

impl Roulette {
    fn new() -> Roulette {
        let mut map = HashMap::<u16, Box<BetKind>>::new();
        simple::all_bets(&mut map);
        straight::all_bets(&mut map);
        split::all_bets(&mut map);
        street::all_bets(&mut map);
        corner::all_bets(&mut map);
        line::all_bets(&mut map);
        Roulette { all_bets: map }
    }

    fn valid_bets(&self, bet_id: u16) -> bool {
        self.all_bets.contains_key(&bet_id)
    }

    fn payout_map(&self, ball: u8) -> HashMap<u16, f64> {
        self.all_bets
            .iter()
            .filter(|&(_, b)| b.bingo(ball))
            .map(|(&id, b)| (id, b.ratio()+1.0))
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payout_map_0(){
        let g = Roulette::new();
        let r = g.payout_map(0);
        assert_eq!(r, hashmap!{100=>36.0, 200=>18.0, 201=>18.0, 202=>18.0, 300=>12.0, 301=>12.0, 422=>9.0,});
    }

    #[test]
    fn test_payout_map_1(){
        let g = Roulette::new();
        let r = g.payout_map(1);
        assert_eq!(r, hashmap!{1=>2.0, 3=>2.0, 6=>2.0, 7=>3.0, 10=>3.0, 101=>36.0, 200=>18.0, 203=>18.0, 236=>18.0, 300=>12.0, 302=>12.0, 400=>9.0, 422=>9.0, 600=>6.0});
    }

    #[test]
    fn test_payout_map_2(){
        let g = Roulette::new();
        let r = g.payout_map(2);
        assert_eq!(r, hashmap!{2=>2.0, 4=>2.0, 6=>2.0, 7=>3.0, 11=>3.0, 102=>36.0, 201=>18.0, 204=>18.0, 236=>18.0, 237=>18.0, 300=>12.0, 301=>12.0, 302=>12.0, 400=>9.0, 401=>9.0, 422=>9.0, 600=>6.0});
    }

    #[test]
    fn test_payout_map_3(){
        let g = Roulette::new();
        let r = g.payout_map(3);
        assert_eq!(r, hashmap!{1=>2.0, 3=>2.0, 6=>2.0, 7=>3.0, 12=>3.0, 103=>36.0, 202=>18.0, 205=>18.0, 237=>18.0, 301=>12.0, 302=>12.0, 401=>9.0, 422=>9.0, 600=>6.0});
    }

    #[test]
    fn test_payout_map_4(){
        let r = Roulette::new().payout_map(4);
        assert_eq!(r, hashmap!{2=>2.0, 4=>2.0, 6=>2.0, 7=>3.0, 10=>3.0, 104=>36.0, 203=>18.0, 206=>18.0, 238=>18.0, 303=>12.0, 400=>9.0, 402=>9.0, 600=>6.0, 601=>6.0});
    }

     #[test]
    fn test_payout_map_5(){
        let r = Roulette::new().payout_map(5);
        assert_eq!(r, hashmap!{400=>9.0,303=>12.0,207=>18.0,403=>9.0,7=>3.0,239=>18.0,6=>2.0,11=>3.0,105=>36.0,3=>2.0,402=>9.0,401=>9.0,600=>6.0,1=>2.0,601=>6.0,238=>18.0,204=>18.0});
    }

    #[test]
    fn test_payout_map_6(){
        let r = Roulette::new().payout_map(6);
        assert_eq!(r, hashmap!{303=>12.0,106=>36.0,403=>9.0,7=>3.0,12=>3.0,208=>18.0,2=>2.0,239=>18.0,6=>2.0,205=>18.0,401=>9.0,600=>6.0,601=>6.0,4=>2.0});
    }

    #[test]
    fn test_payout_map_7(){
        let r = Roulette::new().payout_map(7);
        assert_eq!(r, hashmap!{206=>18.0,304=>12.0,7=>3.0,404=>9.0,6=>2.0,107=>36.0,3=>2.0,602=>6.0,402=>9.0,209=>18.0,240=>18.0,1=>2.0,601=>6.0,10=>3.0});
    }
     
    #[test]
    fn test_payout_map_8(){
        let r = Roulette::new().payout_map(8);
        assert_eq!(r, hashmap!{405=>9.0,207=>18.0,304=>12.0,403=>9.0,7=>3.0,241=>18.0,2=>2.0,108=>36.0,404=>9.0,6=>2.0,11=>3.0,602=>6.0,402=>9.0,240=>18.0,601=>6.0,210=>18.0,4=>2.0});
    }
     
    #[test]
    fn test_payout_map_9(){
        let r = Roulette::new().payout_map(9);
        assert_eq!(r, hashmap!{405=>9.0,304=>12.0,403=>9.0,7=>3.0,241=>18.0,12=>3.0,208=>18.0,109=>36.0,6=>2.0,3=>2.0,602=>6.0,1=>2.0,601=>6.0,211=>18.0});
    }
     
    #[test]
    fn test_payout_map_10(){
        let r = Roulette::new().payout_map(10);
        assert_eq!(r, hashmap!{406=>9.0,7=>3.0,2=>2.0,404=>9.0,6=>2.0,110=>36.0,242=>18.0,602=>6.0,209=>18.0,212=>18.0,305=>12.0,603=>6.0,10=>3.0,4=>2.0});
    }
     
    #[test]
    fn test_payout_map_11(){
        let r = Roulette::new().payout_map(11);
        assert_eq!(r, hashmap!{405=>9.0,213=>18.0,406=>9.0,7=>3.0,243=>18.0,2=>2.0,404=>9.0,6=>2.0,111=>36.0,242=>18.0,407=>9.0,11=>3.0,3=>2.0,602=>6.0,305=>12.0,603=>6.0,210=>18.0});
    }
     
    #[test]
    fn test_payout_map_12(){
        let r = Roulette::new().payout_map(12);
        assert_eq!(r, hashmap!{112=>36.0,405=>9.0,7=>3.0,12=>3.0,243=>18.0,6=>2.0,214=>18.0,407=>9.0,602=>6.0,305=>12.0,1=>2.0,211=>18.0,603=>6.0,4=>2.0});
    }
     
    #[test]
    fn test_payout_map_13(){
        let r = Roulette::new().payout_map(13);
        assert_eq!(r, hashmap!{113=>36.0,408=>9.0,406=>9.0,604=>6.0,244=>18.0,8=>3.0,2=>2.0,6=>2.0,215=>18.0,3=>2.0,212=>18.0,603=>6.0,10=>3.0,306=>12.0});
    }
     
    #[test]
    fn test_payout_map_14(){
        let r = Roulette::new().payout_map(14);
        assert_eq!(r, hashmap!{408=>9.0,213=>18.0,406=>9.0,604=>6.0,244=>18.0,8=>3.0,216=>18.0,6=>2.0,409=>9.0,407=>9.0,11=>3.0,245=>18.0,114=>36.0,1=>2.0,603=>6.0,4=>2.0,306=>12.0});
    }
     
    #[test]
    fn test_payout_map_15(){
        let r = Roulette::new().payout_map(15);
        assert_eq!(r, hashmap!{217=>18.0,604=>6.0,12=>3.0,8=>3.0,2=>2.0,6=>2.0,409=>9.0,214=>18.0,407=>9.0,3=>2.0,245=>18.0,115=>36.0,603=>6.0,306=>12.0});
    }
     
    #[test]
    fn test_payout_map_16(){
        let r = Roulette::new().payout_map(16);
        assert_eq!(r, hashmap!{408=>9.0,604=>6.0,116=>36.0,8=>3.0,246=>18.0,218=>18.0,6=>2.0,307=>12.0,215=>18.0,605=>6.0,1=>2.0,410=>9.0,10=>3.0,4=>2.0});
    }
     
    #[test]
    fn test_payout_map_17(){
        let r = Roulette::new().payout_map(17);
        assert_eq!(r, hashmap!{408=>9.0,411=>9.0,604=>6.0,8=>3.0,246=>18.0,2=>2.0,216=>18.0,6=>2.0,409=>9.0,307=>12.0,11=>3.0,3=>2.0,219=>18.0,605=>6.0,117=>36.0,247=>18.0,410=>9.0});
    }
     
    #[test]
    fn test_payout_map_18(){
        let r = Roulette::new().payout_map(18);
        assert_eq!(r, hashmap!{217=>18.0,411=>9.0,604=>6.0,12=>3.0,8=>3.0,6=>2.0,409=>9.0,307=>12.0,118=>36.0,605=>6.0,220=>18.0,1=>2.0,247=>18.0,4=>2.0});
    }
     
    #[test]
    fn test_payout_map_19(){
        let r = Roulette::new().payout_map(19);
        assert_eq!(r, hashmap!{248=>18.0,5=>2.0,8=>3.0,218=>18.0,119=>36.0,606=>6.0,308=>12.0,3=>2.0,221=>18.0,412=>9.0,605=>6.0,1=>2.0,410=>9.0,10=>3.0});
    }
     
    #[test]
    fn test_payout_map_20(){
        let r = Roulette::new().payout_map(20);
        assert_eq!(r, hashmap!{413=>9.0,222=>18.0,248=>18.0,411=>9.0,5=>2.0,8=>3.0,2=>2.0,606=>6.0,308=>12.0,11=>3.0,120=>36.0,412=>9.0,219=>18.0,605=>6.0,410=>9.0,249=>18.0,4=>2.0});
    }
     
    #[test]
    fn test_payout_map_21(){
        let r = Roulette::new().payout_map(21);
        assert_eq!(r, hashmap!{413=>9.0,411=>9.0,5=>2.0,12=>3.0,8=>3.0,223=>18.0,606=>6.0,308=>12.0,3=>2.0,605=>6.0,220=>18.0,1=>2.0,249=>18.0,121=>36.0});
    }
     
    #[test]
    fn test_payout_map_22(){
        let r = Roulette::new().payout_map(22);
        assert_eq!(r, hashmap!{309=>12.0,607=>6.0,250=>18.0,5=>2.0,8=>3.0,2=>2.0,122=>36.0,606=>6.0,414=>9.0,224=>18.0,221=>18.0,412=>9.0,10=>3.0,4=>2.0});
    }
     
    #[test]
    fn test_payout_map_23(){
        let r = Roulette::new().payout_map(23);
        assert_eq!(r, hashmap!{413=>9.0,309=>12.0,607=>6.0,222=>18.0,225=>18.0,250=>18.0,5=>2.0,251=>18.0,123=>36.0,8=>3.0,606=>6.0,414=>9.0,11=>3.0,3=>2.0,412=>9.0,415=>9.0,1=>2.0});
    }
     
    #[test]
    fn test_payout_map_24(){
        let r = Roulette::new().payout_map(24);
        assert_eq!(r, hashmap!{413=>9.0,309=>12.0,607=>6.0,5=>2.0,251=>18.0,12=>3.0,8=>3.0,223=>18.0,2=>2.0,606=>6.0,415=>9.0,124=>36.0,226=>18.0,4=>2.0});
    }
     
    #[test]
    fn test_payout_map_25(){
        let r = Roulette::new().payout_map(25);
        assert_eq!(r, hashmap!{227=>18.0,607=>6.0,9=>3.0,5=>2.0,416=>9.0,252=>18.0,414=>9.0,3=>2.0,224=>18.0,608=>6.0,125=>36.0,1=>2.0,310=>12.0,10=>3.0});
    }
     
    #[test]
    fn test_payout_map_26(){
        let r = Roulette::new().payout_map(26);
        assert_eq!(r, hashmap!{253=>18.0,607=>6.0,9=>3.0,225=>18.0,5=>2.0,126=>36.0,416=>9.0,2=>2.0,252=>18.0,414=>9.0,11=>3.0,228=>18.0,417=>9.0,608=>6.0,415=>9.0,310=>12.0,4=>2.0});
    }
     
    #[test]
    fn test_payout_map_27(){
        let r = Roulette::new().payout_map(27);
        assert_eq!(r, hashmap!{253=>18.0,607=>6.0,9=>3.0,127=>36.0,229=>18.0,5=>2.0,12=>3.0,3=>2.0,417=>9.0,608=>6.0,415=>9.0,1=>2.0,310=>12.0,226=>18.0});
    }
     
    #[test]
    fn test_payout_map_28(){
        let r = Roulette::new().payout_map(28);
        assert_eq!(r, hashmap!{254=>18.0,227=>18.0,9=>3.0,5=>2.0,416=>9.0,609=>6.0,2=>2.0,311=>12.0,128=>36.0,608=>6.0,230=>18.0,418=>9.0,10=>3.0,4=>2.0});
    }
     
    #[test]
    fn test_payout_map_29(){
        let r = Roulette::new().payout_map(29);
        assert_eq!(r, hashmap!{254=>18.0,129=>36.0,419=>9.0,9=>3.0,5=>2.0,416=>9.0,231=>18.0,609=>6.0,2=>2.0,255=>18.0,311=>12.0,11=>3.0,228=>18.0,3=>2.0,417=>9.0,608=>6.0,418=>9.0});
    }
     
    #[test]
    fn test_payout_map_30(){
        let r = Roulette::new().payout_map(30);
        assert_eq!(r, hashmap!{419=>9.0,232=>18.0,9=>3.0,229=>18.0,5=>2.0,12=>3.0,130=>36.0,609=>6.0,255=>18.0,311=>12.0,417=>9.0,608=>6.0,1=>2.0,4=>2.0});
    }
     
    #[test]
    fn test_payout_map_31(){
        let r = Roulette::new().payout_map(31);
        assert_eq!(r, hashmap!{420=>9.0,9=>3.0,233=>18.0,5=>2.0,610=>6.0,609=>6.0,2=>2.0,131=>36.0,312=>12.0,3=>2.0,256=>18.0,230=>18.0,418=>9.0,10=>3.0});
    }
     
    #[test]
    fn test_payout_map_32(){
        let r = Roulette::new().payout_map(32);
        assert_eq!(r, hashmap!{420=>9.0,419=>9.0,9=>3.0,5=>2.0,421=>9.0,231=>18.0,610=>6.0,609=>6.0,257=>18.0,312=>12.0,11=>3.0,256=>18.0,132=>36.0,1=>2.0,418=>9.0,234=>18.0,4=>2.0});
    }
     
    #[test]
    fn test_payout_map_33(){
        let r = Roulette::new().payout_map(33);
        assert_eq!(r, hashmap!{419=>9.0,232=>18.0,9=>3.0,5=>2.0,235=>18.0,421=>9.0,12=>3.0,610=>6.0,609=>6.0,257=>18.0,2=>2.0,133=>36.0,312=>12.0,3=>2.0});
    }

    #[test]
    fn test_payout_map_34(){
        let r = Roulette::new().payout_map(34);
        assert_eq!(r, hashmap!{420=>9.0,9=>3.0,233=>18.0,5=>2.0,610=>6.0,134=>36.0,258=>18.0,313=>12.0,1=>2.0,10=>3.0,4=>2.0});
    }
     
    #[test]
    fn test_payout_map_35(){
        let r = Roulette::new().payout_map(35);
        assert_eq!(r, hashmap!{420=>9.0,9=>3.0,5=>2.0,135=>36.0,421=>9.0,610=>6.0,2=>2.0,258=>18.0,11=>3.0,3=>2.0,313=>12.0,259=>18.0,234=>18.0});
    }
     
    #[test]
    fn test_payout_map_36(){
        let r = Roulette::new().payout_map(36);
        assert_eq!(r, hashmap!{9=>3.0,5=>2.0,235=>18.0,421=>9.0,12=>3.0,610=>6.0,313=>12.0,136=>36.0,1=>2.0,259=>18.0,4=>2.0});
    }      
}
