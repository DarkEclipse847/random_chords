use std::collections::HashSet;

//Returns string, consisting of all items in HashSet<&str>
//This will be helpful to provide user an explanation of
//limitations within filters and song posting. 
pub fn params_list(hashset: &HashSet<&str>) -> String {
    let mut result: String = String::new();
    for item in hashset{
        result = result +"\n• " + item;
    }
    result
}
