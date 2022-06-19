use {
    gluesql::{
        prelude::*,
        core::store::{GStore, GStoreMut},
    },
    base64::{encode, decode},
    crate::Behavior::*,
};

const DB_INITIATING_SQL: &'static str =
"
CREATE TABLE IF NOT EXISTS notes (
    id INTEGER not null unique,
    content TEXT not null
);
CREATE INDEX idx_id ON notes (id);
";

fn main() {
    let behavior = parse_args();
    let mut note = Note::new();
    note.initiating();
    
    match behavior {
        ShowAll => note.show_all(),
        Create(note_id) => note.create(note_id),
        Read(note_id) => note.read(note_id),
        Delete(note_id) => note.delete(note_id),
    }
}

type NoteId = i64;

struct Note<T> 
where SledStorage: GStoreMut<T> + GStore<T>
{
    glue: Glue<T, SledStorage>,
}

impl<T> Note<T>
where SledStorage: GStoreMut<T> + GStore<T>
{
    fn new() -> Self {
        let sled_path = "/tmp/gluesql/gluenote";
        let storage = SledStorage::new(sled_path)
            .expect("can not create notes db");
        let glue = Glue::new(storage);
        Self {
            glue: glue,
        }
    }
    
    fn initiating(&mut self) {
        self.glue.execute(DB_INITIATING_SQL).expect("initiating failed");
    }

    fn show_all(&mut self) {
        let output = self.glue.execute("SELECT * FROM notes;").expect("show all failed");
        println!("{:?}", output)
    }

    fn create(&mut self, note_id: NoteId) {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).ok().expect("IO failed");
        let encoded = encode(buf);
        let sql = format!("INSERT INTO notes (id, content) VALUES ({note_id}, \"{encoded}\");");
        let output = self.glue.execute(sql).expect("create failed");
        println!("{:?}", output)
    }
    
    fn delete(&mut self, note_id: NoteId) {
        let query = format!("DELETE FROM notes WHERE id={note_id};");
        let output = self.glue.execute(query).expect("delete failed");
        println!{"{:?}", output}
    }
    
    fn read(&mut self, note_id: NoteId) {
        let query = format!("SELECT content FROM notes WHERE id={note_id};");
        let output = self.glue.execute(query).expect("read failed");
        let content: String = match output {
            Payload::Select { labels: _, rows } => rows[0][0].clone().try_into().ok().expect(""),
            _ => panic!("Unexpected result: {:?}", output),
        };
        let decoded = decode(content).ok().unwrap();
        let content = std::str::from_utf8(&decoded).unwrap();
        println!("{content}")
    }
}

#[derive(Debug)]
enum Behavior {
    ShowAll,
    Create(NoteId),
    Read(NoteId),
    Delete(NoteId),
}

fn parse_args() -> Behavior {
    let mut args = std::env::args();

    if args.len() == 1 {
        return ShowAll;
    }

    let command = args.nth(1).expect("argument error");
    println!("{:?}", command);
    match command.as_str() {
        "new" => {
            let note_id = args.next()
                .expect("note id is missing")
                .parse::<NoteId>().ok()
                .expect("note id should be unsigned int");
            Create(note_id)
        },
        "all" => ShowAll,
        "read" => {
            let note_id = args.next()
                .expect("note id is missing")
                .parse::<NoteId>().ok()
                .expect("note id should be unsigned int");
            Read(note_id)
        },
        "delete" => {
            let note_id = args.next()
                .expect("note id is missing")
                .parse::<NoteId>().ok()
                .expect("note id should be unsigned int");
            Delete(note_id)
        }
        _ => panic!("invalid command"),
    }
}

