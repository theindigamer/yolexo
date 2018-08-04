use std::fmt::Display;
use std::io::Write;
use std::vec::Vec;

struct Tag(());

struct Priority(u32);

struct State(());

type Symbol = u8;
type Set<T> = Vec<T>;

struct StateIdx(usize);
struct AlphIdx(usize);

enum Transition {
    AlphTrans(Set<(StateIdx, AlphIdx, (), StateIdx)>),
    EpsTrans(
        Set<
            (StateIdx,
             Option<PriorityIdx>,
             Option<(PriorityIdx, Bool)>,
             StateIdx),
        >
    ),
}

struct TNFA<P> {
    policy: P,
    alphabet: Set<Symbol>,
    tags: Set<Tag>,
    priorities: Set<Priority>,
    states: Set<State>,
    init_state: StateIdx,
    final_states: Set<StateIdx>,
    transition: Transition,
}

// See Observation 1 in the paper.
enum TNFAState {

}
