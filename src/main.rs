use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Song{
    id: i32,
    name: String,
    author: String,
    link: Option<String>,
    mood: Option<String>,
    genre: Option<String>,
}

fn create_db(connection: &Connection) -> Result<()>{
    let query = "CREATE TABLE IF NOT EXISTS songs (id INTEGER PRIMARY KEY, author TEXT, name TEXT, link TEXT, mood TEXT, genre TEXT, dub_checker TEXT UNIQUE);";
    (*connection).execute(query, ())?;
    Ok(())
}

fn add_song(
    connection: &Connection,
    name: String,
    author: String,
    link: Option<String>,
    mood: Option<String>,
    genre: Option<String>,
) -> Result<()>{
    let new_song = Song{
        id: 0,
        name: name,
        author: author,
        link: link,
        mood: mood,
        genre: genre
    };
    let query = "INSERT INTO songs(name, author, link, mood, genre, dub_checker) VALUES (?1, ?2, ?3, ?4, ?5, LOWER(CONCAT(?1, ' ', ?2)));";
    let mut statement = (*connection).prepare(query)?;
    statement.execute((&new_song.name, &new_song.author, &new_song.link, &new_song.mood, &new_song.genre))?;
    Ok(())
}
fn add_link(connection: &Connection, id: i32, link: String){}
fn add_mood(connection: &Connection, id: i32, mood: String){}
fn add_genre(connection: &Connection, id: i32, genre: String){}

fn main() -> Result<()>{
    let connection = Connection::open_in_memory().unwrap();
    let _ = create_db(&connection);
    let _ = add_song(&connection, "ОБЖ".to_string(), "Птицу ЕМЪ".to_string(), Some("amdm.com".to_string()), Some("Scary, Dark, Calm, Moody".to_string()), Some("Hip-hop, Rap".to_string()));
    let _ = add_song(&connection, "Нумеро Уно".to_string(), "Птицу ЕМЪ".to_string(), Some("amdm.com".to_string()), Some("Scary, Dark, Calm, Moody".to_string()), None);
    let mut statement = connection.prepare("SELECT id, name, author, link, mood, genre from songs")?;
    let songs_iter = statement.query_map([], |row| {
        Ok(
            Song{
                id: row.get(0)?,
                name: row.get(1)?,
                author: row.get(2)?,
                link: row.get(3)?,
                mood: row.get(4)?,
                genre: row.get(5)?,
            }
        )
    })?;
    for song in songs_iter {
        println!("{:?}", song?);
    };
    Ok(())
}
