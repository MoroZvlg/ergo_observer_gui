use std::collections::HashMap;
use egui::Pos2;
use crate::data::types::{ApplicationTreeMessage, ErgoPid, LogLevel, ProcessShortInfo, ProcessState};

pub const ROOT_PID: u64 = 1;

pub struct Node {
    pub pid: u64,
    pub parent_id: u64,
    pub childrens: Vec<u64>,
    pub height: u64,
    // process metadata
    pub name: String,
    pub behavior: String,
    pub messages_in: u64,
    pub messages_out: u64,
    pub messages_mailbox: u64,
    pub running_time: u64,
    pub uptime: i64,
    pub state: ProcessState,
    pub log_level: LogLevel,
}

impl Node {
    fn from_process(process: &ProcessShortInfo, parent_height: u64) -> Self {
        Self {
            pid: process.pid.id,
            parent_id: process.parent.id,
            childrens: Vec::new(),
            height: parent_height + 1,
            name: process.name.clone(),
            behavior: process.behavior.clone(),
            messages_in: process.messages_in,
            messages_out: process.messages_out,
            messages_mailbox: process.messages_mailbox,
            running_time: process.running_time,
            uptime: process.uptime,
            state: process.state,
            log_level: process.log_level,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            pid: ROOT_PID,
            parent_id: ROOT_PID,
            childrens: Vec::new(),
            height: 1,
            name: String::new(),
            behavior: String::new(),
            messages_in: 0,
            messages_out: 0,
            messages_mailbox: 0,
            running_time: 0,
            uptime: 0,
            state: ProcessState::Running,
            log_level: LogLevel::Info,
        }
    }
}

pub const H_GAP: f32 = 50.0;
pub const V_GAP: f32 = 200.0;
pub const NODE_W: f32 = 200.0;
pub const NODE_H: f32 = 110.0;

struct NodeLayout {
    prelim: f32, // preliminary x, relative to parent (предварительный)
    modifier: f32, // shift propagated down to children
    shift: f32, // accumulated shift from apportion (распределение?)
    change: f32, // rate of shift change ??
    thread: Option<u64>, // shortcut pointer (contour threading) ? I saw contour lines on charts in graph? Do we really need this? we don't have shortcuts...
    ancestor: u64, // parent (points to self initially for algorithm correctness)
    number: usize, // index among siblings (0 based)
}


pub struct Tree {
    pub all_processes: HashMap<u64, Node>,
    pub height: u64,
    node_pid_height: HashMap<u64, u64>,
    process_groups_per_level: HashMap<u64, HashMap<u64, Vec<u64>>>, // <level, <parent_id, [children's ids]>>
    lost_nodes_pid_parent_pid: HashMap<u64, ProcessShortInfo>,
}

impl Tree {
    pub fn new() -> Self {
        let root = Node::default();
        let root_group_per_level = HashMap::from([(
          root.pid,
            Vec::<u64>::from([1]),
          )]);
        Self {
            height: 1,
            node_pid_height: HashMap::from([(root.pid, 0)]),
            process_groups_per_level: HashMap::from([(0, root_group_per_level)]),
            lost_nodes_pid_parent_pid: HashMap::new(),
            all_processes: HashMap::from_iter([(root.pid, root)]),
        }
    }

    pub fn add_node(&mut self, process: ProcessShortInfo, traverse: bool) {
        if self.node_pid_height.contains_key(&process.parent.id) {
            let parent_height = self.node_pid_height[&process.parent.id];
            let node = Node::from_process(&process, parent_height);
            let parent_node = self.all_processes.get_mut(&process.parent.id).unwrap();
            if node.height > self.height {
                self.height = node.height;
            }
            if self.process_groups_per_level.contains_key(&node.height) {
                let parent_groups =self.process_groups_per_level.get_mut(&node.height).unwrap();
                if parent_groups.contains_key(&parent_node.pid) {
                    let curr_group = parent_groups.get_mut(&parent_node.pid).unwrap();
                    curr_group.push(node.pid)
                } else {
                    parent_groups.insert(parent_node.pid, Vec::from([node.pid]));
                }
            } else { // new level
                self.process_groups_per_level.insert(node.height, HashMap::from([(parent_node.pid, Vec::from([node.pid]))]));
            }
            parent_node.childrens.push(node.pid);
            self.node_pid_height.insert(node.pid, node.height);
            self.all_processes.insert(node.pid, node);
            if traverse {
                if let Some(lost_process) = self.lost_nodes_pid_parent_pid.remove(&process.pid.id) {
                    self.add_node(lost_process, true);
                }
            }
        } else {
            self.lost_nodes_pid_parent_pid.insert(process.parent.id, process);
        }
    }


    fn left_sibling(&self, pid: u64) -> Option<u64> {
        let node = self.all_processes.get(&pid).unwrap(); // Should we make it safer??
        if node.parent_id == pid { // eq self means ROOT node
            return None
        }
        let parent = self.all_processes.get(&node.parent_id).unwrap(); // Should we make it safer??
        let pos = parent.childrens.iter().position(|&x| x == pid)?; // Should we make it safer??
        if pos == 0 {
            None
        } else {
            Some(parent.childrens[pos - 1])
        }
    }

    fn leftmost_sibling(&self, pid: u64) -> u64 {
        let node = self.all_processes.get(&pid).unwrap(); // Should we make it safer??
        let parent = self.all_processes.get(&node.parent_id).unwrap();
        parent.childrens[0]
    }

    fn next_left(&self, pid: u64, layout: &HashMap<u64, NodeLayout>) -> Option<u64> {
        let node = self.all_processes.get(&pid).unwrap();
        if node.childrens.is_empty() {
            layout[&pid].thread
        } else {
            Some(node.childrens[0])
        }
    }

    fn next_right(&self, pid: u64, layout: &HashMap<u64, NodeLayout>) -> Option<u64> {
        let node = self.all_processes.get(&pid).unwrap();
        if node.childrens.is_empty() {
            layout[&pid].thread
        } else {
            Some(*node.childrens.last().unwrap())
        }
    }

    // determines which subree is responsible for a conflic:
    // if vil's record ancestor is a sibling of v -> use it. ut cause overlap
    // otherwise fallback to default_ancestor
    fn ancestor(&self, vil: u64, v: u64, default_ancestor: u64, layout: &HashMap<u64, NodeLayout>) -> u64 {
        let v_parent = self.all_processes[&v].parent_id;
        let vil_ancestor = layout[&vil].ancestor;
        let vil_ancestor_parent = self.all_processes[&vil_ancestor].parent_id;
        if vil_ancestor_parent == v_parent {
            vil_ancestor  // the ancestor of vil is a sibling of v — it caused the conflict
        } else {
            default_ancestor
        }
    }

    fn apportion(&self, v: u64, default_ancestor: &mut u64, layout: &mut HashMap<u64, NodeLayout>) {
        let Some(w) = self.left_sibling(v) else { return }; // no left siblings = no conflicts. return early

        // 4 contours: inner gap between subtrees and outer bounder of childrens
        let mut vir = v; // inner-right: left contour of v's subtree (faces gap)
        let mut vil = w; // inner-left: right contour of w's subtree (faces gap)
        let mut vor = v; // outer right: right contour of v's subtree (outer boundary)
        let mut vol = self.leftmost_sibling(v); // outer-left: left contour of left most sibling (outer boundary)

        // accumulated modifiers for each contour
        let mut sir = layout[&vir].modifier;
        let mut sil = layout[&vil].modifier;
        let mut sor = layout[&vor].modifier;
        let mut sol = layout[&vol].modifier;

        while let (Some(next_vil), Some(next_vir)) = (self.next_right(vil, layout), self.next_left(vir, layout)) {
            vil = next_vil;
            vir = next_vir;
            // outer contours must advance too — they track overall boundaries
            vol = self.next_left(vol, layout).unwrap();
            vor = self.next_right(vor, layout).unwrap();

            // vor's ancestor = v, so future calls to ancestor() can trace back to v
            layout.get_mut(&vor).unwrap().ancestor = v;

            // "true" x position = prelim + all modifiers accumulated above
            // shift = how much we need to push vir right to not overlap vil
            // IMPORTANT: use sil/sir BEFORE adding current node's modifiers
            // (sil/sir represent accumulated modifiers from the common ancestor
            //  down to the PREVIOUS level, which is what positions this level's nodes)
            let vil_prelim = layout[&vil].prelim;
            let vir_prelim = layout[&vir].prelim;
            let shift = (vil_prelim + sil) - (vir_prelim + sir) + NODE_W + H_GAP;

            if shift > 0.0 {
                let anc = self.ancestor(vil, v, *default_ancestor, layout);
                // spread the shift between that ancestor and v
                self.move_subtree(anc, v, shift, layout);

                sir += shift;
                sor += shift;
            }

            // accumulate modifiers AFTER shift computation, for the next level
            sil += layout[&vil].modifier;
            sir += layout[&vir].modifier;
            sol += layout[&vol].modifier;
            sor += layout[&vor].modifier;
        }

        // --- threading: stitch contours when one runs out before the other ---

        // read next pointers before mutating (borrow checker)

        let next_right_vil = self.next_right(vil, layout);
        let next_right_vor = self.next_right(vor, layout);
        let next_left_vir  = self.next_left(vir, layout);
        let next_left_vol  = self.next_left(vol, layout);

        // case 1: left subtree (vil) is deeper than v's left contour (vir)
        // thread vor (outer-right of v) to continue tracking vil's right contour
        // so future siblings know how far right the deep left subtree reaches
        if next_right_vil.is_some() && next_right_vor.is_none() {
            let vor_node = layout.get_mut(&vor).unwrap();
            vor_node.thread = next_right_vil;
            vor_node.modifier += sil - sor;
        }

        // case 2: v's left contour (vir) is deeper than left subtree (vil)
        // thread vol (outer-left) to continue tracking vir's left contour
        // and promote v as the new default ancestor
        if next_left_vir.is_some() && next_left_vol.is_none() {
            let vol_node = layout.get_mut(&vol).unwrap();
            vol_node.thread   = next_left_vir;
            vol_node.modifier += sir - sol;
            *default_ancestor = v; // v's subtree now defines the left boundary
        }
    }

    fn move_subtree(
        &self,
        wl: u64,   // left boundary subtree (the one that caused the conflict)
        wr: u64,   // right boundary subtree (current node v)
        shift: f32,
        layout: &mut HashMap<u64, NodeLayout>,
    ) {
        // how many sibling subtrees sit between wl and wr
        // this is used to spread the shift proportionally across them
        let subtrees = (layout[&wr].number - layout[&wl].number) as f32;

        // mark wr: when execute_shifts runs right-to-left, it will
        // pick up this shift and apply it to wr and everyone to its left up to wl
        layout.get_mut(&wr).unwrap().shift += shift;

        // change is a rate: it causes the accumulated shift to grow/shrink
        // as execute_shifts walks left across siblings
        // wr gets negative rate (shift stops accumulating after wr)
        layout.get_mut(&wr).unwrap().change -= shift / subtrees;
        // wl gets positive rate (shift starts accumulating at wl)
        layout.get_mut(&wl).unwrap().change += shift / subtrees;

        // move wr itself immediately — prelim and modifier both shift
        // modifier shifts too so wr's entire subtree follows
        layout.get_mut(&wr).unwrap().prelim   += shift;
        layout.get_mut(&wr).unwrap().modifier += shift;
    }

    fn execute_shifts(&self, pid: u64, layout: &mut HashMap<u64, NodeLayout>) {
        let children = self.all_processes[&pid].childrens.clone();

        // accumulated shift and change rate, start at zero
        let mut shift  = 0.0_f32;
        let mut change = 0.0_f32;

        // walk RIGHT TO LEFT — shifts accumulate leftward from wr toward wl
        for &child in children.iter().rev() {
            // pick up any change rate set by move_subtree on this child
            // change increases/decreases how much shift grows per step
            change += layout[&child].change;

            // apply total accumulated shift to this child's position
            // modifier shifts too so the whole subtree below moves with it
            layout.get_mut(&child).unwrap().prelim   += shift;
            layout.get_mut(&child).unwrap().modifier += shift;

            // accumulate shift for the next (leftward) sibling
            shift += layout[&child].shift + change;
        }
    }

    pub fn compute_layout(&self) -> HashMap<u64, Pos2> {
        let mut layout: HashMap<u64, NodeLayout> = self.all_processes.keys().map( |&pid| {
            (pid, NodeLayout{
                prelim: 0.0,
                modifier: 0.0,
                shift: 0.0,
                change: 0.0,
                thread: None,
                ancestor: pid,
                number: 0,
            })
        }).collect(); // TODO: I can create it during add_node?

        // assign siblings number
        for node in self.all_processes.values() {
            for (i, &child_pid) in node.childrens.iter().enumerate() {
                layout.get_mut(&child_pid).unwrap().number = i;
            }
        };

        self.first_walk(ROOT_PID, &mut layout);

        let mut positions = HashMap::<u64, Pos2>::new();
        self.second_walk(ROOT_PID, 0.0, 0, &layout, &mut positions);
        positions
    }

    fn first_walk(&self, pid: u64, layout: &mut HashMap<u64, NodeLayout>) {
        let node = self.all_processes.get(&pid).unwrap();
        let children = node.childrens.clone(); // How much it costs? is it possible to reduce it?

        if children.is_empty() {
            // Final leaf node. place it right after left sibling or at 0 if no siblings
            let left_sibling = self.left_sibling(pid);
            layout.get_mut(&pid).unwrap().prelim = match left_sibling {
                None => 0.0,
                Some(sib) => layout.get(&sib).unwrap().prelim + NODE_W + H_GAP,
            }
        } else {
            // Internal node. recurse first_walk to all childrens
            let mut default_ancestor = children[0]; // will be updated

            for &child in &children {
                // HERE because of the recursion we will go up until the end of the tree
                // rest of the algo will start working from the bottom
                self.first_walk(child, layout);
                self.apportion(child, &mut default_ancestor, layout);
            }

            self.execute_shifts(pid, layout); // APPLY SHIFT LATER. STEP 3

            let first_child = &children[0];
            let last_child = children.last().unwrap();
            let midpoint = (layout[first_child].prelim + layout[last_child].prelim) / 2.0;

            let left_sibling = self.left_sibling(pid);

            match left_sibling {
                None => {
                    // No left sibling. Stay in the middle of the childrens
                    layout.get_mut(&pid).unwrap().prelim = midpoint;
                }
                Some(sib) => {
                    // Has left sibling. Sit to the right of it
                    // but also remember the shift, so we can later shift childrens to the same amount
                    let new_prelim = layout[&sib].prelim + NODE_W + H_GAP;
                    let node_layout = layout.get_mut(&pid).unwrap();
                    node_layout.prelim = new_prelim;
                    node_layout.modifier = new_prelim - midpoint;
                }
            }

        }
    }

    fn second_walk(&self, pid: u64, m: f32, depth: u64, layout: &HashMap<u64, NodeLayout>, positions: &mut HashMap<u64, Pos2>) {
        let x = layout.get(&pid).unwrap().prelim + m;
        let y = depth as f32 * (NODE_H + V_GAP);
        positions.insert(pid, Pos2::new(x, y));

        let modifier = layout[&pid].modifier;
        let children = self.all_processes.get(&pid).unwrap().childrens.clone();
        for &child in &children {
            // pass accumulated modifier down so children get correct absolute x
            self.second_walk(child, m + modifier, depth + 1, layout, positions);
        }
    }
}

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tree (height: {})", self.height)?;
        for (level, parent_group) in &self.process_groups_per_level {
            write!(f, "Level {}: ", level)?;
            for (parent_id, group) in parent_group {
                write!(f, "{} -> {:?}; ", parent_id, group)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "Lost nodes: {:?}", self.lost_nodes_pid_parent_pid.keys())?;
        let root = self.all_processes.get(&ROOT_PID).unwrap();
        self.fmt_node(f, root, 0)
    }
}

impl Tree {
    fn fmt_node(&self, f: &mut std::fmt::Formatter<'_>, node: &Node, depth: usize) -> std::fmt::Result {
        let indent = "  ".repeat(depth);
        writeln!(f, "{}[{}] pid={} name={} behavior={} parent_id={}", indent, depth, node.pid, node.name, node.behavior, node.parent_id)?;
        for child_pid in &node.childrens {
            if let Some(child) = self.all_processes.get(child_pid) {
                self.fmt_node(f, child, depth + 1)?;
            }
        }
        Ok(())
    }
}
