
trait CandidateProvider {
    fn candidates(input: String, position: i64) -> Vec<Candidate>
}

type CandidateData;

trait Candidate {
    display: CandidateDisplay;
    data: CandidateData;
}

type CandidateDisplay String;
