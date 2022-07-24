// B-Tree Implementation

// IoT device management

// Device management in the IoT space is mostly about storing and retrieving
// specific devices or device twins. These objects typically store addresses,
// configuration values, encryption keys, or other things for small devices so
// nobody has to connect manually. Consequently, keeping an inventory is
// critical!
// For now, the product team settled on a numerical "name", to be available
// faster than the competition, and to keep the requirements short:
// 1. Store IoT device objects (containing the IP address, numerical name,
// and type)
// 2. Retrieve IoT objects by numerical name
// 3. Iterate over IoT objects
// A great use for a tree: the numerical name can be used to create a tree and
// search for it nice and quickly. The basic object for storing this IoT device
// information looks like this:

// Upsides
// This type of tree achieves great performance with the order parameter set
// accordingly:
// 1. Less complex to implement than other self-balancing trees
// 2. Widely used in database technology
// 3. Predictable performance thanks to self-balancing
// 4. Range queries are possible
// 5. Variants that minimize disk access (B+ Tree)

// Downsides
// Absolute performance depends significantly on the tree's order; other than
// that, this tree does not have many downsides.

use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String,
}

type Tree = Box<Node>;
type KeyType = u64;
type Data = (Option<IoTDevice, Option<Tree>);

#[derive(Clone, PartialEq)]
enum NodeType {
    Leaf,
    Regular,
}

#[derive(Clone, PartialEq)]
enum Direction {
    Left,
    Right(usize),
}

// this node type uses a synchronized index to find the
// children associated with a specified key-value pair. These pairs are also
// created ad hoc by evaluating the numerical_id property of the contained
// device, thereby also simplifying the code and eventual updates to the keys.
// Something that is missing from the node is a parent pointer, which made the
// entire red-black tree code significantly more complex.
#[derive(Clone)]
struct Node {
    devices: Vec<Option<IoTDevice>>,
    children: Vec<Option<Tree>>,
    left_child: Option<Tree>,
    pub node_type: NodeType,
}

// The tree itself is stored as an Option on a boxed node (aliased as Tree), along
// with the order and length properties:
pub struct DeviceDatabase {
    root: Option<Tree>,
    order: usize,
    pub length: u64,
}

// Finally, to check the validity of the tree, here's a validate method that
// recursively finds the minimum and maximum leaf height and checks
// whether the number of children is within bounds (as mentioned in the rules
// indicated earlier):
impl DeviceDatabase {
    pub fn is_a_valid_btree(&self) -> bool {
        if let Some(tree) = self.root.as_ref() {
            let total = self.validate(tree, 0);
            total.0 && total.1 == total.2
        } else {
            false // there is no tree
        }
    }

    fn validate(&self, node: &Tree, level: usize) -> (bool, usize, usize) {
        match node.node_type {
            NodeType::Leaf => (node.len() <= self.order, level, level),
            NodeType::Regular => {
                // Root node only requires two children,
                // every other node at least half the order
                let min_children = if level > 0 {
                    self.order / 2usize 
                } else {
                    2
                };

                let key_rules = node.len() <= self.order && node.len() >= min_children;

                let mut total = (key_rules, usize::max_value(), level);

                for n in node.children.iter().chain(vec![&node.left_child]) {
                    if let Some(ref tree) = n {
                        let stats = self.validate(tree, level + 1);
                        total = (
                            total.0 && stats.0,
                            cmp::min(stats.1, total.1),
                            cmp::max(stats.2, total.2),
                        );
                    }
                }
                total
            }
        }
    }

    // Adding Stuff
    // B-Trees add new entries to their leaves, which then bubble up as nodes grow too large. In
    // order to efficiently find a spot, this is done recursively, removing and replacing ownership
    // as needed. Here is the add() function, which takes care of retrieving ownership of the root
    // node and calling the recursive call with an existing or new node:
    pub fn add(&mut self, device: IoTDevice) {
        let node = if self.root.is_some() {
            mem::replace(&mut self.root, None).unwrap()
        } else {
            Node::new_leaf()
        };

        let (root, _) = self.add_r(node, device, true);
        self.root = Some(root);
    }

    // Except in the case of the root node, the add_r() function (the recursive call) returns two
    // pieces of information: the key it descended into and—in case of a "promotion"—the
    // device and child that are to be added to whichever node it returns to. In principle, this
    // function works as follows:
    // 1. Recursively find the appropriate leaf and perform a sorted insert.
    // 2. Increment the length if it's not a duplicate.
    // 3. If the node now has more keys than are allowed: split.
    // 4. Return the original node and the key with its new value to the caller.
    // 5. Place the new node where it came from.
    // 6. Add the promoted key.
    // 7. Repeat from step 3 until at the root level:
    fn add_r(&mut self, node: Tree, device: IoTDevice, is_root: bool) -> (Tree, Option<Data>) {
        let mut node = node;
        let id = device.numerical_id;

        match  node.node_type {
            NodeType::Leaf => {                                          // 1
                if node.add_key(id, (Some(device), None)) {
                    self.length += 1;                                    // 2
                }
            }
            NodeType::Regular => {
                let (key, (dev, tree)) = node.remove_key(id).unwrap();
                let new = self.add_r(tree.unwrap(), device, false);
                if dev.is_none() {                                       // 5
                    node.add_left_child(Some(new.0));
                } else {
                    node.add_key(key, (dev, Some(new.0)));
                }
                                                                         // 6
                if let Some(split_result) = new.1 {
                    let new_id = &split_result.0.clone().unwrap();
                    node.add_key(new_id.numerical_id, split_result);
                }
            }
        }

        if node.len() > self.order {                                      // 3
            let (new_parent, sibling) = node.split();

            // Check if the root node is "full" and add a new level
            if is_root {
                let mut parent = Node::new_regular();
                // Add the former root to the left
                parent.add_left_child(Some(node));
                // Add the new right part as well
                parent.add_key(new_parent.numerical_id, (Some(new_parent), Some(sibling)));
                (parent, None)
            } else {                                                       // 4
                (node, Some((Some(new_parent), Some(sibling))))
            }
        } else {
            (node, None)
        }
    }

    // Searching for stuff
    // A B-Tree's search works just the way binary tree searches do: recursively
    // checking each node for the path to follow. In B-Trees, this becomes very
    // convenient since it can be done in a loop, in this case, by the get_device()
    // function:
    pub fn get_device(&self, key: KeyType) -> Option<&IoTDevice> {
        let mut result = None;
        for d in self.devices.iter() {
            if let Some(device) = d {
                if device.numerical_id == key {
                    result = Some(device);
                    break;
                }
            }
        }
        result
    }
}


impl Node {
    // In this implementation, a lot of the heavy lifting is done by the node's implementation of
    // several functions, including split(). While this is complex, it encapsulates the inner
    // workings of the tree—something that should not be exposed too much so as to facilitate
    // change:
    pub fn split(&mut self) -> (IoTDevice, Tree) {
        let mut sibling = Node::new(self.node_type.clone());

        let no_of_devices = self.devices.len();
        let split_at = no_of_devices / 2usize;
        
        let dev = self.devices.remove(split_at);
        let node = self.children.remove(split_at);

        for _ in split_at..self.devices.len() {
            let device = self.devices.pop().unwrap();
            let child = self.children.pop().unwrap();
            sibling.add_key(device.as_ref().unwrap().numerical_id,(device, child)); 
        }

        sibling.add_left_child(node);
        (dev.unwrap(), sibling)
    }

    // This function is implemented at the node structure and does a regular linear
    // search for the key itself. If it is unable to find that key, the find_r() function
    // has to decide whether to continue, which it does by evaluating the node
    // type. Since leaf nodes don't have any children, not finding the desired key
    // will end the search, returning None. Regular nodes allow the search to
    // continue on a deeper level of the tree:
    pub fn find(&self, id: KeyType) -> Option<IoTDevice> {
        match self.root.as_ref() {
            Some(tree) => self.find_r(tree, id),
            _ => None,
        }
    }

    fn find_r(&self, node: &Tree, id: KeyType) -> Option<IoTDevice> {
        match node.get_device(id) {
            Some(device) => Some(device.clone()),
            None if node.node_type != NodeType::Leaf => {
                if let Some(tree) = node.get_child(id) {
                    self.find_r(tree, id)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // Walking the tree
    // Walking can be done with different strategies, even if 
    // there are many more branches to walk. The
    // following code shows an in-order tree walking algorithm, where the
    // callback is executed between the left child and before descending into the
    // child that is currently looked at:
    pub fn walk(&self, callback: impl Fn(&IoTDevice) -> ()) {
        if let Some(ref root) = self.root {
            self.walk_in_order(root, &callback);
        }
    }

    pub fn walk_in_order(&self, node: &Tree, callback: impl Fn(&IoTDevice) -> ()) {
        if let Some(ref left) = node.left_child {
            self.walk_in_order(left, callback);
        }

        for i in 0..node.devices.len() {
            if let Some(ref k) = node.devices[i] {
                callback(k);
            }

            if let Some(ref c) = node.children[i] {
                self.walk_in_order(&c, callback);
            }
        }
    }
}

fn main() {

}