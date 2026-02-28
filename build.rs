use vergen::Emitter;
use vergen::BuildBuilder;
use vergen::RustcBuilder;
use vergen_gitcl::GitclBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Emitter::default()
        .add_instructions(&GitclBuilder::all_git()?)?
        .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(&RustcBuilder::all_rustc()?)?
        .emit()?;
    Ok(())
}
