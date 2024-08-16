use rand::Rng;
use serde_json::{json, Value};

use crate::entities::entities::{Gender, StoryGenre, StoryType};

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
    let story_types = vec![comments, narrator];

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
            "What’s the most embarrassing autocorrect fail you’ve ever had?",
            "Have you ever accidentally sent a text to the wrong person? What happened?",
            "What’s the silliest reason you’ve ever been late to something important?",
            "Describe a moment when you couldn’t stop laughing in a serious situation.",
            "Have you ever stumbled upon something that felt like it was from another world?",
            "What’s the most bizarre conspiracy theory you’ve ever heard?",
            "Have you ever had a dream that felt more real than reality?",
            "What’s the oddest gift you’ve ever received?",
            "What’s the most surreal conversation you’ve had with a stranger?",
            "Have you ever seen something so weird you couldn’t stop thinking about it?",
            "Cooking",
            "Hiking",
            "Birdwatching",
            "Woodworking",
            "Knitting",
            "Fishing",
            "Painting",
            "Pottery",
            "Camping",
            "Baking",
            "Cycling",
            "Reading",
            "Yoga",
            ],
        "horrific": [
            "What’s the most unnerving sound you’ve heard in the middle of the night?",
            "Have you ever encountered a person that gave you a deep sense of dread?",
            "What’s the most terrifying natural disaster you’ve experienced?",
            "Describe a time when you felt like you were being watched.",
            "Have you ever visited a place that felt inherently evil?",
            "What’s the most chilling story you’ve heard about a cursed object?",
            "Have you ever felt a presence you couldn’t explain?",
            "What’s the most frightening thing you’ve experienced while camping?",
            "Have you ever had a dream that made you afraid to sleep again?",
            "What’s the scariest thing you’ve ever found in an abandoned place?",
            "What’s the most frightening experience you’ve ever had?",
            "Describe a time when you felt truly terrified for your safety.",
            "What’s the creepiest thing you’ve ever encountered?",
            "What’s the most unsettling dream you’ve ever had?",
            "What’s the most disturbing experience you’ve had in a seemingly ordinary place?",
            "What’s the most unsettling message you’ve received from a stranger?",
            "Have you ever seen something you couldn’t logically explain?",
            "What’s the most profound experience you’ve had that felt otherworldly?",
            "Have you ever had a moment where you felt a deep connection to something greater?",
            "What’s the most powerful feeling of peace you’ve ever experienced?",
            "Describe a time when you felt like you received a sign from the universe.",
            "What’s the most surreal experience you’ve had in a sacred or spiritual place?",
            "Have you ever felt like you were guided by an unseen force?",
            "What’s the most enlightening conversation you’ve had with a stranger?",
            "Have you ever had a dream that felt like a message?",
            "What’s the most mysterious spiritual experience you’ve had?",
            "Have you ever encountered someone who seemed to radiate an unusual energy?",
            "What’s the most mystical experience you’ve had while meditating or praying?",
            "Have you ever had an encounter with a person who seemed like they were from another time?"
        ],
        "sad": [
            "What’s a moment when you felt utterly defeated?",
            "Have you ever had to let go of a dream you deeply cherished?",
            "Describe a time when you felt like you failed someone you cared about.",
            "Have you ever had to make a choice that broke your heart?",
            "What’s the saddest piece of news you’ve ever received?",
            "What’s the most difficult conversation you’ve ever had?",
            "Have you ever been in a situation where you felt there was no hope?",
            "What’s the most thought-provoking question you’ve ever been asked?",
            "What’s a fundamental question about religion or spirituality that you’ve wrestled with?",
            "How do different religious beliefs influence people's values and behaviors?",
            "What’s a religious or spiritual practice that you find intriguing?",
            "How have your views on religion or spirituality evolved over time?",
            "What’s the role of religion in addressing ethical or moral dilemmas?",
            "What’s the most heartbreaking moment you’ve experienced?",
            "Describe a time when you had to cope with a significant loss.",
            "What’s the most difficult personal challenge you’ve faced?",
            "Have you ever had to say goodbye to someone you loved? How did you handle it?",
            "What’s the most poignant memory you have from a tough time in your life?",
            "Gardening",
            "Scrapbooking",
            "Sewing",
            "Origami",
            "DIY Home Projects",
            "Quilting",
            "Model Building",
            "Calligraphy",
            "Wine Tasting",
            "Archery",
            "Rock Climbing",
            "Surfing",
            "Meditation",
            "Journaling",
            "Leatherworking",
            "Have you ever had an existential crisis that changed your perspective?",
            "What’s the deepest conversation you’ve had with yourself?",
            "Describe a moment when you questioned the nature of reality.",
            "What’s the most profound realization you’ve come to about life?",
            "Have you ever pondered the meaning of life in a way that left you unsettled?",
            "What’s the most significant change in your beliefs you’ve experienced?",
            "What’s the most challenging moral dilemma you’ve faced?",
            "Have you ever had an experience that made you reconsider your purpose?",
            "What’s the most enlightening conversation you’ve had about the nature of existence?",
            "What’s a philosophical question that has kept you up at night?",
            "Have you ever encountered an idea that completely shifted your worldview?"
        ],
        "perverted": [
            "What’s the most surprising thing you’ve found on someone else’s phone?",
            "Describe an awkward encounter you had while eavesdropping.",
            "What’s the most bizarre thing you’ve heard someone confess?",
            "Have you ever witnessed a situation that felt way too intimate?",
            "What’s the most scandalous secret you’ve been sworn to keep?",
            "Have you ever accidentally uncovered someone’s double life?",
            "What’s the strangest fetish you’ve ever heard of?",
            "Have you ever stumbled upon something you wish you hadn’t?",
            "What’s the most uncomfortable situation you’ve found yourself in with a stranger?",
            "Have you ever discovered something shocking in someone else’s belongings?",
            "What’s the weirdest thing you’ve overheard in a public restroom?",
            "Describe a moment when you realized you knew too much about someone’s private life.",
            "What’s the most puzzling mystery you’ve ever encountered?",
            "Have you ever discovered something that made you question everything?",
            "What’s the strangest coincidence you’ve noticed in your life?",
            "Describe a time when you found something that shouldn’t have been there.",
            "What’s the most intriguing secret someone’s ever shared with you?",
            "Have you ever witnessed something that made you rethink reality?",
            "What’s the most fascinating rumor you’ve heard that turned out to be true?",
            "Have you ever found a hidden message or code in something unexpected?",
            "What’s the most perplexing question you’ve never found an answer to?",
            "What’s the most unusual fact you’ve come across?",
            "What’s the most curious encounter you’ve had with a stranger?",
            "Have you ever uncovered a mystery that still haunts you?"

        ],
        "crazy": [
            "What’s the most absurd thing you’ve done on a dare?",
            "Have you ever found yourself in the middle of a spontaneous adventure?",
            "What’s the wildest party you’ve ever attended?",
            "Describe a time when you were caught up in a crazy chain of events.",
            "What’s the most bizarre coincidence that’s ever happened to you?",
            "Have you ever been involved in a situation that felt straight out of a movie?",
            "What’s the most unexpected thing you’ve done out of impulse?",
            "Describe a moment when you thought, ‘This can’t be real life.’",
            "Psychology",
            "Neuroscience",
            "Sociology",
            "What’s the most spontaneous thing you’ve ever done?",
            "Describe a time when you were part of a wild and unexpected adventure.",
            "What’s the most outrageous coincidence you’ve experienced?",
            "What’s the most unexpected turn of events you’ve witnessed?",
            "What’s the craziest thing that’s ever happened to you on a vacation or trip?",
            "Biology",
            "Physics",
            "Chemistry",
            "Anthropology",
            "Ecology",
            "Genetics",
            "Cognitive Science",
            "Behavioral Science",
            "Environmental Science",
            "Geology",
            "What’s a math concept you find especially interesting or useful?",
            "How do you use math in your daily life in ways you wouldn’t expect?",
            "What’s the most challenging math problem you’ve ever tackled?",
            "How do you feel about the role of math in technology and innovation?",
            "What’s a math-related fact or puzzle that you find fascinating?",
            "Astronomy",
            "Mathematics",
            "Linguistics",
            "Political Science",
            "Economics",
            "Philosophy",
            "Artificial Intelligence",
            "What’s the most uncomfortable conversation you’ve had with a family member?",
            "Have you ever accidentally walked into the wrong place at the wrong time?",
            "What’s the most awkward thing you’ve witnessed at a social event?",
            "Have you ever had an embarrassing wardrobe malfunction in public?",
            "What’s the most uncomfortable misunderstanding you’ve been part of?",
            "Describe a time when you were caught in an awkward situation you couldn’t escape.",
            "What’s the most embarrassing way you’ve ever injured yourself?",
            "Have you ever been caught doing something embarrassing when you thought you were alone?"
        ],

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
        "two best friends",
        "a cheating spouse and a suspicious partner",
        "an estranged parent and their child",
        "a stepmother and her resentful stepdaughter",
        "a manipulative coworker and their victim",
        "a toxic ex and their former partner",
        "a married couple and a mysterious third person",
        "a jealous sibling",
        "a secretive spouse and an inquisitive partner",
        "a neighbor with a dark secret",
        "a controlling boss and their employee",
        "a distrusting friend",
        "an overprotective parent",
        "a friend with unrequited love",
        "a distant relative with hidden motives",
        "a recently divorced individual and their ex-in-laws",
        "a stranger who knows too much",
        "a friend who betrayed trust",
        "a single parent and their rebellious teenager",
        "a landlord with something to hide",
        "a neighbor with ulterior motives",
        "a mysterious online friend",
        "a couple dealing with infidelity",
        "a sibling rivalry",
        "a roommate with a dangerous secret",
        "a friend turned enemy",
        "a jilted lover",
        "a mentor with questionable intentions",
        "a group of friends with a shared secret",
        "a step-sibling with hidden resentment",
        "a family torn apart by a lie",
        "a recently discovered half-sibling",
        "a long-lost parent",
        "a married couple facing a scandal",
        "a child and their imaginary friend",
        "a couple and their meddling in-laws",
        "a neighbor involved in a crime",
        "a single mother and her new partner",
        "an ex-partner seeking revenge",
        "a teenager with a double life",
        "a police officer and a suspect with a connection",
        "a teacher and their troubled student",
        "a celebrity caught in a scandal",
        "a mysterious traveler with a hidden past",
        "a widow dealing with a family secret",
        "a volunteer with a hidden agenda",
        "a lawyer defending a guilty client",
        "a retired soldier facing old demons",
        "a detective uncovering a personal mystery",
        "a couple with an open relationship",
        "a young couple with a controversial age gap",
        "a landlord dealing with a troublesome tenant",
        "a shopkeeper and a regular customer with a secret",
        "a nurse discovering a family connection",
        "a child with a secret they can't share",
        "a local celebrity hiding a scandal",
        "a cheating partner and a vengeful ex",
        "a barista who overhears a dangerous conversation",
        "a street performer with a mysterious admirer",
        "a gardener discovering a hidden crime",
        "a tourist caught in a local conspiracy",
        "a coach with a controversial past",
        "a journalist uncovering a community scandal",
        "a scientist with a moral dilemma",
        "a musician with a hidden addiction",
        "a priest with a crisis of faith",
        "a fortune teller predicting a dark future",
        "a fisherman discovering a strange catch",
        "a classmate harboring jealousy",
        "a doctor with a difficult patient",
        "a family friend with an inappropriate crush",
        "a child in a custody battle",
        "a couple adopting a child with a mysterious past",
        "a retiree uncovering a family secret",
        "a babysitter who knows too much",
        "a firefighter saving someone with a dark secret",
        "a best friend hiding their true feelings",
        "a sibling who discovered a parent's affair",
        "a roommate involved in illegal activities",
        "a tourist falling in love with a local",
        "a married couple struggling with infertility",
        "a dog walker discovering something strange",
        "a couple dealing with unexpected pregnancy",
        "a neighbor with a hidden agenda",
        "a teacher involved in a student's scandal",
        "a friend with a hidden addiction",
    ];

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..peoples_involved_list.len());
    return peoples_involved_list[num].to_string();
}

pub fn get_situations() -> std::string::String {
    let situations_list = vec![
        "a heated argument over a trivial matter that spirals out of control",
        "discovering a partner's infidelity through a misplaced text",
        "facing a life-or-death situation with an unlikely ally",
        "a surprise party that goes horribly wrong",
        "a family argument that reveals long-buried secrets",
        "winning the lottery but losing trust in everyone around you",
        "a secret love affair exposed in the most public way",
        "a job interview that turns into a moral dilemma",
        "a romantic proposal that gets interrupted by an unexpected guest",
        "a missing pet that leads to an unexpected reunion",
        "a dramatic escape from a dangerous situation",
        "surviving a major accident that leaves you with a haunting memory",
        "a misunderstanding that tears apart a close relationship",
        "a financial crisis that forces tough decisions",
        "a sudden, unexpected move due to a hidden scandal",
        "a heroic rescue that has unintended consequences",
        "discovering a hidden talent that changes everything",
        "finding an old letter that reveals a shocking truth",
        "uncovering a family secret that could destroy everything",
        "an unexpected inheritance with a dark history",
        "being betrayed by a trusted friend at the worst possible moment",
        "revealing a surprise pregnancy that complicates everything",
        "getting caught in a lie that snowballs out of control",
        "meeting a celebrity who changes your life in unexpected ways",
        "a neighbor's bizarre behavior that hides a sinister truth",
        "winning an argument with a boss but losing something more important",
        "facing an embarrassing public moment that goes viral",
        "a chance encounter with an old crush that reignites old feelings",
        "finding a lost item of great value, but it belongs to someone dangerous",
        "being accused of something you didn’t do, leading to unexpected consequences",
        "receiving a mysterious message that changes your life",
        "discovering a secret room in the house with a dark past",
        "being invited to a stranger's wedding and uncovering a family mystery",
        "getting an anonymous gift with hidden implications",
        "overhearing a conversation that changes everything you thought you knew",
        "finding a forgotten diary that reveals unsettling truths",
        "being mistaken for someone else, leading to a dangerous situation",
        "accidentally walking in on a private moment that leads to a scandal",
        "stumbling upon an explicit video that reveals someone's dark side",
        "discovering a secret adult club that leads to unexpected encounters",
        "encountering a stranger with a fetish that makes you question everything",
        "being invited to an exclusive party with hidden dangers",
        "finding out that your neighbor is involved in a secret underground scene",
        "meeting someone at a swingers club who holds the key to your past",
        "finding out that your crush is involved in something taboo",
        "uncovering a hidden world of forbidden desires in your own city",
        "a sudden urge to explore a hidden side of yourself",
        "experiencing a random act of kindness that leads to unexpected consequences",
        "getting stranded during a trip and discovering something new about yourself",
        "encountering an old nemesis with a shocking secret",
        "discovering a surprising connection with a new friend that complicates things",
        "facing a major dilemma at work that could ruin your career",
        "receiving unexpected advice from a mentor that changes your life",
        "stumbling upon an unusual hobby that leads to a new obsession",
        "dealing with a bizarre family tradition that causes tension",
        "confronting a difficult truth about yourself that you can't ignore",
        "a sudden change in plans that leads to an unexpected adventure",
        "receiving an unusual request that challenges your morals",
        "discovering an anonymous note with dangerous implications",
        "experiencing an unexplained event that defies logic",
        "unexpectedly meeting someone from the past who holds a grudge",
        "finding an item with unknown significance that leads to danger",
        "a strange coincidence that can’t be ignored, leading to a shocking discovery",
        "being offered a life-changing opportunity that comes with a catch",
        "witnessing a mysterious act that puts you in danger",
        "receiving an enigmatic message that draws you into a conspiracy",
        "finding a provocative letter that leads to an intense connection with unexpected consequences",
        "a heated argument that turns passionate but ends in betrayal",
        "an intimate conversation that escalates into a forbidden relationship",
        "a spontaneous kiss during a heated moment that reveals a hidden conflict",
        "discovering hidden desires in an old journal, but the journal belongs to someone you know",
        "an unexpected confession of feelings that leads to a surprising twist in your relationship",
        "a forbidden attraction with a colleague that leads to a dangerous situation",
        "an intense flirtation with a stranger that turns into a life-changing event",
        "a stolen glance that leads to a passionate kiss but results in unintended consequences",
        "an unexpected rendezvous that reveals hidden truths",
        "discovering a hidden attraction that complicates your life",
        "a chance encounter that sparks chemistry but ends in tragedy",
        "a secret admirer reveals themselves with shocking consequences",
        "finding a provocative letter that leads to a dark discovery",
        "a heated argument that turns into a scandalous affair",
        "an alluring surprise in a gift that leads to unexpected complications",
        "an intimate conversation that reveals a hidden agenda",
        "a spontaneous kiss that changes everything you thought you knew",
        "a mysterious invitation to a private event that uncovers a dark secret",
        "discovering hidden desires in an old journal that leads to dangerous consequences",
        "an unexpected confession of feelings that turns your world upside down",
        "a forbidden attraction with a colleague that threatens everything you've worked for",
        "an intense flirtation with a stranger that leads to a dangerous game"
    ];

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..situations_list.len());
    return situations_list[num].to_string();
}
