use sqlite::State;

fn main() {
    println!("Database shit");
    test_db()
}

fn test_db() {
    let connection = sqlite::open(":memory:").unwrap();
    let mut query = "
    CREATE TABLE addresses (addr TEXT, seen INTEGER);
    INSERT INTO addresses VALUES ('127.0.0.1', 1);
    INSERT INTO addresses VALUES ('172.27.42.1', 69);
    ";
    connection.execute(query).unwrap();
    query = "UPDATE addresses SET seen = seen + 1";
    connection.execute(query).unwrap();

    // queries are here
    let query = "SELECT * FROM addresses WHERE addr = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, "127.0.0.1")).unwrap();

    while let Ok(State::Row) = statement.next() {
        println!("ip = {}", statement.read::<String, _>("addr").unwrap());
        println!("seen = {}", statement.read::<i64, _>("seen").unwrap());
    }
}
