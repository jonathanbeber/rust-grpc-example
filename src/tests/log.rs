use slog::KV;

struct PrintlnSerializer;
impl slog::Serializer for PrintlnSerializer {
    fn emit_arguments(&mut self, key: slog::Key, val: &std::fmt::Arguments) -> slog::Result {
        print!(", {}={}", key, val);
        Ok(())
    }
}
struct PrintlnDrain;
impl slog::Drain for PrintlnDrain {
    type Ok = ();
    type Err = ();
    fn log(
        &self,
        record: &slog::Record,
        values: &slog::OwnedKVList,
    ) -> std::result::Result<Self::Ok, Self::Err> {
        print!("{}", record.msg());
        record
            .kv()
            .serialize(record, &mut PrintlnSerializer)
            .unwrap();
        values.serialize(record, &mut PrintlnSerializer).unwrap();
        println!("");
        Ok(())
    }
}

pub fn new() -> slog::Logger {
    slog::Logger::root(slog::Fuse(PrintlnDrain), o!("env" => "test"))
}
