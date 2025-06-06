use rusqlite::{Connection, Result};
use clap::Parser;
use clap::ArgAction;
use std::io;
use std::collections::HashSet;

mod filter;

#[derive(Debug)]
pub struct Song{
    id: i32,
    name: String,
    author: String,
    link: Option<String>,
    mood: Option<String>,
    genre: Option<String>,
    lang: Option<String>
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    #[arg(long, short, default_value_t = false, action = ArgAction::SetTrue)]
    //add a song(name and author) to database
    new: bool,
    #[arg(long, short, default_value_t = false, action = ArgAction::SetTrue)]
    //add/replace a link(with chords) on an existing song by id
    link: bool,
    #[arg(long, short, default_value_t = false, action = ArgAction::SetTrue)]
    //add/replace the mood tags on an existing song by id
    mood: bool,
    #[arg(long, short, default_value_t = false, action = ArgAction::SetTrue)]
    //add/replace the genre tags on an existing song by id
    genre: bool,
    #[arg(long, default_value_t = false, action = ArgAction::SetTrue)]
    //add/replace language of the song by id
    lang: bool,
    #[arg(long, short, default_value_t = false, action = ArgAction::SetTrue)]
    //delete song from pool by id
    delete: bool,
    

    #[arg(long, short, default_value_t = false, action = ArgAction::SetTrue)]
    filter: bool
}




//This function handles arguments using 'clap'
//it calls some add/delete functions(presented below) depending on argument provided
//When certain argument encountered, this function calls std::io lib to handle user inputs
//that inputs are then trimmed and assigned to types.
fn args_handler(
    connection: &Connection,
    args: &Args,
    mood_hashset: &HashSet<&str>,
    genre_hashset: &HashSet<&str>,
    lang_hashset: &HashSet<&str>,
    filter_hashset: HashSet<&str>
){
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
            lang: None,
        };
        let _ = add_song(connection, new_song.name, new_song.author, new_song.link, new_song.mood, new_song.genre, new_song.lang);
    }
    if args.link == true{
        let mut id_input = String::new();
        let mut link_input = String::new();
        println!("\nPlease input id of the song");
        io::stdin().read_line(&mut id_input).expect("Failed to process your input");
        println!("Please input name of the song");
        io::stdin().read_line(&mut link_input).expect("Failed to process your input");
        let _ = add_link(connection, id_input.trim().parse::<i32>().unwrap(), link_input.trim().to_string());
    }
    if args.mood == true{
        println!("Here is a list of supported moods:\n{:?}", &mood_hashset);

        let mut id_input = String::new();
        let mut mood_input = String::new();
        println!("\nPlease input id of the song");
        io::stdin().read_line(&mut id_input).expect("Failed to process your input");
        println!("Please input mood of the song");
        io::stdin().read_line(&mut mood_input).expect("Failed to process your input");
        let _ = add_mood(connection, id_input.trim().parse::<i32>().unwrap(), mood_input.trim().to_string(), mood_hashset);
    }
    if args.genre == true{
        let mut id_input = String::new();
        let mut genre_input = String::new();
        println!("\nPlease input id of the song");
        io::stdin().read_line(&mut id_input).expect("Failed to process your input");
        println!("Please input genre of the song");
        io::stdin().read_line(&mut genre_input).expect("Failed to process your input");
        let _ = add_genre(connection, id_input.trim().parse::<i32>().unwrap(), genre_input.trim().to_string(), genre_hashset);
    }
    if args.lang == true{
        let mut id_input = String::new();
        let mut lang_input = String::new();
        println!("\nPlease input id of the song");
        io::stdin().read_line(&mut id_input).expect("Failed to process your input");
        println!("Please input language of the song");
        io::stdin().read_line(&mut lang_input).expect("Failed to process your input");
        let _ = add_lang(connection, id_input.trim().parse::<i32>().unwrap(), lang_input.trim().to_string(), lang_hashset);
    }
    if args.delete == true{
        let mut id_input = String::new();
        println!("\nPlease input id of the song");
        io::stdin().read_line(&mut id_input).expect("Failed to process your input");
        let _ = delete_song(connection, id_input.trim().parse::<i32>().unwrap());
    }

    if args.filter == true{
        let mut filter_input = String::new();
        println!("\nPlease enter preferable types of filter");
        io::stdin().read_line(&mut filter_input).expect("Failed to precess filter input");
        filter::filter(connection, filter_input.trim().to_string(), filter_hashset, mood_hashset, genre_hashset, lang_hashset);
    }
}

fn create_db(connection: &Connection) -> Result<()>{
    let query = "CREATE TABLE IF NOT EXISTS songs (id INTEGER PRIMARY KEY, author TEXT, name TEXT, link TEXT, mood TEXT, genre TEXT, lang TEXT, dub_checker TEXT UNIQUE);";
    (*connection).execute(query, ())?;
    Ok(())
}

//Add song to the db with only 'name' and 'author' columns filled
fn add_song(
    connection: &Connection,
    name: String,
    author: String,
    link: Option<String>,
    mood: Option<String>,
    genre: Option<String>,
    lang: Option<String>,
) -> Result<()>{
    let new_song = Song{
        id: 0,
        name: name,
        author: author,
        link: link,
        mood: mood,
        genre: genre,
        lang: lang
    };
    let query = "INSERT INTO songs(name, author, link, mood, genre, lang, dub_checker) VALUES (?1, ?2, ?3, ?4, ?5, ?6, LOWER(CONCAT(?1, ' ', ?2)));";
    let mut statement = (*connection).prepare(query)?;
    statement.execute((&new_song.name, &new_song.author, &new_song.link, &new_song.mood, &new_song.genre, &new_song.lang))?;
    Ok(())
}

//Adds/replaces link in existing db row
fn add_link(connection: &Connection, id: i32, link: String)-> Result<()>{
    let query = "UPDATE songs SET link = ?2 WHERE id = ?1";
    let mut statement = (*connection).prepare(query)?;
    statement.execute((&id, &link))?;
    Ok(())
}

//Adds/replaces mood tags in existing db row
fn add_mood(connection: &Connection, id: i32, mood: String, mood_hashset: &HashSet<&str>) -> Result<()>{
    let binding = mood.to_lowercase();
    let mood_slice_hash: HashSet<&str> = HashSet::from_iter(binding.split(", ").collect::<Vec<&str>>().into_iter());
    let hash_diff: HashSet<_> = mood_slice_hash.difference(mood_hashset).collect();
    if hash_diff.is_empty(){    
        let query = "UPDATE songs SET mood = ?2 WHERE id = ?1";
        let mut statement = (*connection).prepare(query)?;
        statement.execute((&id, &mood))?;
    } else {
        println!("You cannot use {:?} as mood", hash_diff);
    }
    Ok(())
}

//Adds/replaces genre tags in existing db row
fn add_genre(connection: &Connection, id: i32, genre: String, genre_hashset: &HashSet<&str>) -> Result<()>{
    let binding = genre.to_lowercase();
    let genre_slice_hash: HashSet<&str> = HashSet::from_iter(binding.split(", ").collect::<Vec<&str>>().into_iter());
    let hash_diff: HashSet<_> = genre_slice_hash.difference(genre_hashset).collect();
    if hash_diff.is_empty(){    
        let query = "UPDATE songs SET genre = ?2 WHERE id = ?1";
        let mut statement = (*connection).prepare(query)?;
        statement.execute((&id, &genre))?;
    } else {
        println!("You cannot use {:?} as genre", hash_diff);
    }
    Ok(())
}

//Adds/replaces language tag
fn add_lang(connection: &Connection, id: i32, lang: String, lang_hashset: &HashSet<&str>) -> Result<()>{
    let binding = lang.to_lowercase();
    let lang_slice_hash: HashSet<&str> = HashSet::from_iter(binding.split(", ").collect::<Vec<&str>>().into_iter());
    let hash_diff: HashSet<_> = lang_slice_hash.difference(lang_hashset).collect();
    if hash_diff.is_empty(){    
        let query = "UPDATE songs SET lang = ?2 WHERE id = ?1";
        let mut statement = (*connection).prepare(query)?;
        statement.execute((&id, &lang))?;
    } else {
        println!("You cannot use {:?} as lang", hash_diff);
    }
    Ok(())
}
//Deletes entire row by provided id
fn delete_song(connection: &Connection, id: i32) -> Result<()>{
    let query = "DELETE FROM songs WHERE id = ?1";
    let mut statement = (*connection).prepare(query)?;
    let _ = statement.execute(((&id),));
    Ok(())
}



fn randomise_song(connection: &Connection) -> Result<()>{
    //This query shuffles row order in database and selectts only one value
    //there is no need to use rand crate :D
    let mut statement = (*connection).prepare("SELECT id, name, author, link, mood, genre, lang FROM songs ORDER BY RANDOM() LIMIT 1")?;
    let songs_iter = statement.query_map([], |row| {
        Ok(
            Song{
                id: row.get(0)?,
                name: row.get(1)?,
                author: row.get(2)?,
                link: row.get(3)?,
                mood: row.get(4)?,
                genre: row.get(5)?,
                lang: row.get(6)?,
            }
        )
    })?;
    for song in songs_iter {
        //Variable to avoid moving value
        let rand_song = song.unwrap();
        println!("\n{:?} – {:?}", rand_song.author, rand_song.name);
        println!("ID: {:?}", rand_song.id);
        match rand_song.link {
            Some(_)=> println!("Link: {:?}", rand_song.link.unwrap()),
            None => println!("Link: ")
        }
        match rand_song.mood {
            Some(_)=> println!("Mood: {:?}", rand_song.mood.unwrap()),
            None => println!("Mood: ")
        }
        match rand_song.genre {
            Some(_)=> println!("Genre: {:?}", rand_song.genre.unwrap()),
            None => println!("Genre: ")
        }
        match rand_song.lang {
            Some(_)=> println!("Language: {:?}\n", rand_song.lang.unwrap()),
            None => println!("Language: \n")
        }
    };
    Ok(())
}

fn main() -> Result<()>{
    //Establishing connection with database, creating reference to connection instanse
    //That instance uses deref in functions later to avoid 'moved value' issues

    let db_path = "./songs_db.db3";
    let connection = Connection::open(db_path)?;
    let _ = create_db(&connection);

    let mut mood: HashSet<&str> = HashSet::from(["calm", "energetic", "sad", "positive", "strange", "common", "relaxing", "uplifting", "entertaining", "outrageous", "absurd", "surreal", "desperate", "vibey", "melancholic", "dreary"]);
    let mut genre: HashSet<&str> = HashSet::from(["hip-hop", "reggae", "metal", "soul", "pop", "folk", "jazz", "blues", "rock", "indie", "punk", "country"]);
    let mut filter: HashSet<&str> = HashSet::from(["genre", "mood", "lang"]);
    let mut lang: HashSet<&str> = HashSet::from(["russian", "english"]);

    let args = Args::parse();
    let _ = args_handler(&connection, &args, &mood, &genre, &lang, filter);
    //Checking if no 'manipulation' arguments were provided, then returning random song from db
    if (args.new, args.link, args.mood, args.genre, args.lang, args.delete, args.filter) == (false, false, false, false, false, false, false){
        let _ = randomise_song(&connection);
    }
    Ok(())
}
