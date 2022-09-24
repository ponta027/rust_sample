use libactionkv::ActionKV; // <1>
use std::collections::HashMap;

//
#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage :
    akv_disk.exe FILE get KEY
    akv_disk.exe FILE delete KEY
    akv_disk.exe FILE insert KEY VALUE
    akv_disk.exe FILE update KEY VALUE
";
#[cfg(not(taget_os = "windows"))]
const USAGE: &str = "
Usage :
    akv_disk FILE get KEY
    akv_disk FILE delete KEY
    akv_disk FILE insert KEY VALUE
    akv_disk FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;

fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = std::collections::HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
    //
}

fn main() {
    const INDEX_KEY: &ByteStr = b"+index";

    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let maybe_key = args.get(3);
    //let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action {
        "get" => {
            let index_as_bytes = store.get(&INDEX_KEY).unwrap().unwrap();

            let index_decoded = bincode::deserialize(&index_as_bytes);

            let index: HashMap<ByteString, u64> = index_decoded.unwrap();
            let key: &[u8] = maybe_key.expect(&USAGE).as_ref();
            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(&i) => {
                    let kv = store.get_at(i).unwrap();
                    //let c: &[u8] = &value;
                    println!("{:?}", String::from_utf8_lossy(&kv.value[..]))
                }
            }
        }
        "delete" => {
            let key = maybe_key.expect(&USAGE).as_ref();
            println!("delete key ={:?}", key);
            store.delete(key).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        "insert" => {
            let key = maybe_key.expect(&USAGE).as_ref();
            let value = maybe_value.expect(&USAGE).as_ref();
            println!("value={:?}", value);
            store.insert(key, value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        "update" => {
            let key = maybe_key.expect(&USAGE).as_ref();
            let value = maybe_value.expect(&USAGE).as_ref();
            println!("value={:?}", value);
            store.update(key, value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        // additional function
        "list" => {
            println!("List");
            for key in store.list().unwrap() {
                println!("Key:{:?}", String::from_utf8_lossy(key));
            }
        }

        _ => {
            eprintln!("{}", &USAGE)
        } //
    }

    //
}
