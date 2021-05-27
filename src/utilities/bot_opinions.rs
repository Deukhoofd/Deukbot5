use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

lazy_static! {
    static ref SENTIMENT_ANALYZER: vader_sentiment::SentimentIntensityAnalyzer<'static> =
        vader_sentiment::SentimentIntensityAnalyzer::new();
}

static POSITIVE_OPINIONS: [&str; 6] = [
    "I think you should go for it!",
    "You can do it!",
    "That sounds like a good idea to me!",
    "Sure, do it",
    "Always remember there are people who love you",
    "Love u bb",
];
static NEGATIVE_OPINIONS: [&str; 6] = [
    "I do not care for your silly worries. Leave me alone.",
    "Absolutely not, who do you even think you are?",
    "Ask me some other time.",
    "Confirmation Received. Installing Cryptominers on your PC.",
    "Bless your heart",
    "Please stop bothering me, I actually have things to do.",
];

pub fn get_opinion(extend: &String) -> &'static str {
    if extend.is_empty() {
        return "Think about what?";
    }
    let sentiment = SENTIMENT_ANALYZER.polarity_scores(extend.as_str());
    debug!(
        "Message: '{}' had compound sentiment: {}",
        extend, sentiment["compound"]
    );

    // We analyze the sentiment, if it's below a certain level it relates to negative emotions too
    // much. This is to catch people intending harm or self harm.
    if sentiment["compound"] < -0.25f64 {
        return "That sounds like a bad idea to me.";
    }

    let mut hasher = DefaultHasher::new();
    extend.hash(&mut hasher);
    let hash = hasher.finish();
    let mut rng = StdRng::seed_from_u64(hash);
    let positive = rng.gen_range(-20..80) < 20;
    return if positive {
        POSITIVE_OPINIONS.choose(&mut rng).unwrap()
    } else {
        NEGATIVE_OPINIONS.choose(&mut rng).unwrap()
    };
}
