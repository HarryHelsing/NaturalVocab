// Print functions
pub fn print_welcome_1() {
    println!(
        "

    =================================
        Welcome to NaturalVocab
    =================================
    
    This app is designed to help you
    learn vocabulary in any language
    in a natural and interesting way.
    
    Upload content that you find
    interesting and engaging, so that
    you learn the words in context,
    in a way that you personally
    connect with.
    
    - Continue (press ENTER to continue)"
    );
}

pub fn print_welcome_2() {
    println!(
        "
    The app primarily tracks the
    amount of words you've become
    comfortable with, it is deisgned
    so that over time the grammar
    of the language is naturally
    acquired.
    
    The app is made to be structured,
    while being flexible. It
    tracks your progress in a gentle,
    self defined way.
    
    - Continue (press ENTER to continue)"
    );
}

pub fn print_lang_select() {
    println!(
        "

    =================================
         Language Selection
    =================================
    
    ... Later add proper alternative language support
"
    );
}

pub fn print_mode_select() {
    println!(
        "

    =================================
            Mode Selection
    =================================
    Select a Mode
    Type 1 then press enter to begin
    Text Upload and Review Mode
    
    Type 2 then press enter to begin
    Word Review and Study Mode
    
    Type 3 then press enter to begin
    Overview Progress Mode
    
    Type 4 then press enter to Exit\n\n
"
    );
}

pub fn print_continue() {
    println!(
        "

    Press enter to continue

    "
    );
}

pub fn print_adding_text() {
    println!(
        "
    How would you like to add text?

    From file? Press 1 then enter
    Write by hand? Press 2 then enter

    "
    );
}

pub fn print_writing_text () {
    println!(
        "
    Type the name of the text title, press enter
    Then type up the body of the text and press enter

    Note: if you type #! within your text it will
          split the text into seperate chunks

    E.G. This is a chunk #! This is also a chunk #!
         This is another chunk #! And another

         "
     );
}














