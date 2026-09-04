struct RaftNode {
    id: u32,
    current_term: u64,
    state: RaftState,
}

enum  RaftState {
    Follower,
    Leader,
    Candidate,
}

fn main () {
    let RaftNode {
        id: 1,
        current_term: 1,
        state: RaftState::Follower,
    };

}