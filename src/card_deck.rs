
pub fn build_card_deck() {
      //           diamonds    clubs       hearts      spades   
let col = [ "\u{2666}", "\u{2663}", "\u{2665}", "\u{2660}" ];
let val = [ "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2", "A" ];


let ranks: Vec<String> = col.iter().flat_map(|c| {
  val.iter().map(|v| v.to_string() + c)
}).collect();

println!("ranks :\n{:?}\n", ranks);

let doc: Vec<String> = col.iter()
                            .flat_map( |c| val.iter()
                                              .clone()
                                              .map( move |v| v.to_string() + c ) )
                            .collect();


println!("ORDERED :\n{:?}\n", doc);
}