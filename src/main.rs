use sqlx::query;
use uuid::Uuid;

fn main() {
    //This *should* compile.
    //This does not return an error from the database but refuses to compile, complaining that DATABASE_URL is unset...
    query!("SELECT bar FROM foo");
    //This *should not* compile.
    //...this *does* return an error from the database meaning the connection must exist, but rust analyzer is doing something very strange.
    query!("SELECT baz FROM foo");
    //Perhaps RA is running proc macros twice? Once with the environment variables set in config.toml and once without?

    //Other elements of config.toml are respected...

    //This should only compile if RUSTFLAGS="--cfg uuid_unstable" is set.
    //That it does indicates that rust-analyzer is indeed obeying config.toml to some extent.
    //if it weren't, then rust-analyzer should emit an error while rustc compiles the crate just fine.
    //instead, RA sees no problem with this code.
    let id = Uuid::now_v7();
}
