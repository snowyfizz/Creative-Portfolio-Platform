// main.rs - A comprehensive Rust starter script
struct Project<'a> { name: &'a str, version: &'a str }
impl<'a> Project<'a> {
    fn new(name: &'a str) -> Self { Project { name, version: "1.0.0" } }
    fn display_info(&self) { println!("Project: {}, v{}", self.name, self.version); }
}
fn main() {
    let my_project = Project::new("GitHub Auto-Repo Project");
    my_project.displayInfo();
    println!("\nFeatures: Structs, Impl Blocks, Loops");
    for i in 1..=5 { println!("  - Loop {}", i); }
}
