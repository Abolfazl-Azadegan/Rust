




fn main (){



    /*
    The easiest way to understand enum in Rust is to first understand the programming problem that enum solves.

    You already understand structs and generics, so I'll build from those.

    1. The problem: sometimes a value can be one of several different things

    Imagine you are writing a program that represents the status of a network connection.

    A connection can be:

    connected
    disconnected
    connecting

    One way to represent this badly would be:

    let connected = false;
    let connecting = true;

    But this creates a problem.

    What happens if both become true?

    connected = true;
    connecting = true;

    Now your program represents an impossible state.

    You could add more variables and rules:

    let connected = false;
    let connecting = false;
    let disconnected = true;

    But now the programmer has to manually make sure that exactly one of those states is true.

    Rust gives us a much better way to represent this concept:

    enum ConnectionStatus {
        Connected,
        Disconnected,
        Connecting,
    }

    Now a ConnectionStatus value can be one of these three possibilities.

    For example:

    let status = ConnectionStatus::Connected;

    or:

    let status = ConnectionStatus::Disconnected;

    or:

    let status = ConnectionStatus::Connecting;

    But it cannot simultaneously be two of them.

    That is the fundamental idea behind an enum.

    2. What does enum mean?

    enum is short for enumeration.

    An enum defines a type that can have one value from a fixed set of possible variants.

    For example:

    enum Direction {
        North,
        South,
        East,
        West,
    }

    This creates a new type:

    Direction

    And that type has four possible variants:

    Direction::North
    Direction::South
    Direction::East
    Direction::West

    Notice the syntax:

    Direction::North

    Direction is the enum type.

    North is one of its variants.

    The :: means that North belongs to Direction.

    So:

    let direction = Direction::North;

    means:

    Create a value whose type is Direction, and whose particular variant is North.

    3. Why not just use strings?

    You might ask:

    Why don't I just write "north"?

    You could:

    let direction = "north";

    But now Rust doesn't know that only these values are valid:

    north
    south
    east
    west

    You could accidentally write:

    let direction = "banana";

    and Rust would happily accept it because "banana" is a valid string.

    With an enum:

    enum Direction {
        North,
        South,
        East,
        West,
    }

    Rust knows that a Direction can only be one of those four variants.

    This is one of the major benefits of enums:

    They let you express the valid possibilities directly in the type system.

    4. An enum is not a struct

    You learned that a struct groups multiple pieces of data together.

    For example:

    struct Employee {
        name: String,
        age: u32,
    }

    A value of Employee contains both:

    name
    age

    An enum is different.

    An enum says:

    This value can be one of several alternatives.

    For example:

    enum Direction {
        North,
        South,
        East,
        West,
    }

    A particular Direction value is one of those variants.

    It isn't all four simultaneously.

    So a useful way to think about the difference is:

    Struct:

    "This thing has these pieces of information."

    Enum:

    "This thing can be one of these different kinds."

    5. The variants don't have to be empty

    This is where Rust enums become extremely powerful.

    Consider a network message.

    Suppose your program receives different kinds of messages:

    text message
    integer message
    error message

    You could define:

    enum Message {
        Text(String),
        Number(i32),
        Error(String),
    }

    Now each variant can contain its own data.

    For example:

    let message1 = Message::Text(String::from("Hello"));

    let message2 = Message::Number(42);

    let message3 = Message::Error(String::from("Connection failed"));

    All three variables have the same overall type:

    Message

    But they contain different variants.

    6. This is extremely useful

    Look at these three values:

    let message1 = Message::Text(String::from("Hello"));
    let message2 = Message::Number(42);
    let message3 = Message::Error(String::from("Connection failed"));

    Their types are all:

    Message

    But internally they represent different situations.

    message1 means:

    "I have a text message."

    message2 means:

    "I have a number."

    message3 means:

    "I have an error message."

    This gives us something very useful:

    One variable/type can represent several different possibilities while still being type-safe.

    7. How do we know which variant we have?

    This is where match comes in.

    You will see enums and match together very frequently in Rust.

    For example:

    enum Direction {
        North,
        South,
        East,
        West,
    }

    Then:

    let direction = Direction::North;

    match direction {
        Direction::North => {
            println!("Going north");
        }

        Direction::South => {
            println!("Going south");
        }

        Direction::East => {
            println!("Going east");
        }

        Direction::West => {
            println!("Going west");
        }
    }

    The match asks:

    Which variant is this value?

    If direction is:

    Direction::North

    Rust executes:

    Direction::North => {
        println!("Going north");
    }
    8. Why is match so important?

    Because enums represent alternatives.

    Suppose:

    enum ConnectionStatus {
        Connected,
        Disconnected,
        Connecting,
    }

    Your program needs to do something different depending on the status.

    You can write:

    let status = ConnectionStatus::Connecting;

    match status {
        ConnectionStatus::Connected => {
            println!("Connection is active");
        }

        ConnectionStatus::Disconnected => {
            println!("Connection is down");
        }

        ConnectionStatus::Connecting => {
            println!("Trying to connect");
        }
    }

    Rust has an important feature here:

    match normally requires you to handle every possible variant.

    So if you forget:

    ConnectionStatus::Connecting

    Rust will warn/error that your match is not exhaustive.

    This is extremely useful in real software.

    Suppose six months later you add:

    enum ConnectionStatus {
        Connected,
        Disconnected,
        Connecting,
        Reconnecting,
    }

    Rust can tell you that existing match expressions need to account for the new possibility.

    This prevents many bugs.

    9. Enums become much more powerful when variants contain different data

    Consider a network packet.

    Maybe your system receives:

    Login request
    Data packet
    Disconnect request
    Error

    Each one needs different information.

    We can represent that naturally:

    enum Packet {
        Login {
            username: String,
            password: String,
        },

        Data {
            source: String,
            payload: String,
        },

        Disconnect {
            reason: String,
        },

        Error {
            code: u32,
            message: String,
        },
    }

    Now look at the different variants.

    Login contains:

    username
    password

    Data contains:

    source
    payload

    Disconnect contains:

    reason

    Error contains:

    code
    message

    These are all different structures, but they belong to the same conceptual category:

    Packet

    That's a very important use case for enums.

    10. Creating those values

    You can create:

    let login_packet = Packet::Login {
        username: String::from("efe"),
        password: String::from("1234"),
    };

    Or:

    let data_packet = Packet::Data {
        source: String::from("192.168.1.10"),
        payload: String::from("Hello"),
    };

    Or:

    let error_packet = Packet::Error {
        code: 500,
        message: String::from("Internal server error"),
    };

    Notice something important.

    All of these variables have type:

    Packet

    Even though their contents are completely different.

    11. Processing them

    Now:

    match data_packet {
        Packet::Login {
            username,
            password,
        } => {
            println!("Login from {}", username);
        }

        Packet::Data {
            source,
            payload,
        } => {
            println!("Data from {}: {}", source, payload);
        }

        Packet::Disconnect {
            reason,
        } => {
            println!("Disconnected: {}", reason);
        }

        Packet::Error {
            code,
            message,
        } => {
            println!("Error {}: {}", code, message);
        }
    }

    This is called pattern matching.

    Rust looks at the actual variant and extracts its data.

    For example, if the value is:

    Packet::Error {
        code: 500,
        message: String::from("Internal server error"),
    }

    then this pattern:

    Packet::Error {
        code,
        message,
    }

    extracts those fields so you can use:

    code
    message

    inside that branch.

    12. A very important real-world example: Option

    One of the most important enums in Rust is actually built into the language/library:

    Option<T>

    You will encounter this everywhere in Rust.

    It represents:

    "There may or may not be a value."

    It has two variants:

    enum Option<T> {
        Some(T),
        None,
    }

    Don't worry about the exact internal implementation yet. Focus on the idea.

    There are two possibilities:

    Some(value)

    means:

    A value exists.

    And:

    None

    means:

    No value exists.

    13. Why do we need Option?

    Imagine a function that searches for a user.

    Maybe the user exists.

    Maybe they don't.

    In some languages, you might return:

    null

    when the user doesn't exist.

    That creates a major class of bugs:

    The programmer forgets to check whether the result is null and tries to use it anyway.

    Rust handles this differently.

    You can return:

    Option<String>

    which explicitly tells the caller:

    "This function might give you a String, but it might also give you nothing."

    For example:

    fn find_username(user_id: u32) -> Option<String> {
        if user_id == 10 {
            Some(String::from("efe"))
        } else {
            None
        }
    }

    Then:

    let result = find_username(10);

    result is:

    Option<String>

    It is either:

    Some(String)

    or:

    None
    14. Handling Option

    You can use match:

    match result {
        Some(username) => {
            println!("User is {}", username);
        }

        None => {
            println!("User does not exist");
        }
    }

    Rust forces you to think about both possibilities.

    That's one of the biggest reasons enums are useful:

    They make different possible states explicit in the type.

    15. Another extremely important enum: Result

    Another major Rust type is:

    Result<T, E>

    It represents an operation that can either:

    succeed
    fail

    Conceptually:

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    For example, suppose you try to read a file.

    It might succeed:

    Ok(file_contents)

    or fail:

    Err(error)

    You can process it:

    match result {
        Ok(contents) => {
            println!("File contents: {}", contents);
        }

        Err(error) => {
            println!("Could not read file: {}", error);
        }
    }

    This is enormously important in Rust programming.

    Instead of silently ignoring errors or relying on null, Rust makes the possibility of failure explicit.

    16. Option and Result show the real philosophy behind enums

    This is why I don't want you to think of enum as simply:

    "A list of names."

    That's the beginner definition, but Rust enums are much more powerful.

    The deeper idea is:

    An enum lets you model different possible states of a value as a single type.

    For example:

    Connection
    enum ConnectionStatus {
        Connected,
        Connecting,
        Disconnected,
    }
    User search
    Option<User>

    means:

    User exists
    or
    No user
    File operation
    Result<String, Error>

    means:

    Operation succeeded
    or
    Operation failed
    Network packet
    enum Packet {
        Login { ... },
        Data { ... },
        Disconnect { ... },
        Error { ... },
    }

    means:

    This packet is one of several possible packet types.
    17. Enum versus struct

    This distinction is extremely important.

    Suppose you have:

    struct Employee {
        name: String,
        age: u32,
        salary: u32,
    }

    This says:

    Every Employee has a name, age, and salary.

    All three fields exist simultaneously.

    Now suppose you have:

    enum LoginResult {
        Success,
        WrongPassword,
        UserNotFound,
        AccountLocked,
    }

    This says:

    A LoginResult is one of these possible states.

    Only one variant represents the value at a time.

    So:

    struct

    is generally about:

    AND

    An Employee has a name and an age and a salary.

    Whereas:

    enum

    is generally about:

    OR

    A login result is Success or WrongPassword or UserNotFound or AccountLocked.

    This isn't a formal Rust definition, but it's an extremely useful way to understand the design.

    18. Enum variants can even contain structs-like fields

    You can combine structs and enums.

    For example:

    enum NetworkEvent {
        Connected {
            ip_address: String,
        },

        DataReceived {
            source: String,
            data: Vec<u8>,
        },

        Disconnected {
            reason: String,
        },
    }

    Now your network program can have a single type:

    NetworkEvent

    while each event contains exactly the information relevant to that event.

    For example:

    let event = NetworkEvent::DataReceived {
        source: String::from("192.168.32.100"),
        data: vec![1, 2, 3, 4],
    };

    Then:

    match event {
        NetworkEvent::Connected { ip_address } => {
            println!("Connected to {}", ip_address);
        }

        NetworkEvent::DataReceived { source, data } => {
            println!("Received {} bytes from {}", data.len(), source);
        }

        NetworkEvent::Disconnected { reason } => {
            println!("Disconnected: {}", reason);
        }
    }

    This kind of design appears frequently in systems programming.

    19. Enums can also have methods

    Just like structs, enums can have an impl block.

    For example:

    enum ConnectionStatus {
        Connected,
        Disconnected,
        Connecting,
    }

    Then:

    impl ConnectionStatus {
        fn description(&self) -> &str {
            match self {
                ConnectionStatus::Connected => "Connection is active",
                ConnectionStatus::Disconnected => "Connection is down",
                ConnectionStatus::Connecting => "Trying to connect",
            }
        }
    }

    Now:

    let status = ConnectionStatus::Connected;

    println!("{}", status.description());

    outputs:

    Connection is active

    So enums aren't just passive data. You can implement behavior for them just like structs.

    20. The most important practical use case

    Suppose you're writing a telecom system.

    A function needs to report the result of a call attempt.

    You could create an enum:

    enum CallResult {
        Connected,
        Busy,
        NoAnswer,
        Rejected,
        NetworkError,
    }

    Then:

    fn make_call() -> CallResult {
        // ...
    }

    Now whoever calls this function knows:

    make_call() doesn't return an arbitrary string. It returns one of the defined CallResult states.

    Then:

    let result = make_call();

    match result {
        CallResult::Connected => {
            println!("Call connected");
        }

        CallResult::Busy => {
            println!("User is busy");
        }

        CallResult::NoAnswer => {
            println!("No answer");
        }

        CallResult::Rejected => {
            println!("Call rejected");
        }

        CallResult::NetworkError => {
            println!("Network error");
        }
    }

    This is much safer than:

    let result = "busy";

    because the compiler knows what a CallResult is allowed to be.

    21. The deeper reason Rust uses enums so heavily

    Rust puts a lot of emphasis on making invalid states difficult or impossible to represent.

    Suppose you represent a connection using:

    let connected: bool;
    let connecting: bool;
    let disconnected: bool;

    There are many combinations:

    true / false / false
    false / true / false
    false / false / true
    true / true / false
    true / false / true
    ...

    Some combinations don't make sense.

    With:

    enum ConnectionStatus {
        Connected,
        Connecting,
        Disconnected,
    }

    the type itself prevents those combinations.

    A variable has exactly one ConnectionStatus variant.

    So enums help you move the rules of your program into the type system.

    That is a major part of Rust's design philosophy.

    22. What you should remember

    For now, don't try to memorize all the syntax.

    Remember these four ideas.

    First

    An enum creates a new type with a fixed set of possible variants:

    enum Direction {
        North,
        South,
        East,
        West,
    }
    Second

    A value has one variant at a time:

    let direction = Direction::North;
    Third

    Variants can contain data:

    enum Message {
        Text(String),
        Number(i32),
        Error(String),
    }
    Fourth

    match lets you handle the different possibilities:

    match message {
        Message::Text(text) => {
            println!("{}", text);
        }

        Message::Number(number) => {
            println!("{}", number);
        }

        Message::Error(error) => {
            println!("{}", error);
        }
    }

    And this combination:

    enum
    +
    match

    is one of the most important patterns you will learn in Rust.

    One sentence that captures the whole idea

    A struct says "this value contains these pieces of data," while an enum says "this value can be one of 
    these different kinds of things."

    And the reason Rust enums are so useful is that the compiler knows all those possible kinds, allowing Rust 
    to force your code to handle them explicitly instead of letting impossible or forgotten states silently slip through.
     */





     /*
     
     you can absolutely check the data inside an enum variant and make decisions based on that data. I think the confusing part is that there are actually two levels of information in an enum.

    Let's use your exact example and separate those two levels.

    1. First, look at the enum itself

    We had:

    enum NetworkEvent {
        Connected {
            ip_address: String,
        },

        DataReceived {
            source: String,
            data: Vec<u8>,
        },

        Disconnected {
            reason: String,
        },
    }

    There are three variants:

    NetworkEvent::Connected
    NetworkEvent::DataReceived
    NetworkEvent::Disconnected

    But notice that each variant can contain different data.

    Connected

    Contains:

    ip_address: String
    DataReceived

    Contains:

    source: String
    data: Vec<u8>
    Disconnected

    Contains:

    reason: String

    So there are two different questions you can ask about an enum value:

    Question 1: Which variant is it?

    Is it:

    Connected

    or:

    DataReceived

    or:

    Disconnected

    Question 2: What data does that variant contain?

    For example, if it is DataReceived:

    source = "192.168.32.100"
    data = [1, 2, 3, 4]

    match can deal with both.

    2. Let's look at this line
    let event = NetworkEvent::DataReceived {
        source: String::from("192.168.32.100"),
        data: vec![1, 2, 3, 4],
    };

    We are creating an enum value.

    The type of event is:

    NetworkEvent

    And its particular variant is:

    NetworkEvent::DataReceived

    And that variant contains:

    source = "192.168.32.100"
    data = [1, 2, 3, 4]

    So conceptually, you can think:

    event is a NetworkEvent, and right now its variant is DataReceived, with source and data inside it.

    3. What does match event do?

    Now:

    match event {

    means:

    "Look at the event value and determine which pattern it matches."

    For example:

    match event {
        NetworkEvent::Connected { ip_address } => {
            // ...
        }

        NetworkEvent::DataReceived { source, data } => {
            // ...
        }

        NetworkEvent::Disconnected { reason } => {
            // ...
        }
    }

    Rust checks the variant first.

    Our event is:

    NetworkEvent::DataReceived

    So this branch matches:

    NetworkEvent::DataReceived { source, data } => {

    The other two branches don't execute.

    4. But what are source and data doing there?

    This is the important part.

    Look at:

    NetworkEvent::DataReceived { source, data }

    We are not just saying:

    "Check whether this is DataReceived."

    We are also saying:

    "If it is DataReceived, take the values stored in source and data and make them available inside this block."

    This is called destructuring or pattern matching.

    So if:

    event

    contains:

    NetworkEvent::DataReceived {
        source: String::from("192.168.32.100"),
        data: vec![1, 2, 3, 4],
    }

    then:

    NetworkEvent::DataReceived { source, data }

    extracts those values.

    Inside the block:

    {
        println!("Received {} bytes from {}", data.len(), source);
    }

    you can use:

    source

    and:

    data
    5. So yes: you can check the values too

    This is exactly what you were asking.

    Suppose you don't merely want to know:

    "Was this a DataReceived event?"

    Suppose you want:

    "Was data received from 192.168.32.100?"

    You can do that.

    For example:

    match event {
        NetworkEvent::DataReceived { source, data } => {
            if source == "192.168.32.100" {
                println!("Data came from our server");
            } else {
                println!("Data came from another source");
            }
        }

        NetworkEvent::Connected { ip_address } => {
            println!("Connected to {}", ip_address);
        }

        NetworkEvent::Disconnected { reason } => {
            println!("Disconnected: {}", reason);
        }
    }

    Now we have two levels of decision-making.

    First:

    NetworkEvent::DataReceived

    asks:

    Is this event a DataReceived event?

    Then:

    if source == "192.168.32.100"

    asks:

    What is the value of source?

    If it matches the IP address, we do one thing.

    Otherwise, we do another thing.

    6. But you can actually check the data directly inside match

    You don't necessarily need an if.

    Rust allows you to put conditions into match arms using a match guard.

    For example:

    match event {
        NetworkEvent::DataReceived { source, data }
            if source == "192.168.32.100" =>
        {
            println!("Data came from our server");
        }

        NetworkEvent::DataReceived { source, data } => {
            println!("Data came from {}", source);
        }

        NetworkEvent::Connected { ip_address } => {
            println!("Connected to {}", ip_address);
        }

        NetworkEvent::Disconnected { reason } => {
            println!("Disconnected: {}", reason);
        }
    }

    Now something interesting happens.

    Suppose:

    event

    is:

    NetworkEvent::DataReceived {
        source: String::from("192.168.32.100"),
        data: vec![1, 2, 3, 4],
    }

    Rust examines the first branch:

    NetworkEvent::DataReceived { source, data }
        if source == "192.168.32.100"

    Two things must be true:

    The variant must be DataReceived.
    source must equal "192.168.32.100".

    Both are true.

    Therefore this branch executes:

    println!("Data came from our server");
    7. You can check data too

    Suppose your enum is:

    enum NetworkEvent {
        Connected {
            ip_address: String,
        },

        DataReceived {
            source: String,
            data: Vec<u8>,
        },

        Disconnected {
            reason: String,
        },
    }

    You could do:

    match event {
        NetworkEvent::DataReceived { source, data }
            if source == "192.168.32.100" && data.len() > 1000 =>
        {
            println!("Large packet from our server");
        }

        NetworkEvent::DataReceived { source, data } => {
            println!("Normal data packet from {}", source);
        }

        NetworkEvent::Connected { ip_address } => {
            println!("Connected to {}", ip_address);
        }

        NetworkEvent::Disconnected { reason } => {
            println!("Disconnected: {}", reason);
        }
    }

    Now you're checking:

    which variant?
    what is the source?
    how large is the data?

    So yes, your understanding is basically correct.

    8. There is an even more direct way: matching the actual value

    You can also match against specific values.

    For example, suppose you have:

    enum Connection {
        Connected {
            ip_address: String,
        },

        Disconnected {
            reason: String,
        },
    }

    You can write:

    match event {
        NetworkEvent::Connected { ip_address } => {
            if ip_address == "192.168.32.100" {
                println!("Connected to our server");
            } else {
                println!("Connected to another server");
            }
        }

        NetworkEvent::Disconnected { reason } => {
            println!("Disconnected: {}", reason);
        }

        NetworkEvent::DataReceived { source, data } => {
            println!("Received data from {}", source);
        }
    }

    This is probably the easiest version for you to understand initially:

    match event {
        NetworkEvent::DataReceived { source, data } => {
            // We know the variant.
            // Now inspect source/data with normal if statements.
        }

        NetworkEvent::Connected { ip_address } => {
            // We know the variant.
            // Now inspect ip_address.
        }

        NetworkEvent::Disconnected { reason } => {
            // We know the variant.
            // Now inspect reason.
        }
    }
    9. But your wording has one small misunderstanding

    You said:

    "in enum i have 3 possible data and those 3 possible data can have 3 possible variables"

    Not exactly.

    You have three possible variants, not three possible data values.

    Your enum is:

    enum NetworkEvent {
        Connected {
            ip_address: String,
        },

        DataReceived {
            source: String,
            data: Vec<u8>,
        },

        Disconnected {
            reason: String,
        },
    }

    The three variants are:

    Connected
    DataReceived
    Disconnected

    And each variant has its own fields.

    So:

    Connected

    has one field:

    ip_address

    DataReceived has two:

    source
    data

    Disconnected has one:

    reason

    And one NetworkEvent value is one of those variants.

    10. Let's make a concrete example

    Imagine your program receives three events one after another.

    First:

    let event1 = NetworkEvent::Connected {
        ip_address: String::from("192.168.32.100"),
    };

    Second:

    let event2 = NetworkEvent::DataReceived {
        source: String::from("192.168.32.100"),
        data: vec![1, 2, 3, 4],
    };

    Third:

    let event3 = NetworkEvent::Disconnected {
        reason: String::from("Connection timeout"),
    };

    All three have the same overall type:

    NetworkEvent

    But each has a different variant.

    Now you can process each one:

    match event2 {
        NetworkEvent::Connected { ip_address } => {
            println!("Connected to {}", ip_address);
        }

        NetworkEvent::DataReceived { source, data } => {
            println!("Received data from {}", source);

            if source == "192.168.32.100" {
                println!("This is our server");
            }

            if data.len() > 100 {
                println!("Large packet");
            }
        }

        NetworkEvent::Disconnected { reason } => {
            println!("Disconnected because: {}", reason);
        }
    }

    For event2, Rust enters:

    NetworkEvent::DataReceived { source, data } => {

    because that's the variant.

    Then you can inspect:

    source

    and:

    data

    and perform whatever logic you want.

    11. So what exactly does match do?

    This is the key distinction.

    match is not simply "check the value of the enum."

    More precisely:

    match compares a value against a set of patterns and executes the code belonging to the first matching pattern.

    With an enum, those patterns commonly identify the enum variant.

    For example:

    match event {
        NetworkEvent::Connected { ip_address } => { ... }

        NetworkEvent::DataReceived { source, data } => { ... }

        NetworkEvent::Disconnected { reason } => { ... }
    }

    The patterns are:

    NetworkEvent::Connected { ip_address }
    NetworkEvent::DataReceived { source, data }
    NetworkEvent::Disconnected { reason }

    The first part identifies the variant.

    The fields extract the data stored inside that variant.

    Then you can use that data.

    12. And match can be much more specific

    You can eventually write patterns that inspect the actual contents too.

    For example, with a simpler enum:

    enum Message {
        Number(i32),
        Text(String),
    }

    You can do:

    match message {
        Message::Number(0) => {
            println!("The number is exactly zero");
        }

        Message::Number(number) => {
            println!("The number is {}", number);
        }

        Message::Text(text) => {
            println!("The text is {}", text);
        }
    }

    Notice this:

    Message::Number(0)

    doesn't merely say:

    "Is this a Number?"

    It says:

    "Is this a Number whose actual value is exactly 0?"

    Whereas:

    Message::Number(number)

    means:

    "Is this a Number? If so, give me its value and call it number inside this branch."

    That is the deeper power of pattern matching.

    13. And this is why enums + match are so powerful

    Suppose your telecom program receives events:

    enum Event {
        CallStarted {
            caller: String,
            destination: String,
        },

        SmsReceived {
            source: String,
            destination: String,
            message: String,
        },

        CallEnded {
            caller: String,
            destination: String,
        },
    }

    Now you could do:

    match event {
        Event::CallStarted {
            caller,
            destination,
        } => {
            if destination == "1234" {
                println!("Special service number called");
            }
        }

        Event::SmsReceived {
            source,
            destination,
            message,
        } => {
            if destination == "5000" {
                println!("SMS sent to service number");
            }

            if source == "192.168.32.100" {
                println!("SMS came from our server");
            }

            println!("Message: {}", message);
        }

        Event::CallEnded {
            caller,
            destination,
        } => {
            println!("Call ended: {} -> {}", caller, destination);
        }
    }

    Here you're doing exactly what you described:

    Check which kind of event it is.
    Extract the variables belonging to that variant.
    Inspect the values of those variables.
    Perform different actions depending on those values.

    So your understanding is on the right track. The important distinction is that the enum variant tells you what kind 
    of thing you have, while the fields inside that variant contain the data you can then inspect. match can participate in 
    both decisions, either directly through patterns/guards or by extracting the fields and using if logic inside the 
    matching branch.

      */


    /*
    
    Yes. You are right to question this. In my previous explanation, I mixed two different things and made it more confusing.

    You can absolutely use match itself to check the values stored inside enum variants, without using if.

    The important thing is to understand patterns.

    Let's forget the previous example for a moment and build this from the simplest possible case.

    1. Start with an enum that contains values

    Consider:

    enum Message {
        Number(i32),
        Text(String),
    }

    There are two variants:

    Message::Number(...)
    Message::Text(...)

    The parentheses mean that each variant contains a value.

    For example:

    let message1 = Message::Number(0);
    let message2 = Message::Number(50);
    let message3 = Message::Text(String::from("Hello"));

    All three are:

    Message

    But their contents are different.

    2. match can check the value inside the variant

    Look at this:

    match message1 {
        Message::Number(0) => {
            println!("The number is zero");
        }

        Message::Number(50) => {
            println!("The number is fifty");
        }

        Message::Number(number) => {
            println!("Some other number: {}", number);
        }

        Message::Text(text) => {
            println!("Text: {}", text);
        }
    }

    There is no if here.

    And match is checking both:

    Which variant?
    What value is inside the variant?

    For example:

    Message::Number(0)

    means:

    The variant must be Number AND the value inside it must be exactly 0.

    3. Let's execute it

    Suppose:

    let message1 = Message::Number(0);

    Then:

    match message1 {

    Rust checks the first pattern:

    Message::Number(0)

    Does message1 have the Number variant?

    Yes.

    Does the value inside it equal 0?

    Yes.

    Therefore this executes:

    println!("The number is zero");

    The other branches aren't executed.

    4. What if the value is 50?

    Suppose:

    let message1 = Message::Number(50);

    Rust checks:

    Message::Number(0)

    No. The value isn't 0.

    Then:

    Message::Number(50)

    Yes.

    So:

    The number is fifty

    is printed.

    Again, no if.

    5. What does Message::Number(number) mean?

    Now look at this:

    Message::Number(number)

    This is different from:

    Message::Number(0)

    0 is a specific value.

    number is a new variable that captures whatever value is inside.

    For example:

    let message1 = Message::Number(123);

    and:

    match message1 {
        Message::Number(number) => {
            println!("{}", number);
        }

        Message::Text(text) => {
            println!("{}", text);
        }
    }

    When this branch runs:

    Message::Number(number)

    the variable:

    number

    gets the value:

    123

    So:

    println!("{}", number);

    prints:

    123

    This is how you get access to the value stored inside an enum variant.

    6. Now let's return to your NetworkEvent

    Your original enum was:

    enum NetworkEvent {
        Connected {
            ip_address: String,
        },

        DataReceived {
            source: String,
            data: Vec<u8>,
        },

        Disconnected {
            reason: String,
        },
    }

    This looks different because the variants contain named fields instead of one unnamed value.

    For example:

    NetworkEvent::Connected {
        ip_address: String,
    }

    and:

    NetworkEvent::DataReceived {
        source: String,
        data: Vec<u8>,
    }

    and:

    NetworkEvent::Disconnected {
        reason: String,
    }
    7. You can match the actual fields

    Suppose we create:

    let event = NetworkEvent::DataReceived {
        source: String::from("192.168.32.100"),
        data: vec![1, 2, 3, 4],
    };

    We can write:

    match event {
        NetworkEvent::DataReceived {
            source: String::from("192.168.32.100"),
            data,
        } => {
            println!("Data came from our server");
        }

        NetworkEvent::DataReceived {
            source,
            data,
        } => {
            println!("Data came from another source: {}", source);
        }

        NetworkEvent::Connected {
            ip_address,
        } => {
            println!("Connected to {}", ip_address);
        }

        NetworkEvent::Disconnected {
            reason,
        } => {
            println!("Disconnected: {}", reason);
        }
    }

    However, there is an important Rust detail here: you cannot generally use an expression like String::from(...) 
    as a pattern. So for a String field, you normally use a pattern such as source if source == "..." or, more naturally, 
    match on &str/use a guard.

    But this is exactly why your Number(0) example works so cleanly: 0 is a valid literal pattern.

    So let's use values that Rust can directly match as patterns.

    8. Use an enum with simple values first

    For example:

    enum NetworkEvent {
        Connected {
            server_id: u32,
        },

        DataReceived {
            source_id: u32,
            data_size: u32,
        },

        Disconnected {
            reason_code: u32,
        },
    }

    Now we can directly match values.

    Suppose:

    let event = NetworkEvent::DataReceived {
        source_id: 100,
        data_size: 500,
    };

    We can write:

    match event {
        NetworkEvent::DataReceived {
            source_id: 100,
            data_size: 500,
        } => {
            println!("Server 100 sent exactly 500 bytes");
        }

        NetworkEvent::DataReceived {
            source_id: 100,
            data_size,
        } => {
            println!("Server 100 sent {} bytes", data_size);
        }

        NetworkEvent::DataReceived {
            source_id,
            data_size,
        } => {
            println!("Some server sent {} bytes", data_size);
        }

        NetworkEvent::Connected {
            server_id,
        } => {
            println!("Connected to server {}", server_id);
        }

        NetworkEvent::Disconnected {
            reason_code,
        } => {
            println!("Disconnected. Reason code: {}", reason_code);
        }
    }

    Now there is no if anywhere.

    match itself is doing the checking.

    9. What does this pattern mean?

    Look carefully at:

    NetworkEvent::DataReceived {
        source_id: 100,
        data_size: 500,
    }

    This means:

    The enum variant must be DataReceived, AND source_id must be exactly 100, AND data_size must be exactly 500.

    So if the value is:

    NetworkEvent::DataReceived {
        source_id: 100,
        data_size: 500,
    }

    the first branch matches.

    If the value is:

    NetworkEvent::DataReceived {
        source_id: 100,
        data_size: 700,
    }

    the first branch does not match because:

    700 != 500

    Then Rust moves to:

    NetworkEvent::DataReceived {
        source_id: 100,
        data_size,
    }

    This one matches because:

    source_id == 100

    and data_size can be anything.

    10. This is the same idea as Number(0)

    Compare these:

    Message::Number(0)

    and:

    NetworkEvent::DataReceived {
        source_id: 100,
        data_size: 500,
    }

    They are doing exactly the same kind of thing.

    First:
    Message::Number(0)

    means:

    Variant is Number AND contained value is 0.

    Second:
    NetworkEvent::DataReceived {
        source_id: 100,
        data_size: 500,
    }

    means:

    Variant is DataReceived AND source_id is 100 AND data_size is 500.

    That's the connection you were missing.

    11. Now how do you access the values?

    This is the other part of your question.

    Suppose:

    let event = NetworkEvent::DataReceived {
        source_id: 100,
        data_size: 500,
    };

    And:

    match event {
        NetworkEvent::DataReceived {
            source_id,
            data_size,
        } => {
            println!("Source = {}", source_id);
            println!("Size = {}", data_size);
        }

        NetworkEvent::Connected {
            server_id,
        } => {
            println!("Server = {}", server_id);
        }

        NetworkEvent::Disconnected {
            reason_code,
        } => {
            println!("Reason = {}", reason_code);
        }
    }

    Here:

    NetworkEvent::DataReceived {
        source_id,
        data_size,
    }

    does two things.

    First, it checks:

    Is this value the DataReceived variant?

    Second, if yes:

    Take the values stored in the fields and bind them to variables named source_id and data_size.

    So inside:

    {
        println!("Source = {}", source_id);
        println!("Size = {}", data_size);
    }

    you now have access to the actual values.

    If the event contains:

    source_id: 100
    data_size: 500

    then inside that branch:

    source_id

    has the value:

    100

    and:

    data_size

    has the value:

    500
    12. There are two different things you can write

    This is very important.

    A. Check for a specific value
    NetworkEvent::DataReceived {
        source_id: 100,
        data_size: 500,
    }

    This means:

    I specifically want 100 and 500.

    The numbers are patterns.

    B. Capture the value
    NetworkEvent::DataReceived {
        source_id,
        data_size,
    }

    This means:

    I don't care what the values are. Give me whatever values are actually there, and let me use them as variables.

    So if the event is:

    NetworkEvent::DataReceived {
        source_id: 200,
        data_size: 900,
    }

    then:

    source_id

    becomes 200, and:

    data_size

    becomes 900.

    13. You can mix checking and capturing

    This is where match becomes really powerful.

    Suppose:

    match event {
        NetworkEvent::DataReceived {
            source_id: 100,
            data_size,
        } => {
            println!("Server 100 sent {} bytes", data_size);
        }

        NetworkEvent::DataReceived {
            source_id,
            data_size,
        } => {
            println!("Server {} sent {} bytes", source_id, data_size);
        }

        NetworkEvent::Connected {
            server_id,
        } => {
            println!("Connected to {}", server_id);
        }

        NetworkEvent::Disconnected {
            reason_code,
        } => {
            println!("Disconnected: {}", reason_code);
        }
    }

    Look at the first DataReceived pattern:

    NetworkEvent::DataReceived {
        source_id: 100,
        data_size,
    }

    It means:

    source_id must be 100, but data_size can be anything. Give me its value in a variable called data_size.

    So:

    NetworkEvent::DataReceived {
        source_id: 100,
        data_size: 700,
    }

    matches it.

    Inside the branch:

    data_size

    contains:

    700
    14. You can also ignore a value

    Suppose you only care about the source:

    match event {
        NetworkEvent::DataReceived {
            source_id: 100,
            data_size: _,
        } => {
            println!("Data came from server 100");
        }

        NetworkEvent::DataReceived {
            source_id,
            data_size,
        } => {
            println!("Data came from server {}", source_id);
        }

        NetworkEvent::Connected {
            server_id,
        } => {
            println!("Connected to {}", server_id);
        }

        NetworkEvent::Disconnected {
            reason_code,
        } => {
            println!("Disconnected: {}", reason_code);
        }
    }

    The _ means:

    I don't care what value is stored here.

    So this matches:

    NetworkEvent::DataReceived {
        source_id: 100,
        data_size: 10,
    }

    and:

    NetworkEvent::DataReceived {
        source_id: 100,
        data_size: 5000,
    }

    and:

    NetworkEvent::DataReceived {
        source_id: 100,
        data_size: 999999,
    }

    because the size is ignored.

    15. You can also use ranges

    Suppose you want to distinguish small and large packets.

    You can use a range pattern:

    match event {
        NetworkEvent::DataReceived {
            source_id,
            data_size: 0..=100,
        } => {
            println!("Small packet from {}", source_id);
        }

        NetworkEvent::DataReceived {
            source_id,
            data_size: 101..=1000,
        } => {
            println!("Medium packet from {}", source_id);
        }

        NetworkEvent::DataReceived {
            source_id,
            data_size,
        } => {
            println!("Large packet from {}", source_id);
        }

        NetworkEvent::Connected {
            server_id,
        } => {
            println!("Connected to {}", server_id);
        }

        NetworkEvent::Disconnected {
            reason_code,
        } => {
            println!("Disconnected: {}", reason_code);
        }
    }

    Now match itself is determining the category based on the field's value.

    No if.

    For example:

    data_size = 50

    matches:

    0..=100

    while:

    data_size = 500

    matches:

    101..=1000

    and:

    data_size = 5000

    matches the final branch.

    16. One important limitation: not every Rust value can be written as a simple literal pattern

    This explains why your original String example caused confusion.

    For example, this works:

    Message::Number(0)

    because 0 is a valid pattern.

    But you can't simply write:

    NetworkEvent::DataReceived {
        source: String::from("192.168.32.100"),
        data,
    }

    because:

    String::from("192.168.32.100")

    is an expression that constructs a String; it isn't a valid pattern.

    For strings, you will commonly do something like:

    match event {
        NetworkEvent::DataReceived {
            source,
            data,
        } if source == "192.168.32.100" => {
            println!("Data from our server");
        }

        NetworkEvent::DataReceived {
            source,
            data,
        } => {
            println!("Data from {}", source);
        }

        NetworkEvent::Connected {
            ip_address,
        } => {
            println!("Connected to {}", ip_address);
        }

        NetworkEvent::Disconnected {
            reason,
        } => {
            println!("Disconnected: {}", reason);
        }
    }

    Here the if is called a match guard.

    But don't let this confuse the fundamental idea.

    The fundamental idea is still:

    match value {
        PATTERN => action,
        PATTERN => action,
        PATTERN => action,
    }

    And patterns can inspect the structure and, for types that support it, the values.

    17. The exact mental model I want you to have

    When you see:

    match event {
        NetworkEvent::DataReceived {
            source_id: 100,
            data_size: 500,
        } => {
            // ...
        }
    }

    read it in English:

    "Look at event. Does it contain a NetworkEvent::DataReceived variant? If yes, does its source_id equal 100? Does 
    its data_size equal 500? If all of those match, execute this code."

    And when you see:

    match event {
        NetworkEvent::DataReceived {
            source_id,
            data_size,
        } => {
            println!("{}", source_id);
            println!("{}", data_size);
        }
    }

    read it as:

    "Look at event. If it is a DataReceived variant, extract its source_id and data_size values into variables with those names, 
    then execute this code."

    And when you see:

    Message::Number(0)

    read it as:

    "The enum must be Number, and the value inside it must be exactly 0."

    That is the connection between your Number(0) example and the enum with named fields.

    So yes: match can check the variant, inspect fields inside the variant, compare those fields against patterns, 
    extract their values into variables, ignore fields with _, and choose different code based on those patterns — all 
    without necessarily using if.
     */




    /*
    
    these are two different questions, and I misunderstood the second one. Let's separate them.

    1. Why can't we match a String exactly like an integer?

    First, this works:

    enum Message {
        Number(i32),
        Text(String),
    }

    let message = Message::Number(0);

    match message {
        Message::Number(0) => {
            println!("The number is zero");
        }

        Message::Number(number) => {
            println!("Some other number: {}", number);
        }

        Message::Text(text) => {
            println!("Text: {}", text);
        }
    }

    Here:

    Message::Number(0)

    is a valid pattern.

    The important thing is that 0 is an integer literal. Rust can use integer literals directly in patterns.

    You might therefore expect this to work:

    Message::Text(String::from("hello"))

    But it does not.

    The reason is not that Rust cannot compare Strings.

    Rust absolutely can compare Strings:

    let first = String::from("hello");
    let second = String::from("hello");

    if first == second {
        println!("They are equal");
    }

    This works.

    The issue is that:

    String::from("hello")

    is an expression, meaning it is code that creates a String.

    But the thing after => in a match must be a pattern.

    For example:

    Message::Number(0)

    is a pattern.

    But:

    Message::Text(String::from("hello"))

    is trying to use a function call as a pattern, and Rust does not allow that.

    So the important distinction is:

    0

    is a literal that Rust allows in patterns.

    Whereas:

    String::from("hello")

    is a function call/expression, not a pattern.

    But what about &str?

    There is an important difference between String and &str.

    This can work:

    enum Message {
        Text(&'static str),
        Number(i32),
    }

    let message = Message::Text("hello");

    match message {
        Message::Text("hello") => {
            println!("Exactly hello");
        }

        Message::Text(text) => {
            println!("Some other text: {}", text);
        }

        Message::Number(number) => {
            println!("Number: {}", number);
        }
    }

    Why?

    Because "hello" is a string literal, whose type is &str.

    It is a value that can be used directly in a pattern.

    But:

    String::from("hello")

    creates an owned String, and that function call cannot be placed directly into a pattern.

    So don't take away the idea that "Rust cannot compare String values in match."

    That would be incorrect.

    The real idea is:

    Rust can compare String values, but String::from(...) cannot be used as a pattern.

    If you have an actual String, you can capture it and then compare it with a match guard:

    enum Message {
        Text(String),
        Number(i32),
    }

    let message = Message::Text(String::from("hello"));

    match message {
        Message::Text(text) if text == "hello" => {
            println!("It is hello");
        }

        Message::Text(text) => {
            println!("Some other text: {}", text);
        }

        Message::Number(number) => {
            println!("Number: {}", number);
        }
    }

    Here:

    Message::Text(text)

    is the pattern.

    And:

    if text == "hello"

    is a match guard. It performs the actual comparison.

    2. Your second question: how do I access the enum's stored value outside match?

    This is a very important question.

    Suppose we have:

    enum NetworkEvent {
        Connected {
            ip_address: String,
        },

        DataReceived {
            source: String,
            data: Vec<u8>,
        },

        Disconnected {
            reason: String,
        },
    }

    And we create:

    let event = NetworkEvent::Connected {
        ip_address: String::from("192.168.32.100"),
    };

    You are asking:

    I have event. How can I directly get the ip_address from it in my normal code, without using match?

    For example, you might want something like:

    println!("{}", event.ip_address);

    But you cannot do that.

    Why?

    Because event does not have an ip_address field in the way a struct does.

    This is a very important difference between a struct and an enum.

    Struct

    Suppose:

    struct Connection {
        ip_address: String,
        port: u16,
    }

    Now:

    let connection = Connection {
        ip_address: String::from("192.168.32.100"),
        port: 5060,
    };

    You can directly access:

    println!("{}", connection.ip_address);
    println!("{}", connection.port);

    Because every Connection has:

    ip_address
    port

    Those fields always exist.

    Enum

    Now consider:

    enum NetworkEvent {
        Connected {
            ip_address: String,
        },

        DataReceived {
            source: String,
            data: Vec<u8>,
        },

        Disconnected {
            reason: String,
        },
    }

    Look at the three possibilities.

    A Connected contains:

    ip_address

    A DataReceived contains:

    source
    data

    A Disconnected contains:

    reason

    There is no single field that exists for every NetworkEvent.

    For example, this would make no sense:

    event.ip_address

    because what should happen if event is actually:

    NetworkEvent::Disconnected {
        reason: String::from("timeout"),
    }

    There is no ip_address inside that value.

    That's one of the fundamental reasons enums exist: different variants can contain completely different data.

    So how do I get the value?

    You have several options.

    The most direct way is match.

    match event {
        NetworkEvent::Connected { ip_address } => {
            println!("IP address: {}", ip_address);
        }

        NetworkEvent::DataReceived { source, data } => {
            println!("Source: {}", source);
            println!("Data length: {}", data.len());
        }

        NetworkEvent::Disconnected { reason } => {
            println!("Reason: {}", reason);
        }
    }

    But I understand your question:

    "I don't want to match it. I want to access the value directly in my code."

    Then you need to design the enum or provide a method that does that.

    For example:

    impl NetworkEvent {
        fn ip_address(&self) -> Option<&String> {
            match self {
                NetworkEvent::Connected { ip_address } => Some(ip_address),
                _ => None,
            }
        }
    }

    Now you can write:

    let event = NetworkEvent::Connected {
        ip_address: String::from("192.168.32.100"),
    };

    if let Some(ip) = event.ip_address() {
        println!("IP address: {}", ip);
    }

    The important thing is that the match hasn't disappeared.

    We simply moved the match inside the method.

    The outside code can now conveniently say:

    event.ip_address()
    Why does it return Option?

    Because the IP address doesn't necessarily exist.

    Consider:

    let event = NetworkEvent::Disconnected {
        reason: String::from("timeout"),
    };

    Now:

    event.ip_address()

    cannot return an actual IP address because this particular event doesn't have one.

    So we use:

    Option<&String>

    which means:

    "I might give you a reference to a String, or I might give you nothing."

    Therefore:

    Some(ip_address)

    means:

    Yes, this event has an IP address.

    And:

    None

    means:

    No, this event doesn't have an IP address.

    There is another useful possibility

    You can create methods for each variant's data.

    For example:

    impl NetworkEvent {
        fn source(&self) -> Option<&String> {
            match self {
                NetworkEvent::DataReceived { source, .. } => Some(source),
                _ => None,
            }
        }
    }

    Then:

    let event = NetworkEvent::DataReceived {
        source: String::from("192.168.32.100"),
        data: vec![1, 2, 3],
    };

    if let Some(source_ip) = event.source() {
        println!("Source IP: {}", source_ip);
    }

    Again, the user-facing code doesn't need to know how the enum stores the value.

    The key idea

    You should distinguish these two situations.

    Struct
    struct User {
        name: String,
        age: u32,
    }

    Every User has both fields, so you can directly write:

    println!("{}", employee.name);
    println!("{}", employee.age);
    Enum
    enum Message {
        Text(String),
        Number(i32),
    }

    A Message has either a String or an i32.

    Therefore this is not possible:

    message.value

    because there isn't one universal value field.

    You have to determine which variant you have before you can safely access its contained data.

    That determination can be done directly with match, or encapsulated inside a method such as:

    message.text()

    that internally handles the variants.

    So when you asked:

    "How can I have access to the variables' values of variants in my code, not in match?"

    The answer is:

    You cannot directly access an enum variant's fields using normal field syntax like event.ip_address. You must somehow 
    account for which variant you have. Usually that is done with match, if let, or a method that internally performs the match.
     */



    /*
    Exactly. Your code is wrong for the same reason, even though the value is an integer. The important issue is not String vs i32. The issue is function calls cannot be used as patterns.

    For example:

    enum Message {
        Number(i32),
    }

    fn add_digits(a: i32, b: i32) -> i32 {
        a + b
    }

    let message = Message::Number(0);

    match message {
        Message::Number(add_digits(2, 4)) => {
            println!("The number is zero");
        }
    }

    This is invalid.

    The problem is this part:

    add_digits(2, 4)

    because it is a function call/expression, not a pattern.

    Compare these two

    This is valid:

    Message::Number(6)

    because 6 is a literal pattern.

    This is invalid:

    Message::Number(add_digits(2, 4))

    because add_digits(2, 4) means:

    "Run this function and use the result."

    That's an operation that produces a value.

    A match arm doesn't work that way. The thing before => describes a pattern that the existing value must match.

    So conceptually, Rust is asking:

    Does message match the pattern Message::Number(6)?

    when you write:

    Message::Number(6)

    It is not asking:

    Run add_digits(2, 4), get 6, and then compare.
    But you can calculate the value before the match

    For example:

    let number_to_find = add_digits(2, 4);

    let message = Message::Number(6);

    match message {
        Message::Number(number_to_find) => {
            println!("Found it!");
        }
    }

    But be careful: this does not mean what you might think.

    Here:

    Message::Number(number_to_find)

    number_to_find is not interpreted as "the value 6".

    It is interpreted as a variable pattern:

    "Whatever number is inside Message::Number, bind it to a new variable called number_to_find."

    So this would match any number:

    Message::Number(10)
    Message::Number(500)
    Message::Number(-20)

    That's a very important rule.

    If you want to compare against the result of the function

    You can use a match guard:

    let message = Message::Number(6);

    match message {
        Message::Number(number) if number == add_digits(2, 4) => {
            println!("The number is equal to add_digits(2, 4)");
        }

        Message::Number(number) => {
            println!("Some other number: {}", number);
        }
    }

    Now the sequence is:

    Message::Number(number)

    First matches the variant and extracts the actual number.

    Then:

    if number == add_digits(2, 4)

    performs the calculation and comparison.

    Since:

    add_digits(2, 4)

    returns:

    6

    the first arm matches when the message contains 6.

    So the rule you should remember

    A match pattern can contain things such as:

    Message::Number(0)
    Message::Number(10)
    Message::Number(number)
    Message::Number(0..=10)
    Message::Text("hello")

    But not arbitrary calculations such as:

    Message::Number(add_digits(2, 4))   // ❌
    Message::Number(2 + 4)              // ❌
    Message::Number(some_function())    // ❌

    If you need a calculation, function call, or other runtime condition, you generally put it in a match guard:

    Message::Number(number) if number == add_digits(2, 4)

    This distinction between pattern and expression is one of the most important things to understand about Rust's match.
     */





     /*
     We will focus on just one question:

    Why does Message::Number(number_to_find) mean something completely different from Message::Number(6)?

    1. First, forget the function for a moment

    Start with this:

    enum Message {
        Number(i32),
    }

    This says:

    Message is an enum, and one possible kind of Message is Number, which contains one i32.

    So we can create:

    let message = Message::Number(6);

    The value stored inside message is 6.

    Now we can write:

    match message {
        Message::Number(6) => {
            println!("It is six!");
        }
    }

    Rust looks at the actual value:

    Message::Number(6)

    and looks at the pattern:

    Message::Number(6)

    They match.

    So the code prints:

    It is six!
    2. Now change 6 to a variable name

    Look at this:

    match message {
        Message::Number(number_to_find) => {
            println!("The number is {}", number_to_find);
        }
    }

    At first glance, you might think:

    "I created number_to_find earlier, so Rust should compare the value against it."

    But that's not what Rust thinks.

    Why?

    Because inside a match, Rust treats a plain name like:

    number_to_find

    as a new variable that receives the value.

    It does NOT mean:

    "Compare against the existing variable called number_to_find."

    Instead it means:

    "Whatever value is inside Message::Number, give that value the name number_to_find inside this match arm."

    Let's use a concrete example.

    let message = Message::Number(500);

    match message {
        Message::Number(number_to_find) => {
            println!("The number is {}", number_to_find);
        }
    }

    What happens?

    The enum contains:

    500

    The pattern says:

    Message::Number(number_to_find)

    The number_to_find part says:

    "I don't care what the number is. Give me whatever number is there, and I'll call it number_to_find."

    Therefore:

    number_to_find

    becomes:

    500

    and the output is:

    The number is 500
    3. It would also work with 10

    Change:

    let message = Message::Number(500);

    to:

    let message = Message::Number(10);

    The same match:

    match message {
        Message::Number(number_to_find) => {
            println!("The number is {}", number_to_find);
        }
    }

    works.

    Now number_to_find becomes 10.

    Output:

    The number is 10
    4. And it works with -20
    let message = Message::Number(-20);

    match message {
        Message::Number(number_to_find) => {
            println!("The number is {}", number_to_find);
        }
    }

    Now:

    number_to_find

    contains:

    -20

    So:

    The number is -20
    5. So compare these two very carefully
    Case 1: 6
    Message::Number(6)

    Here 6 is a specific value.

    It means:

    The number must be exactly 6.

    Therefore:

    Message::Number(6)

    matches:

    Message::Number(6)

    but does NOT match:

    Message::Number(10)
    Case 2: number_to_find
    Message::Number(number_to_find)

    Here number_to_find is a variable pattern.

    It means:

    I don't care what the number is. Take whatever number is there and put it into a variable called number_to_find.

    Therefore it matches:

    Message::Number(6)
    Message::Number(10)
    Message::Number(500)
    Message::Number(-20)

    Basically any Message::Number(...).

    6. Now your original example makes sense

    You had:

    let number_to_find = add_digits(2, 4);

    Suppose:

    fn add_digits(a: i32, b: i32) -> i32 {
        a + b
    }

    Then:

    let number_to_find = add_digits(2, 4);

    produces:

    number_to_find = 6

    Now you might think this:

    match message {
        Message::Number(number_to_find) => {
            ...
        }
    }

    means:

    "Check whether the number is 6."

    But it doesn't.

    The number_to_find inside the pattern creates a new binding.

    So you actually have two different things:

    let number_to_find = add_digits(2, 4);

    This creates a variable in your normal code.

    But:

    Message::Number(number_to_find)

    creates a new pattern variable inside the match arm.

    The second one is not saying:

    "Use the first number_to_find."

    It is saying:

    "Take whatever number is inside the message and give it this name."

    7. Let's prove it

    Suppose:

    fn add_digits(a: i32, b: i32) -> i32 {
        a + b
    }

    let number_to_find = add_digits(2, 4);

    let message = Message::Number(999);

    match message {
        Message::Number(number_to_find) => {
            println!("Found: {}", number_to_find);
        }
    }

    You might expect:

    No match

    because:

    number_to_find = 6

    and the message contains:

    999

    But that's NOT what happens.

    It prints:

    Found: 999

    Why?

    Because this:

    Message::Number(number_to_find)

    doesn't use the previous number_to_find.

    It creates a new variable and puts 999 into it.

    8. Then how do we actually compare against our variable?

    This is where the match guard comes in.

    We write:

    fn add_digits(a: i32, b: i32) -> i32 {
        a + b
    }

    let number_to_find = add_digits(2, 4);

    let message = Message::Number(6);

    match message {
        Message::Number(number) if number == number_to_find => {
            println!("The number is what we were looking for!");
        }

        Message::Number(number) => {
            println!("The number is {}", number);
        }
    }

    Now let's execute it mentally.

    First:

    let number_to_find = add_digits(2, 4);

    So:

    number_to_find = 6

    Then:

    let message = Message::Number(6);

    The message contains 6.

    Now Rust sees:

    Message::Number(number) if number == number_to_find

    First:

    Message::Number(number)

    means:

    "Is this a Number variant? If yes, take the number inside it and put it into number."

    So:

    number = 6

    Then Rust checks the if:

    number == number_to_find

    which is:

    6 == 6

    That's true.

    So the first arm runs.

    9. Why can we write Message::Number(6) but not Message::Number(add_digits(2, 4))?

    Now we can understand this properly.

    This:

    Message::Number(6)

    uses a literal.

    Rust knows that this pattern means:

    "The value must be exactly 6."

    But this:

    Message::Number(add_digits(2, 4))

    contains a function call.

    A function call means:

    "Execute this function and calculate a value."

    But a match pattern isn't a place where Rust executes arbitrary calculations.

    So if you want:

    "The number must equal whatever add_digits(2, 4) produces"

    you do:

    Message::Number(number) if number == add_digits(2, 4)

    This separates the two jobs:

    Message::Number(number)

    identifies the variant and extracts its value.

    Then:

    if number == add_digits(2, 4)

    performs your calculation and comparison.

    The one rule I want you to understand

    When you see a name inside a match pattern:

    Message::Number(number)

    don't think:

    "Compare with my existing variable called number."

    Think:

    "Take the value from the message and give it the name number."

    But when you see a literal:

    Message::Number(6)

    think:

    "The value must be exactly 6."

    And when you want to compare against a previously calculated value:

    let target = add_digits(2, 4);

    use:

    Message::Number(number) if number == target

    That is the fundamental distinction.
      */




    /*
    
    Let's break this down from the very beginning, because Text(&'static str) contains several things at once.

    We have:

    enum Message {
        Text(&'static str),
        Number(i32),
    }

    Let's look only at:

    Text(&'static str)

    It means:

    The Text variant contains one value, and that value is a string slice (&str) that has the 'static lifetime.

    There are three parts:

    Text    (&'static str)
    │       │        │
    │       │        └── type: str
    │       └─────────── reference: &
    └─────────────────── enum variant name

    I'll explain each one.

    1. Text is the variant name

    Just like:

    enum Message {
        Number(i32),
    }

    Number is a variant.

    We can create one:

    let message = Message::Number(25);

    Similarly:

    Text

    is another variant.

    So:

    let message = Message::Text("hello");

    means:

    Create a Message whose variant is Text, and put "hello" inside it.

    2. What does ( ... ) mean?

    Look at:

    Number(i32)

    The parentheses mean:

    This variant contains one value.

    For example:

    let message = Message::Number(25);

    The 25 is the value stored inside the Number variant.

    Likewise:

    Text(&'static str)

    means:

    The Text variant contains one value whose type is &'static str.

    For example:

    let message = Message::Text("hello");

    Here:

    "hello"

    is the value stored inside Text.

    3. What is str?

    You already know that Rust has:

    String

    and:

    &str

    String is an owned, growable string.

    For example:

    let text = String::from("hello");

    text has type:

    String

    But:

    let text = "hello";

    has type:

    &str

    A string literal such as:

    "hello"

    is a string slice.

    So normally you can think:

    "hello"  →  &str
    4. What does the first & mean?

    Look at:

    &str

    The & means:

    a reference to a string slice.

    So:

    &str

    is a borrowed string slice.

    For example:

    let text = "hello";

    The type of text is:

    &str

    You can also explicitly write:

    let text: &str = "hello";
    5. Then what is 'static?

    This is the part that usually causes confusion.

    You already learned that 'a in:

    struct Person<'a> {
        name: &'a str,
    }

    is a lifetime parameter.

    'static is also a lifetime.

    But it is a special lifetime provided by Rust.

    When you write:

    &'static str

    you are saying:

    This reference is valid for the entire lifetime of the program.

    So:

    &'static str

    means:

    a reference to a string slice that lives for the entire program.

    6. Why does "hello" work with 'static?

    This is the important connection.

    When you write:

    let message = Message::Text("hello");

    the "hello" is a string literal.

    String literals are stored in the program's binary and are available for the entire lifetime of the running program.

    Therefore their type can be treated as:

    &'static str

    So this works:

    enum Message {
        Text(&'static str),
        Number(i32),
    }

    let message = Message::Text("hello");

    The "hello" can satisfy the &'static str requirement.

    7. Why did I use 'static in that example?

    Actually, for teaching the enum itself, we don't necessarily need 'static.

    We could simply write:

    enum Message<'a> {
        Text(&'a str),
        Number(i32),
    }

    Now the enum has a lifetime parameter.

    Then:

    let message = Message::Text("hello");

    can use an appropriate lifetime for the string slice.

    However, if our enum is specifically intended to hold only string literals, we can write:

    enum Message {
        Text(&'static str),
        Number(i32),
    }

    This says something more restrictive:

    Text can contain only a string slice that is valid for the entire program.

    String literals satisfy this.

    8. Compare these three types

    This is worth understanding carefully:

    String

    means:

    An owned string.

    Then:

    &str

    means:

    A borrowed string slice.

    And:

    &'static str

    means:

    A borrowed string slice whose lifetime is 'static, meaning it is valid for the entire program.

    For example:

    let text1: String = String::from("hello");

    let text2: &str = "hello";

    let text3: &'static str = "hello";

    All three contain the text:

    hello

    But they represent different Rust types and ownership/lifetime relationships.

    9. Why can't I put a normal String into this enum?

    With:

    enum Message {
        Text(&'static str),
        Number(i32),
    }

    this works:

    let message = Message::Text("hello");

    But this does not:

    let my_text = String::from("hello");

    let message = Message::Text(my_text);

    because my_text is:

    String

    while Text expects:

    &'static str

    Those are different types.

    You could borrow the String:

    let my_text = String::from("hello");

    let message = Message::Text(&my_text);

    but that still doesn't work here, because &my_text is a reference to my_text whose lifetime is tied to my_text. 
    It is not necessarily 'static.

    10. If I want the enum to own a normal String

    Then I would simply write:

    enum Message {
        Text(String),
        Number(i32),
    }

    Now:

    let my_text = String::from("hello");

    let message = Message::Text(my_text);

    works.

    Here, the Text variant takes ownership of the String.

    This is often more useful when the text comes from something like:

    a network packet
    a file
    user input
    a database
    an HTTP request

    because those strings are not necessarily string literals.

    So why did the earlier example use &'static str?

    I used it specifically because I was demonstrating that a string literal can be matched directly:

    match message {
        Message::Text("hello") => {
            println!("It is hello");
        }

        Message::Text(text) => {
            println!("Some other text: {}", text);
        }

        Message::Number(number) => {
            println!("Number: {}", number);
        }
    }

    Here:

    "hello"

    can be used directly as a pattern because we're dealing with a string slice (&str), and the string literal has a 
    'static lifetime.

    If we instead use:

    enum Message {
        Text(String),
        Number(i32),
    }

    then the situation is different, and we cannot write:

    Message::Text("hello") // not the same kind of pattern for String

    So the main thing to remember right now is:

    Text(&'static str)

    means:

    "Text is a variant that contains one borrowed string slice, and that string slice is guaranteed to be valid for the 
    entire program."

    And:

    Text(String)

    means:

    "Text is a variant that contains one owned String."

    Those are two different designs.
     */



     /*
     
     Let's slow this down a lot. The difficult part is understanding what &my_text actually is and then understanding 
     what 'static requires.

    We have:

    enum Message {
        Text(&'static str),
        Number(i32),
    }

    The important part is:

    Text(&'static str)

    It says:

    The Text variant must contain a reference to a string slice, and that reference must remain valid for the entire 
    lifetime of the program.

    Now look at this:

    let my_text = String::from("hello");

    let message = Message::Text(&my_text);

    Why doesn't this work?

    1. First understand my_text

    When you write:

    let my_text = String::from("hello");

    you create an owned String.

    So:

    my_text

    has type:

    String

    my_text owns the string "hello".

    Now suppose we write:

    let reference = &my_text;

    What is reference?

    It is not another String.

    Its type is:

    &String

    So:

    my_text

    is the owner, while:

    reference

    is borrowing it.

    2. What does &my_text mean?

    When you write:

    &my_text

    the & means:

    "Give me a reference to the value owned by my_text."

    So:

    let reference = &my_text;

    means approximately:

    reference → borrows my_text

    I know you don't like diagrams, so the important thing in normal words is:

    reference does not own the String. my_text still owns it.

    3. Now think about how long my_text exists

    Consider:

    fn main() {
        let my_text = String::from("hello");

        let reference = &my_text;

        println!("{}", reference);
    }

    This is perfectly valid.

    Why?

    Because my_text is still alive when we use reference.

    The important relationship is:

    my_text

    must remain alive for as long as:

    reference

    needs to be used.

    4. Now look at 'static

    When we write:

    &'static str

    we are asking for something much stronger.

    'static means:

    This reference is valid for the entire duration of the program.

    So imagine this requirement:

    Text(&'static str)

    The Text variant says:

    "Give me a string reference that I can safely keep for the entire program."

    Now look at:

    let my_text = String::from("hello");

    let message = Message::Text(&my_text);

    What are we giving it?

    We're giving it:

    &my_text

    which is a reference to a String owned by my_text.

    But my_text does not live for the entire program.

    5. How long does my_text live?

    Suppose:

    fn main() {
        let my_text = String::from("hello");

        let message = Message::Text(&my_text);

        println!("Hello");
    }

    my_text was created inside main.

    When execution reaches the end of main, my_text is destroyed.

    Therefore the reference:

    &my_text

    cannot remain valid after my_text is destroyed.

    So its lifetime is approximately:

    from the point where my_text is created until my_text is destroyed.

    It is not the lifetime of the entire program.

    6. Compare that with a string literal

    Now look at:

    let message = Message::Text("hello");

    Where does "hello" come from?

    It is a string literal.

    String literals are built into the program itself. They are available for the entire duration of the program.

    Therefore:

    "hello"

    can be treated as:

    &'static str

    That's why this works:

    enum Message {
        Text(&'static str),
        Number(i32),
    }

    let message = Message::Text("hello");

    The string "hello" is available for the whole program.

    7. The key difference

    Compare these:

    String literal
    let message = Message::Text("hello");

    The "hello" is available for the entire program.

    Therefore:

    "hello"

    can satisfy:

    &'static str
    Borrowing a local String
    let my_text = String::from("hello");

    let message = Message::Text(&my_text);

    my_text is a local variable.

    It will eventually be destroyed.

    Therefore:

    &my_text

    does not have a 'static lifetime.

    That's the whole reason.

    8. But there is another thing you should notice

    There is also a type difference.

    We wrote:

    let my_text = String::from("hello");

    so:

    my_text

    has type:

    String

    When we write:

    &my_text

    the type is:

    &String

    But our enum expects:

    &'static str

    Those aren't exactly the same type either.

    Rust can often convert/coerce:

    &String

    to:

    &str

    because a String contains string data.

    So this:

    let my_text = String::from("hello");

    let reference: &str = &my_text;

    is valid.

    But the lifetime is still the problem if the destination requires:

    &'static str

    For example:

    let my_text = String::from("hello");

    let reference: &'static str = &my_text;

    This doesn't work.

    Why?

    Because you're saying:

    "I want a reference to my_text that remains valid for the entire program."

    But my_text is only a local variable.

    9. What if I want to store a reference to my String in the enum?

    Then don't require 'static.

    For example, we can make the enum itself have a lifetime parameter:

    enum Message<'a> {
        Text(&'a str),
        Number(i32),
    }

    Now:

    let my_text = String::from("hello");

    let message = Message::Text(&my_text);

    can work.

    Why?

    Because 'a does not say:

    "This must live forever."

    Instead, it says:

    "The reference stored inside Message must remain valid for whatever lifetime 'a represents."

    So if my_text lives long enough for message, that's enough.

    For example:

    enum Message<'a> {
        Text(&'a str),
        Number(i32),
    }

    fn main() {
        let my_text = String::from("hello");

        let message = Message::Text(&my_text);

        println!("{}", message);
    }

    The reference inside message borrows my_text.

    my_text is still alive while message is being used.

    That's okay.

    10. Why would we ever use 'static then?

    Because sometimes we specifically want data that is guaranteed to exist for the whole program.

    For example:

    enum Message {
        Text(&'static str),
        Number(i32),
    }

    is appropriate if Text is supposed to contain things like:

    Message::Text("Connected")
    Message::Text("Disconnected")
    Message::Text("Timeout")
    Message::Text("Invalid request")

    These are string literals.

    They exist for the entire program.

    The simplest way to remember it

    Look at these two:

    &'static str

    means:

    "Borrowed string slice that is valid for the whole program."

    While:

    &'a str

    means:

    "Borrowed string slice whose lifetime is represented by 'a."

    And:

    String

    means:

    "An owned String."

    So this:

    let my_text = String::from("hello");
    let message = Message::Text(&my_text);

    fails specifically when Message::Text requires:

    &'static str

    because my_text is a local String and does not live for the entire program.

    If we change the enum to:

    enum Message<'a> {
        Text(&'a str),
        Number(i32),
    }

    then borrowing a local String can work because the enum only needs the reference to remain valid for as long as 
    the enum value needs it.
      */


}