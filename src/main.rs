use rusqlite::{Connection, Result};
use clap::Parser;
use clap::ArgAction;
use std::io;

#[derive(Debug)]
struct Song{
    id: i32,
    name: String,
    author: String,
    link: Option<String>,
    mood: Option<String>,
    genre: Option<String>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    #[arg(long, short, default_value_t = false, action = ArgAction::SetTrue)]
    new: bool,
    #[arg(long, short, default_value_t = false, action = ArgAction::SetTrue)]
    link: bool,
    #[arg(long, short, default_value_t = false, action = ArgAction::SetTrue)]
    mood: bool,
    #[arg(long, short, default_value_t = false, action = ArgAction::SetTrue)]
    genre: bool,
    #[arg(long, short, default_value_t = false, action = ArgAction::SetTrue)]
    delete: bool
}

fn args_handler(connection: &Connection, args: &Args){
    if args.new == true{
        let mut name_input = String::new();
        let mut author_input = String::new();
        println!("\nPlease input name of the song");
        io::stdin().read_line(&mut name_input).expect("Failed to process your input");
        println!("\nPlease input author of the song");
        io::stdin().read_line(&mut author_input).expect("Failed to precess your input");
        let new_song = Song{
            id: 0,
            name: name_input.trim().to_string(),
            author: author_input.trim().to_string(),
            link: None,
            mood: None,
            genre: None,
        };
        let _ = add_song(connection, new_song.name, new_song.author, new_song.link, new_song.mood, new_song.genre);
    }
    if args.link == true{
        let mut id_input = String::new();
        let mut link_input = String::new();
        println!("\nPlease input id of the song");
        io::stdin().read_line(&mut id_input).expect("Failed to process your input");
        println!("\nPlease input name of the song");
        io::stdin().read_line(&mut link_input).expect("Failed to process your input");
        let _ = add_link(connection, id_input.trim().parse::<i32>().unwrap(), link_input.trim().to_string());
    }
    if args.mood == true{
        let mut id_input = String::new();
        let mut mood_input = String::new();
        println!("\nPlease input id of the song");
        io::stdin().read_line(&mut id_input).expect("Failed to process your input");
        println!("\nPlease input mood of the song");
        io::stdin().read_line(&mut mood_input).expect("Failed to process your input");
        let _ = add_mood(connection, id_input.trim().parse::<i32>().unwrap(), mood_input.trim().to_string());
    }
    if args.genre == true{
        let mut id_input = String::new();
        let mut genre_input = String::new();
        println!("\nPlease input id of the song");
        io::stdin().read_line(&mut id_input).expect("Failed to process your input");
        println!("\nPlease input genre of the song");
        io::stdin().read_line(&mut genre_input).expect("Failed to process your input");
        let _ = add_genre(connection, id_input.trim().parse::<i32>().unwrap(), genre_input.trim().to_string());
    }
    if args.delete == true{
        let mut id_input = String::new();
        println!("\nPlease input id of the song");
        io::stdin().read_line(&mut id_input).expect("Failed to process your input");
        let _ = delete_song(connection, id_input.trim().parse::<i32>().unwrap());
    }
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

fn add_link(connection: &Connection, id: i32, link: String)-> Result<()>{
    let query = "UPDATE songs SET link = ?2 WHERE id = ?1";
    let mut statement = (*connection).prepare(query)?;
    statement.execute((&id, &link))?;
    Ok(())
}
fn add_mood(connection: &Connection, id: i32, mood: String) -> Result<()>{
    let query = "UPDATE songs SET mood = ?2 WHERE id = ?1";
    let mut statement = (*connection).prepare(query)?;
    statement.execute((&id, &mood))?;
    Ok(())
}
fn add_genre(connection: &Connection, id: i32, genre: String) -> Result<()>{
    let query = "UPDATE songs SET genre = ?2 WHERE id = ?1";
    let mut statement = (*connection).prepare(query)?;
    statement.execute((&id, &genre))?;
    Ok(())
}
fn delete_song(connection: &Connection, id: i32) -> Result<()>{
    let query = "DELETE FROM songs WHERE id = ?1";
    let mut statement = (*connection).prepare(query)?;
    let _ = statement.execute(((&id),));
    Ok(())
}

fn randomise_song(connection: &Connection) -> Result<()>{
    let mut statement = (*connection).prepare("SELECT id, name, author, link, mood, genre FROM songs ORDER BY RANDOM() LIMIT 1")?;
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
        let rand_song = song.unwrap();
        println!("\n{:?} – {:?}", rand_song.author, rand_song.name);
        match rand_song.link {
            Some(_)=> println!("Link: {:?}", rand_song.link.unwrap()),
            None => println!("Link: ")
        }
        match rand_song.mood {
            Some(_)=> println!("Mood: {:?}", rand_song.mood.unwrap()),
            None => println!("Genre: ")
        }
        match rand_song.genre {
            Some(_)=> println!("Genre: {:?}\n", rand_song.genre.unwrap()),
            None => println!("Genre: \n")
        }
    };
    Ok(())
}

fn main() -> Result<()>{
    let db_path = "./songs_db.db3";
    let connection = Connection::open(db_path)?;
    let _ = create_db(&connection);

    let args = Args::parse();
    let _ = args_handler(&connection, &args);
    if (args.new, args.link, args.mood, args.genre, args.delete) == (false, false, false, false, false){
        let _ = randomise_song(&connection);
    }
    Ok(())
}
