fn main() {
    let mut args = std::env::args().skip(1);
    let filename = args.next().expect("Expected filename");
    let buf = std::fs::read(filename).expect("Failed to read file");

    let (_, meta) = unityfs::UnityFsMeta::parse(&buf).unwrap();
    let fs = meta.read_unityfs();
    let asset = fs.main_asset();
    for object in asset.objects() {
        println!("{}", object.type_name(&asset))
    }
}
