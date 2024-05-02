// use frame_support::parameter_types;
use sp_io::hashing::twox_128;

// parameter_types! {
//     pub const Council: &'static str = "Council";
//     pub const CouncilCollective: &'static str = "CouncilCollective";
//     pub const TechnicalCommittee: &'static str = "TechnicalCommittee";
//     pub const TechCommitteeCollective: &'static str = "TechCommitteeCollective";
// }

fn main() {
    let keys = vec![
        "Council",
        "CouncilCollective",
        "TechnicalCommittee",
        "TechCommitteeCollective",
        "Democracy",
        "VotingOf",
    ];

    for key in keys {
        let h = twox_128(key.as_bytes());
        println!("{:#25} - {:?}", key, hex::encode(h));
    }
    //
}
