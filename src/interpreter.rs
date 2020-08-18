#[derive(Debug, Clone, Copy, PartialEq)]
enum SupportLevel {
    ///no support
    Unsupported = 0,
    ///run the code in the line, all is contained within and no variable declaration/initialisation happens before
    Line = 1,
    ///run a bloc of code, same limitations as Line
    Bloc = 2,
    ///run a line/bloc of code, but include variable/functions definitions found in the file
    File = 10,
    ///run a line/bloc of code, but include variable/functions found in the project
    Project = 20,
    ///Run a line/bloc of code, but include variable/function from the project and project or system-wide dependencies
    System = 30,
}

trait Interpreter {
    //create
    fn new(data: DataHolder, level: SupportLevel) -> Self;

    fn get_supported_languages() -> Vec<String>;
    fn get_current_level(&self) -> SupportLevel;
    fn set_current_level(&mut self, level: SupportLevel);
    fn get_max_support_level() -> SupportLevel {
        //to overwrite in trait impls
        return SupportLevel::Unsupported;
    }
    fn get_data(&self) -> DataHolder;

    fn fetch_code(&mut self); //mut to allow modification of the current_level
    fn add_boilerplate(&mut self);
    fn build(&mut self); //return path to executable
    fn execute(&mut self) -> Result<String, String>;

    fn run_at_level(&self, level: SupportLevel) -> Result(String, String) {
        self.set_current_level(level);
        self.fetch_code();
        self.add_boilerplate();
        self.build();
        self.execute()
    }
    fn run(&self) -> Result(String, String) {
        self.run_at_level(&self.get_support_level())
    }
}