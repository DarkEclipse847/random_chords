use rusqlite::{params_from_iter, Connection, Result};
use std::io;
use std::collections::HashSet;

use crate::Song;

//TODO: add lang filter(need to add fixed values as it is done for
//      genres and moods in main.rs)

//This function is too large in my opinion, thinking about a way
//to separate this shit into smaller pieces of code
pub fn filter(
    connection: &Connection,
    params: String,
    params_set: HashSet<&str>,
    mood_set: &HashSet<&str>,
    genre_set: &HashSet<&str>
) -> Result<()>{
    let binding = params.to_lowercase();
    let params_slice_hash: HashSet<&str> = HashSet::from_iter(binding.split(", ").collect::<Vec<&str>>().into_iter());
    let hash_diff: HashSet<_> = params_slice_hash.difference(&params_set).collect();
    if hash_diff.is_empty(){
        
        let mut query: String = "SELECT id, name, author, link, mood, genre, lang FROM songs WHERE".to_string();
        let mut counter: i32 = 0;
        let mut filter_vec: Vec<String> = Vec::new();

        for param in &params_slice_hash{
            match *param{
                "genre" => {
                    let genre_res = filter_genre(genre_set).unwrap().unwrap();
                    let mut genre_sql = String::new();
                    for item in genre_res.iter(){
                        counter = counter + 1;
                        filter_vec.push(item.to_string());
                        genre_sql = genre_sql + format!("genre LIKE '%' || ?{} || '%' AND ", counter).as_str();
                    }
                    genre_sql = genre_sql.trim_end_matches("AND ").to_string();
                    query = query + " ( " + &genre_sql + " ) AND ";
                },
                "mood" => {
                    let mood_res = filter_mood(mood_set).unwrap().unwrap();
                    let mut mood_sql = String::new();
                    for item in mood_res.iter(){
                        counter = counter + 1;
                        filter_vec.push(item.to_string());
                        mood_sql = mood_sql + format!("mood LIKE '%' || ?{} || '%' AND ", counter).as_str();
                    }
                    mood_sql = mood_sql.trim_end_matches("AND ").to_string();
                    query = query + " ( " + &mood_sql + " ) AND ";
                },
                "lang" => println!("tbd"),
                &_ => println!("WTF")
            }
        }
        query = query.trim_end_matches("AND ").to_string() + " ORDER BY RANDOM() LIMIT 1";
        println!("Query: {:?}", query);
        let mut statement = (*connection).prepare(&query)?;
        for test in filter_vec.iter(){println!("{:?}", test);};
        let songs_iter = statement.query_map(rusqlite::params_from_iter(filter_vec), |row| {
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
    } else {
        println!("You cannot use {:?} as a filter param", hash_diff);
    }
    Ok(())
}

//This function will be called only if there is some mood filter
//it will return hashset with user input(if it is suitable)
fn filter_mood<'a>(
    mood_set: &'a HashSet<&'a str>
) -> Result<Option<HashSet<String>>>{
    let mut mood_input = String::new();
    println!("Enter MOOD filter");
    io::stdin().read_line(&mut mood_input).expect("Error happened, while processing mood input");
    let binding = mood_input.trim().to_string().to_lowercase();
    let mood_slice_hash: HashSet<&str> = HashSet::from_iter(binding.split(", ").collect::<Vec<&str>>().into_iter());
    let hash_diff: HashSet<_> = mood_slice_hash.difference(mood_set).collect();
    if hash_diff.is_empty(){
        //There is a simple way using format!() macro but this is not injection-safe
        //so i needed to come up with a better solution that used prepared statement
        //
        //Using format!() on the other hand will be useful to better incapsulate
        //`return` of the func

        //Here i have reassingned mood_slice_hashset to return HashSet<String>
        //Because HashSet<&str> cannot be returned(compiler will wipe out all refs to clear up the memory)
        let mut mood_slice_hash_derefed = HashSet::new();
        for item in mood_slice_hash.iter(){
            mood_slice_hash_derefed.insert(item.to_string());
        }
        Ok(Some(mood_slice_hash_derefed))
    } else {
        //I think it will be a bad choise to throw a custom err,
        //so it will return Ok(None) in case of unsuitable args
        println!("You cannot use {:?} as a mood filter param", hash_diff);
        Ok(None)
    }
}

fn filter_genre<'a>(
    genre_set: &'a HashSet<&'a str>
) -> Result<Option<HashSet<String>>>{
    let mut genre_input = String::new();
    println!("Enter GENRE filter");
    io::stdin().read_line(&mut genre_input).expect("Error happened, while processing genre input");
    let binding = genre_input.trim().to_string().to_lowercase();
    let genre_slice_hash: HashSet<&str> = HashSet::from_iter(binding.split(", ").collect::<Vec<&str>>().into_iter());
    let hash_diff: HashSet<_> = genre_slice_hash.difference(genre_set).collect();
    if hash_diff.is_empty(){
        let mut genre_slice_hash_derefed = HashSet::new();
        for item in genre_slice_hash.iter(){
            genre_slice_hash_derefed.insert(item.to_string());
        }
        Ok(Some(genre_slice_hash_derefed))
    } else {
        println!("You cannot use {:?} as a genre filter param", hash_diff);
        Ok(None)
    }
}


