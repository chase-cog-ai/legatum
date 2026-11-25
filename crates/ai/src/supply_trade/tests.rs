use super::*;
use legatum_core::supply_trade::PartnerView;
use legatum_core::{GameSetup, new_game, player_view};

const ME: PlayerId = PlayerId(0);
const THEM: PlayerId = PlayerId(1);
const IRON: Resource = Resource::Iron;

/// Our view of a fresh game with `spare` Iron to spare, `idle` Iron users short of supply,
/// 40 Gold a turn and 200 Gold in the treasury. Armies are equal and nobody has VP.
fn view(spare: i32, idle: usize) -> PlayerView {
    let mut v = player_view(&new_game(&GameSetup::all_ai(1)), ME);
    let s = v
        .supply
        .iter_mut()
        .find(|s| s.resource == IRON)
        .expect("Iron supply line");
    s.spare = spare;
    s.idle = (0..idle).map(|i| format!("Pikeman {i}")).collect();
    v.economy.gold = 200;
    v.economy.gold_per_turn.total = 40;
    for p in v.players.iter_mut() {
        p.army = 100;
    }
    v
}

fn offer(v: &mut PlayerView, selling: bool, from: PlayerId, amount: i32, gold: i32) -> u32 {
    let id = v.supply_trade.offers.len() as u32 + 1;
    v.supply_trade.offers.push(SupplyOffer {
        id,
        from,
        to: ME,
        selling,
        resource: IRON,
        amount,
        gold,
        turn: v.turn,
    });
    id
}

/// What we answer the offer `id`: accepted, and the note.
fn answered(orders: &[Order], id: u32) -> (bool, Option<String>) {
    orders
        .iter()
        .find_map(|o| match o {
            Order::AnswerSupplyOffer {
                offer,
                accept,
                note,
            } if *offer == id => Some((*accept, note.clone())),
            _ => None,
        })
        .expect("an answer")
}

/// Their bid to buy `amount` of our Iron for `gold` a turn.
fn bid(v: &mut PlayerView, amount: i32, gold: i32) -> (bool, Option<String>) {
    let id = offer(v, false, THEM, amount, gold);
    answered(&plan(v, None), id)
}

#[test]
fn it_sells_spare_supply_at_its_asking_price_and_not_a_gold_below() {
    // 2 Iron at Peace: 2 × 4 Gold × 150% = 12 Gold a turn.
    assert_eq!(ask(&view(3, 0), THEM, IRON, 2), 12);
    assert_eq!(bid(&mut view(3, 0), 2, 12), (true, None));
    assert_eq!(
        bid(&mut view(3, 0), 2, 11),
        (false, Some("we want 12 Gold a turn for it".into()))
    );
    // No price buys a human the AI's supply for nothing.
    assert!(!bid(&mut view(3, 0), 2, 0).0);
}

#[test]
fn it_keeps_one_supply_spare_however_much_gold_is_offered() {
    assert_eq!(
        bid(&mut view(3, 0), 3, 500),
        (false, Some("we can spare 2 Iron".into()))
    );
    assert_eq!(
        bid(&mut view(1, 0), 1, 500),
        (false, Some("we can spare 0 Iron".into()))
    );
}

#[test]
fn two_bids_in_one_turn_cannot_buy_the_same_supply() {
    let mut v = view(3, 0);
    let a = offer(&mut v, false, THEM, 2, 50);
    let b = offer(&mut v, false, PlayerId(2), 2, 50);
    let orders = plan(&v, None);
    assert!(answered(&orders, a).0);
    assert_eq!(
        answered(&orders, b),
        (false, Some("we promised that supply elsewhere".into()))
    );
}

#[test]
fn it_does_not_arm_a_bigger_army_a_leader_or_its_target() {
    let mut v = view(3, 0);
    v.players[THEM.0 as usize].army = 101;
    assert_eq!(
        bid(&mut v, 2, 500),
        (
            false,
            Some("we will not arm an army larger than ours".into())
        )
    );

    let mut v = view(3, 0);
    let team = v.team_of(THEM);
    let mut standings = (*v.standings).clone();
    let s = standings.iter_mut().find(|s| s.team == team).unwrap();
    s.vp = s.target * 6 / 10;
    v.standings = standings.into();
    assert_eq!(
        bid(&mut v, 2, 500),
        (
            false,
            Some("we will not arm an empire this close to winning".into())
        )
    );

    let mut v = view(3, 0);
    let id = offer(&mut v, false, THEM, 2, 500);
    assert_eq!(
        answered(&plan(&v, Some(team)), id),
        (
            false,
            Some("we will not arm a rival we mean to stop".into())
        )
    );
}

#[test]
fn it_buys_only_what_it_is_short_of_at_a_capped_price() {
    // Two idle Pikemen: 2 Iron is worth at most 2 × 4 × 200% = 16 Gold, and a quarter of 40
    // Gold income is 10.
    let sold = |v: &mut PlayerView, amount, gold| {
        let id = offer(v, true, THEM, amount, gold);
        answered(&plan(v, None), id)
    };
    assert_eq!(bid_limit(&view(0, 2), IRON, 2), 10);
    assert_eq!(sold(&mut view(0, 2), 2, 10), (true, None));
    assert_eq!(
        sold(&mut view(0, 2), 2, 11),
        (false, Some("we would pay 10 Gold a turn for it".into()))
    );
    assert_eq!(
        sold(&mut view(0, 2), 3, 1),
        (false, Some("we only need 2 Iron".into()))
    );
    // Supply it has no use for is not worth 1 Gold: a human cannot sell it a drain.
    assert_eq!(
        sold(&mut view(2, 0), 1, 1),
        (false, Some("we have no use for more Iron now".into()))
    );
    // A richer buyer still stops at twice the fair price.
    let mut rich = view(0, 2);
    rich.economy.gold_per_turn.total = 400;
    assert_eq!(bid_limit(&rich, IRON, 2), 16);
    assert!(!sold(&mut rich, 2, 17).0);
}

#[test]
fn it_bids_for_supply_it_is_short_of_from_an_empire_with_spare() {
    let mut v = view(0, 2);
    v.supply_trade.partners = vec![
        PartnerView {
            player: THEM,
            blocked: None,
            spare: vec![(IRON, 2)],
            uses: vec![IRON],
        },
        PartnerView {
            player: PlayerId(2),
            blocked: None,
            spare: vec![(IRON, 4)],
            uses: vec![IRON],
        },
    ];
    // The one with the most to spare; 2 × 4 × 150% = 12 is over a quarter of income, so the
    // bid waits until income covers it.
    assert_eq!(plan(&v, None), vec![]);
    v.economy.gold_per_turn.total = 48;
    let bid = Order::OfferSupply {
        to: PlayerId(2),
        selling: false,
        resource: IRON,
        amount: 2,
        gold: 12,
    };
    assert_eq!(plan(&v, None), vec![bid.clone()]);
    // Never from the rival it means to stop.
    let rival = v.team_of(PlayerId(2));
    assert!(!plan(&v, Some(rival)).contains(&bid));
    // With that empire out of reach it takes the 1 Iron the other can spare.
    v.supply_trade.partners[1].blocked = Some("you are at War".into());
    assert_eq!(
        plan(&v, None),
        vec![Order::OfferSupply {
            to: THEM,
            selling: false,
            resource: IRON,
            amount: 1,
            gold: 6,
        }]
    );
    // Nothing short, nothing bid.
    let mut calm = view(0, 0);
    calm.supply_trade.partners = v.supply_trade.partners.clone();
    assert_eq!(plan(&calm, None), vec![]);
}

#[test]
fn it_ends_a_trade_that_stopped_making_sense() {
    let trade = |seller, buyer| SupplyTrade {
        id: 7,
        seller,
        buyer,
        resource: IRON,
        amount: 2,
        gold: 12,
        since: 1,
    };
    let end = vec![Order::EndSupplyTrade { trade: 7 }];
    // Selling: kept while we have spare, ended when our own users go short or the buyer
    // outgrows us.
    let mut v = view(1, 0);
    v.supply_trade.trades = vec![trade(ME, THEM)];
    assert_eq!(plan(&v, None), vec![]);
    let mut short = view(0, 1);
    short.supply_trade.trades = vec![trade(ME, THEM)];
    assert_eq!(plan(&short, None), end);
    v.players[THEM.0 as usize].army = 200;
    assert_eq!(plan(&v, None), end);
    // Buying: kept while we use it, ended when we could do without it or run a deficit.
    let mut v = view(1, 0);
    v.supply_trade.trades = vec![trade(THEM, ME)];
    assert_eq!(plan(&v, None), vec![]);
    v.economy.gold_per_turn.total = -1;
    assert_eq!(plan(&v, None), end);
    let mut idle = view(2, 0);
    idle.supply_trade.trades = vec![trade(THEM, ME)];
    assert_eq!(plan(&idle, None), end);
}
