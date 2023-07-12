use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Debug, Clone)]
pub struct CTM {
    pub title_id: String,
    pub git_hash: String,
    pub init_time: DateTime<Utc>,
    pub movie_id: u64,
    pub author: String,
    pub rerecords: u32,
    pub frames: u64,
    pub inputs: u64,
    pub u64_time: u64,
}

impl Default for CTM {
    fn default() -> Self {
        CTM {
            title_id: "".to_string(),
            git_hash: "".to_string(),
            init_time: DateTime::from_utc(NaiveDateTime::from_timestamp_opt(0, 0).unwrap(), Utc),
            movie_id: 0,
            author: "".to_string(),
            rerecords: 0,
            frames: 0,
            inputs: 0,
            u64_time: 0,
        }
    }
}

pub fn read_value(content: &Vec<u8>, position: &mut usize, count: usize) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    let end_position = *position + count;
    while *position < end_position {
        result.push(content[*position]);
        *position += 1;
    }
    result
}

pub fn from_vec(content: Vec<u8>) -> Result<CTM, String> {
    let mut position: usize = 0;
    let magic = read_value(&content, &mut position, 4);
    if magic != vec![67, 84, 77, 27] {
        return Err("Invalid magic".to_string());
    }
    let mut ctm: CTM = CTM::default();

    let mut title_id: [u8; 8] = read_value(&content, &mut position, 8)[0..=7].try_into().unwrap();
    title_id.reverse();

    let git_hash: [u8; 20] = read_value(&content, &mut position, 20)[0..=19].try_into().unwrap();

    let init_time = i64::from_le_bytes(
                            read_value(&content, &mut position, 8)[0..=7].try_into().unwrap()
    );
    
    let init_time = NaiveDateTime::from_timestamp_opt(init_time, 0).unwrap();
    
    let movie_id: [u8; 8] = read_value(&content, &mut position, 8)[0..=7].try_into().unwrap();
    let movie_id: u64 = u64::from_le_bytes(movie_id);
    
    let author: String = String::from_utf8(read_value(&content, &mut position, 32)).unwrap();

    let rerecords: [u8; 4] = read_value(&content, &mut position, 4)[0..=3].try_into().unwrap();
    let rerecords: u32 = u32::from_le_bytes(rerecords);

    ctm.init_time = DateTime::from_utc(init_time, Utc);
    ctm.title_id = hex::encode(title_id);
    ctm.git_hash = hex::encode(git_hash);
    ctm.author = author;
    ctm.movie_id = movie_id;
    ctm.rerecords = rerecords;


    return Ok(ctm);
}