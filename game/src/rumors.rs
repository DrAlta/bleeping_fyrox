const K1: f64 = 10.0;
const K2: f64 = 10.0;


type AgentID = usize;

struct Person {
    my_actual_opinions: Vec<f64>,
    heard_opinions: HashMap<AgentID, Vec<f64>>,
}

struct World {
    agents : Vec<Option<Person>>
}
impl World {

    fn delta_x_listener_speaker(&mut self, l_index: AgentID, s_index: AgentID, opinion: f64, o_index: AgentID) -> f64 {
        let l = swap_out(self.agents, l_index, None);
        let s = swap_out(self.agents, s_index, None);
        let x_o_s_prime = l.declared_opinions[s_index][o_index];
        let x_l_o = l.actual_opinions[o_index];
        (x_l_o * x_o_s_prime) / K1;
        self.agents[l_index] = l;
        self.agents[s_index] = s;
    }
    }

fn swap_out<K, T>(selfie:&mut Vec<T>, idx: usize, v:T) -> T {
    selfie.push(v);
    selfie.swap_remove(idx)
}
