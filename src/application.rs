use std::fs;

type Path String;

struct Application {
    search_paths: Vec<Path>,
}

impl Application {
}

fn application_display(path: Path) -> CandidateDisplay {

}

impl CandidateProvider for Application {
    fn candidates(input: String, position: i64) -> Vec<Candidate> {
        search_paths.iter().flat_map(|dir| {
            fs::read_dir(dir).map(|path| {
                Candidate::new(path, application_display(path))
            })
        })
    }
}

impl ActionProvider for Application {
    fn action()
}
