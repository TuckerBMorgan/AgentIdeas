use std::collections::HashMap;

pub struct AgentState {

}

type ToolFunction = fn(&mut AgentState) -> ();
type ToolDesecrationFunction = fn(&AgentState) -> bool;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct ToolID {
    id: usize
}

impl ToolID {
    pub fn new(id: usize) -> ToolID {
        ToolID {
            id
        }
    }
}
pub struct Agent {
    tool_functions: HashMap<ToolID, fn(&mut AgentState) -> ()>,
    tool_desecration_functions: HashMap<ToolID, fn(&AgentState) -> bool>,
    tool_id_count: usize
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            tool_functions: HashMap::new(),
            tool_desecration_functions: HashMap::new(),
            tool_id_count: 0
        }
    }

    pub fn add_tool(&mut self, tool_function: ToolFunction, tool_desecration_function: ToolDesecrationFunction) -> ToolID {
        let new_tool_id = self.allocate_tool_id();
        self.tool_functions.insert(new_tool_id, tool_function);
        self.tool_desecration_functions.insert(new_tool_id, tool_desecration_function);
        return new_tool_id;
    }

    fn allocate_tool_id(&mut self) -> ToolID {
        self.tool_id_count += 1;
        return ToolID::new(self.tool_id_count);
    }
}



fn main() {
    let mut agent = Agent::new();
    

}