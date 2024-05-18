use wasm_bindgen::prelude::*;
use std::fmt;


#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
#[derive(Clone, Debug, PartialEq)]
pub struct Tree {
    pub node: String,
    pub children: Vec<Tree>,
}

impl Tree {
    pub fn new(node: String, children: Vec<Tree>) -> Tree {
        Tree {
            node,
            children,
        }
    }

    pub fn get(&self, key: &str) -> Option<&Tree> {
        for child in &self.children {
            if child.node == key {
                return Some(child);
            }
        }
        None
    }

    pub fn iter(&self) -> std::slice::Iter<Tree> {
        self.children.iter()
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DrawTree {
    x: f64,
    y: f64,
    tree: Tree,
    children: Vec<DrawTree>,
    parent: Option<Box<DrawTree>>,
    thread: Option<Box<DrawTree>>,
    offset: f64,
    ancestor: Option<Box<DrawTree>>, // Make ancestor optional
    change: f64,
    mod_name: f64,
    shift: f64,
    lmost_sibling: Option<Box<DrawTree>>,
    number: i32,
}


impl DrawTree {
    pub fn new(tree: Tree, parent: Option<Box<DrawTree>>, depth: f64, number: i32) -> DrawTree {
        let mut dt = DrawTree {
            x: -1.0,
            y: depth,
            tree: tree.clone(),
            children: Vec::new(),
            parent: parent.clone(),
            thread: None,
            offset: 0.0,
            ancestor: None, // Initialize ancestor as None
            change: 0.0,
            shift: 0.0,
            mod_name: 0.0,
            lmost_sibling: None,
            number: number,
        };
        if let Some(ref parent) = dt.parent {
            for (i, c) in parent.tree.children.iter().enumerate() {
                dt.children.push(DrawTree::new(c.clone(), Some(Box::new(dt.clone())), depth + 1.0, (i as i32) + 1));
            }
        }
        dt.ancestor = Some(Box::new(dt.clone())); // Now that dt is fully initialized, we can clone it
        dt
    }

    fn left(&mut self) -> Option<&mut DrawTree> {
        match &mut self.thread {
            Some(thread) => Some(thread),
            None => self.children.first_mut(),
        }
    }

    fn right(&mut self) -> Option<&mut DrawTree> {
        match &mut self.thread {
            Some(thread) => Some(thread),
            None => self.children.last_mut(),
        }
    }


    pub fn left_brother(&self) -> Option<&DrawTree> {
        if let Some(ref parent) = self.parent {
            let mut n = None;
            for node in &parent.children {
                if node as *const _ == self as *const _ {
                    return n;
                } else {
                    n = Some(node);
                }
            }
        }
        None
    }

    pub fn get_lmost_sibling(&mut self) -> Option<&DrawTree> {
        self.lmost_sibling.as_deref()
    }




}


fn buchheim(tree: Tree) -> DrawTree {
    let mut dt = DrawTree::new(tree, None, 0.0, 0);
    firstwalk(&mut dt, 1.0);
    let min = second_walk(&mut dt, 0.0, 0.0, None);
    if min < 0.0 {
        third_walk(&mut dt, -min);
    }
    dt
}


fn firstwalk(v: &mut DrawTree, distance: f64) -> &mut DrawTree {
    if v.children.is_empty() {
        if let Some(lmost_sibling) = v.get_lmost_sibling() {
            v.x = lmost_sibling.x + distance;
        } else {
            v.x = 0.0;
        }
    } else {
        let len = v.children.len();
        let mut default_ancestor_index = 0;
        for i in 1..len {
            let mut child_clone = v.children[i].clone();
            firstwalk(&mut child_clone, distance);
            default_ancestor_index = apportion(&mut child_clone, v, default_ancestor_index, distance);
        }
        println!("finished v = {:?} children", v.tree);

        execute_shifts(v);

        let midpoint = (v.children[0].x + v.children.last().unwrap().x) / 2.0;

        if let Some(w) = v.left_brother() {
            v.x = w.x + distance;
            v.mod_name = v.x - midpoint;
        } else {
            v.x = midpoint;
        }
    }
    v
}

fn apportion<'a>(v: &'a mut DrawTree, parent: &'a mut DrawTree, mut default_ancestor_index: usize, distance: f64) -> usize {
    if let Some(w) = v.left_brother() {
        let (mut vir, mut vor) = (&mut v.clone(), &mut v.clone());
        let mut v_clone = v.clone();
        let (mut vil, mut vol) = (&mut w.clone(), &mut v_clone.get_lmost_sibling().unwrap().clone());
        let (mut sir, mut sor) = (v.offset, v.offset);
        let (mut sil, mut sol) = (vil.offset, vol.offset);

        while vil.right().is_some() && vir.left().is_some() {
            vil = vil.right().unwrap();
            vir = vir.left().unwrap();
            if vol.left().is_some() {
                vol = vol.left().unwrap();
            }
            if vor.right().is_some() {
                vor = vor.right().unwrap();
            }
            vor.ancestor = Some(Box::new(v.clone()));
            let shift = (vil.x + sil) - (vir.x + sir) + distance;
            if shift > 0.0 {
                let mut v_clone1 = v.clone();
                let a = ancestor(&mut *vil, &mut v_clone1, &mut parent.children[default_ancestor_index]);
                let mut v_clone2 = v.clone();
                move_subtree(a, &mut v_clone2, shift);
                sir += shift;
                sor += shift;
            }
            sil += vil.offset;
            sir += vir.offset;
            if vol.left().is_some() {
                sol += vol.offset;
            }
            if vor.right().is_some() {
                sor += vor.offset;
            }
        }

        if vil.right().is_some() && vor.right().is_none() {
            vor.thread = vil.right().map(|node| Box::new(node.clone()));
            vor.offset += sil - sor;
        } else {
            if vir.left().is_some() && vol.left().is_none() {
                vol.thread = vir.left().map(|node| Box::new(node.clone()));
                vol.offset += sir - sol;
            }
            default_ancestor_index = parent.children.iter().position(|x| x == v).unwrap();
        }
    }
    default_ancestor_index
}



fn move_subtree(wl: &mut DrawTree, wr: &mut DrawTree, shift: f64) {
    let subtrees = wr.number - wl.number;
    wr.change -= shift / subtrees as f64;
    wr.shift += shift;
    wl.change += shift / subtrees as f64;
    wr.x += shift;
    wr.offset += shift;
}




fn execute_shifts(v: &mut DrawTree) {
    let mut shift = 0.0 as f64;
    let mut change = 0.0 as f64;
    for w in v.children.iter_mut().rev() {
        w.x += shift;
        w.offset += shift;
        change += w.change;
        shift += w.shift + change;
    }
}

fn ancestor<'a>(vil: &'a mut DrawTree, v: &'a mut DrawTree, default_ancestor: &'a mut DrawTree) -> &'a mut DrawTree {
    if v.parent.as_mut().unwrap().children.iter().any(|child| *child == **vil.ancestor.as_mut().unwrap()) { 
        vil.ancestor.as_mut().unwrap()
    } else {
        default_ancestor
    }
}


fn second_walk(v: &mut DrawTree, m: f64, depth: f64, min: Option<f64>) -> f64 {
    v.x += m;
    v.y = depth;

    let mut min = match min {
        Some(min_val) => if v.x < min_val { v.x } else { min_val },
        None => v.x,
    };

    for w in &mut v.children {
        min = second_walk(w, m + v.offset, depth + 1.0, Some(min));
    }

    min
}


fn third_walk(tree: &mut DrawTree, n: f64) {
    tree.x += n;
    for child in &mut tree.children {
        third_walk(child, n);
    }
}



impl fmt::Display for DrawTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node: {}, Depth: {}, Number: {}", self.tree.node, self.y, self.number)
    }
}