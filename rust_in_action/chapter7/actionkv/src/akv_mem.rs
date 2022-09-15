use libactionkv::ActionKV; // <1>

//
#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage :
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";
#[cfg(not(taget_os = "windows"))]
const USAGE: &str = "
Usage :
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

fn main() {
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
            let key = maybe_key.expect(&USAGE).as_ref();
            match store.get(key).unwrap() {
                None => eprintln!("{:?} not found", key),
                Some(value) => {
                    let c: &[u8] = &value;
                    println!("{:?}", String::from_utf8_lossy(c))
                }
            }
        }
        "delete" => {
            let key = maybe_key.expect(&USAGE).as_ref();
            store.delete(key).unwrap()
        }
        "insert" => {
            let key = maybe_key.expect(&USAGE).as_ref();
            let value = maybe_value.expect(&USAGE).as_ref();
            println!("value={:?}", value);
            store.insert(key, value).unwrap()
        }
        "update" => {
            let key = maybe_key.expect(&USAGE).as_ref();
            let value = maybe_value.expect(&USAGE).as_ref();
            println!("value={:?}", value);
            store.update(key, value).unwrap()
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
