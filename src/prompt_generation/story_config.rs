use rand::Rng;
use serde_json::{json, Value};

use crate::write_duration::{Gender, StoryGenre, StoryType};

pub fn get_voices_google(gender: Gender) -> String {
    let male = vec!["en-US-Casual-K", "en-US-Standard-J", "	en-US-Neural2-D"];
    let female = vec!["en-US-Studio-O", "en-US-Standard-H", "en-US-Journey-O"];

    let voices = json!({
        "male": male,
        "female":female,
    });

    // Retrieve the list of topics for the given genre
    let string_gender = match gender {
        Gender::Male(_) => String::from("male"),
        Gender::Female(_) => String::from("female"),
    };

    // Randomly select a topic from the list
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..2);
    let random_voice = voices.get(string_gender).unwrap()[random_index]
        .as_str()
        .unwrap();

    random_voice.to_string()
}

pub fn get_comment_type() -> std::string::String {
    let comment_type = vec![
        "add some stupid comments  from users inbetween the story.",
        "add some funny comments from usesr inbetween the story.",
        "add some ironic comments from a users inbetween the story.",
        "add some rage bait comments from users inbetween the story.",
    ];

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..comment_type.len());
    return comment_type[num].to_string();
}

pub fn get_genre() -> String {
    let genres = vec![
        StoryGenre::Funny(String::from("funny")),
        StoryGenre::Horrific(String::from("horrific")),
        StoryGenre::Sad(String::from("sad")),
        StoryGenre::Perverted(String::from("perverted")),
        StoryGenre::Crazy(String::from("crazy")),
    ];

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..genres.len());

    // Get the random genre
    let genre = genres[random_index].clone();

    let selected_genre = match genre {
        StoryGenre::Funny(_) => String::from("funny"),
        StoryGenre::Horrific(_) => String::from("horrific"),
        StoryGenre::Sad(_) => String::from("sad"),
        StoryGenre::Perverted(_) => String::from("perverted"),
        StoryGenre::Crazy(_) => String::from("crazy"),
    };

    selected_genre
}

pub fn get_story_type() -> StoryType {
    let narrator = StoryType::Narrator(String::from("narrator"));
    let chat = StoryType::Chat(String::from("chat"));
    let call = StoryType::Call(String::from("call"));
    let comments = StoryType::Comments(String::from("comments"));
    let story_types = vec![comments];

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..story_types.len());

    // Get the random story type
    let story_type = story_types[random_index].clone();

    // Match and return the corresponding string
    story_type
}

pub fn get_context() -> String {
    let contexts = vec![
        "A surprise party went hilariously wrong when the guest of honor showed up a day early.",
        "An office prank resulted in an unintended yet humorous chain reaction.",
        "A cooking disaster turned into a family tradition and favorite story.",
        "A miscommunication during a group project led to a series of comedic errors.",
        "A pet’s unexpected and comical reaction to a new toy or costume.",
        "A mix-up at a wedding led to an unplanned but amusing series of events.",
        "An attempt at DIY home improvement ended in a series of funny mishaps.",
        "An unsettling discovery was made in a seemingly abandoned place.",
        "A night spent alone in a house with a known dark history.",
        "An experience of being followed or watched in an unsettling manner.",
        "An encounter with a real-life stranger danger situation that felt like a thriller.",
        "A shocking secret uncovered about a close friend or family member.",
        "A disturbing event involving an eerie or supernatural element.",
        "A life-threatening situation faced in an isolated or dangerous location.",
        "A painful goodbye that changed the course of someone's life.",
        "A moment of betrayal by someone trusted deeply.",
        "A regretful decision with lasting emotional impact.",
        "An experience of feeling alone despite being surrounded by people.",
        "A scandalous situation involving a public figure or celebrity.",
        "A shocking rumor or revelation about someone well-known.",
        "An inappropriate or taboo conversation overheard unexpectedly.",
        "A hidden aspect of someone’s life that was both surprising and revealing.",
        "An accidental encounter with a deeply awkward or compromising situation.",
        "An unbelievable coincidence that seemed almost too surreal to be true.",
        "A wild decision made that led to an extreme or unexpected outcome.",
        "An eccentric behavior observed that seemed out of the ordinary.",
        "An adventure that turned into a chaotic and memorable saga.",
        "A public behavior so extreme that it became a talking point for everyone who saw it.",
        "An unexpected twist in a personal story that led to a dramatic change.",
    ];

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..contexts.len());

    let context = contexts[random_index];

    context.to_string()
}

pub fn get_topic(genre: String) -> String {
    // Define the topics for each genre
    let topics = json!({
        "funny": [
            "What’s the most embarrassing thing you’ve sent to a boss?",
            "Have you ever had a prank go hilariously wrong?",
            "What’s the weirdest misunderstanding you’ve had with a stranger?",
            "What’s the most absurd excuse you’ve used to skip something?",
            "What’s the funniest thing you’ve seen at a family event?",
            "Tell us about a bizarre roommate experience.",
            "What’s the most awkward first date moment you’ve had?"
        ],
        "horrific": [
            "What’s the scariest thing you’ve faced alone at night?",
            "Have you ever found yourself at a real crime scene?",
            "What’s the creepiest paranormal encounter you’ve had?",
            "Describe a life-threatening situation you’ve experienced.",
            "What’s the most disturbing story you’ve heard about a haunted place?",
            "Have you ever uncovered a shocking secret about someone close to you?",
            "What’s the most terrifying thing you’ve witnessed?"
        ],
        "sad": [
            "What’s the most heartbreaking moment you’ve endured?",
            "Have you ever been betrayed by someone you trusted?",
            "Describe a painful goodbye you’ve had to face.",
            "What’s a regret you wish you could undo?",
            "Have you ever felt alone despite being surrounded by people?",
            "What’s the hardest goodbye you’ve ever experienced?",
            "What’s the toughest challenge you’ve overcome?"
        ],
        "perverted": [
            "What’s the most shocking secret you’ve discovered about someone?",
            "Share a taboo experience that left you questioning morals.",
            "What’s the wildest rumor you’ve heard about a celebrity?",
            "Describe a scandalous situation you’ve been part of.",
            "What’s the most inappropriate conversation you’ve overheard?",
            "Tell us about a hidden aspect of someone’s life that was revealing.",
            "Have you ever accidentally found yourself in an awkward situation?"
        ],
        "crazy": [
            "What’s the most unbelievable coincidence you’ve experienced?",
            "Describe a wild decision you made that went out of control.",
            "What’s the craziest public behavior you’ve witnessed?",
            "Tell us about an extreme situation that spiraled unexpectedly.",
            "What’s the most surreal event you’ve been part of?",
            "What’s the most eccentric behavior you’ve encountered?",
            "Share an adventure that turned into a crazy saga."
        ]
    });

    // Convert the genre to a string
    let genre_str = genre;
    // Retrieve the list of topics for the given genre
    let genre_topics = match topics.get(genre_str) {
        Some(topics_list) => topics_list.as_array().unwrap(),
        None => return "No topics available for this genre.".to_string(),
    };

    // Randomly select a topic from the list
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..genre_topics.len());
    let random_topic = genre_topics[random_index].as_str().unwrap();

    random_topic.to_string()
}

pub fn get_peoples_involved() -> std::string::String {
    let peoples_involved_list = vec![
        "two guys",
        "one woman",
        "two brothers",
        "involving a neighbor ",
        "a couple",
        "step son and mother",
        "step father",
        "mother and child",
        "step father and daughter",
        "a dog and a man",
        "two couples",
        "a best friend",
        "an old man",
        "a young girl",
        "two childs",
        "a child",
        "a single mother",
        "a group of teenagers",
        "a detective",
        "a mysterious traveler",
        "a boss",
        "a mentor",
        "an enemy",
        "a teacher",
        "a stranger",
        "a coworker",
        "a sibling",
        "a roommate",
        "a distant relative",
        "a childhood friend",
        "a rival",
        "a police officer",
        "two strangers",
        "two brothers-in-law",
        "three siblings",
        "a father and his children",
        "two secret lovers",
        "a love triangle",
        "a grandmother and her grandchild",
        "a foster parent and their foster child",
        "two friends with benefits",
        "a roommate and a landlord",
        "cousins",
        "a business partner",
        "a priest",
        "a doctor",
        "two siblings-in-law",
        "a mail carrier",
        "a celebrity",
        "a rival",
        "a shy neighbor",
        "an ex-partner",
        "a landlord",
        "a classmate",
        "a mysterious stranger",
        "a family friend",
        "a secret admirer",
        "a shopkeeper",
        "a neighbor's child",
        "a local celebrity",
        "a nurse",
        "a mail carrier",
        "a retired soldier",
        "a taxi driver",
        "a barista",
        "a street performer",
        "a librarian",
        "a coach",
        "a reporter",
        "a janitor",
        "a chef",
        "a tourist",
        "a pet owner",
        "a park ranger",
        "a firefighter",
        "a babysitter",
        "a gardener",
        "a volunteer",
        "a bus driver",
        "a lawyer",
        "a doctor",
        "a musician",
        "a scientist",
        "a writer",
        "a priest",
        "a fisherman",
        "a fortune teller",
    ];

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..peoples_involved_list.len());
    return peoples_involved_list[num].to_string();
}

pub fn get_situations() -> std::string::String {
    let situations_list = vec![
        "fight about a sandwich",
        "a person that is cheating",
        "someone dying",
        "beating a sickness",
        "a surprise party",
        "a family disagreement",
        "winning a lottery",
        "a secret being revealed",
        "a romantic proposal",
        "a job interview",
        "a broken relationship",
        "a missing pet",
        "a dramatic escape",
        "a major accident",
        "a big misunderstanding",
        "a financial crisis",
        "a sudden move",
        "a heroic rescue",
        "discovering a hidden talent",
        "finding an old letter",
        "uncovering a family secret",
        "an unexpected inheritance",
        "being betrayed by a friend",
        "revealing a surprise pregnancy",
        "getting caught in a lie",
        "unexpectedly meeting a celebrity",
        "a neighbor's bizarre behavior",
        "winning an argument with a boss",
        "facing an embarrassing public moment",
        "a chance encounter with an old crush",
        "finding a lost item of great value",
        "being accused of something you didn’t do",
        "receiving a mysterious message",
        "discovering a secret room in the house",
        "being invited to a stranger's wedding",
        "getting an anonymous gift",
        "overhearing a conversation that changes everything",
        "finding a forgotten diary",
        "being mistaken for someone else",
        "Receiving a mysterious package containing adult toys",
        "Stumbling upon an explicit video in a hidden folder on your computer",
        "Discovering a secret adult club in the city's underground scene",
        "Encountering a stranger who wants to show you their fetish outfit",
        "Being invited to an exclusive party for people with unusual sexual preferences",
        "Finding out that your neighbor is into BDSM and wants to share their experience with you",
        "Meeting someone at a swingers club who knows all your deepest desires",
        "Finding out that your crush is a dominatrix and wants to take you under her wing",
        "Uncovering a hidden world of kink parties and events in the city",
        "A sudden urge to experiment with different types of sex toys and positions",
        "experiencing a random act of kindness",
        "getting stranded during a trip",
        "encountering an old nemesis",
        "discovering a surprising connection with a new friend",
        "facing a major dilemma at work",
        "receiving unexpected advice from a mentor",
        "stumbling upon an unusual hobby",
        "dealing with a bizarre family tradition",
        "confronting a difficult truth about yourself",
        "a sudden change in plans",
        "receiving an unusual request",
        "discovering an anonymous note",
        "experiencing an unexplained event",
        "unexpectedly meeting someone from the past",
        "finding an item with unknown significance",
        "a strange coincidence that can’t be ignored",
        "being offered a life-changing opportunity",
        "witnessing a mysterious act",
        "receiving an enigmatic message",
        "Inadvertently walking in on a private moment",
        "Encountering an unexpected sexual fantasy in a public place",
        "Discovering a forbidden desire hidden deep within yourself",
        "Being coerced into a taboo experience by someone you trust",
        "Finding yourself caught up in a scandalous affair",
        "Unwittingly becoming part of a kinky group activity",
        "Receiving an erotic text message from a mysterious stranger",
        "Accidentally stumbling upon a secret erotica library",
        "Experiencing an unexpected sexual awakening during a mundane event",
        "Receiving an unsolicited erotic gift from a secret admirer",
        "encountering an unexpected obstacle",
        "finding yourself in an unfamiliar place",
        "coming across an old photograph",
        "being involved in an unusual situation",
        "experiencing a surprising turn of events",
        "a stolen glance that leads to more",
        "an unexpected rendezvous",
        "discovering a hidden attraction",
        "a chance encounter that sparks chemistry",
        "a secret admirer reveals themselves",
        "finding a provocative letter",
        "a heated argument that turns passionate",
        "an alluring surprise in a gift",
        "an intimate conversation that escalates",
        "a spontaneous kiss during a heated moment",
        "a mysterious invitation to a private event",
        "discovering hidden desires in an old journal",
        "an unexpected confession of feelings",
        "a forbidden attraction with a colleague",
        "an intense flirtation with a stranger",
        "a stolen glance that leads to a passionate kiss, but then someone walks in",
        "an unexpected rendezvous turns awkward when you realize you know the other person",
        "discovering a hidden attraction only to find out they’re involved with someone close to you",
        "a chance encounter that sparks chemistry, but it’s interrupted by a sudden emergency",
        "a secret admirer reveals themselves, but their identity is not what you expected",
        "finding a provocative letter that leads to an intense connection, but it turns out to be a prank",
        "a heated argument that turns passionate, but suddenly a family member shows up",
        "an alluring surprise in a gift that leads to unexpected complications",
        "an intimate conversation that escalates, only to be interrupted by a pressing phone call",
        "a spontaneous kiss during a heated moment, but it reveals a hidden conflict",
        "a mysterious invitation to a private event that turns out to be a setup for something else",
        "discovering hidden desires in an old journal, but the journal belongs to someone you know",
        "an unexpected confession of feelings that leads to a surprising twist in your relationship",
        "a forbidden attraction with a colleague, only for one of you to be offered a job elsewhere",
        "an intense flirtation with a stranger, but they suddenly vanish without a trace"
    ];

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..situations_list.len());
    return situations_list[num].to_string();
}