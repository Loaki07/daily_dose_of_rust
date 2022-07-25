# Overview of Generic Tree/N-ary/M-arr trees

# Types of m-arr tree:
1. A rooted tree is called an m-arr tree if every internal
vertex has no more than m children.
2. The tree is called a full m-arr tree if every internal
vertex has exactly m children.
3. A complete m-arr tree is an m-arr tree in which every
internal vertex has exactly m children and all leaves have 
the same depth.
4. A rooted m-arr tree of height h is balanced if all leaves
are at levels h or h-1.

# Properties of m-arr tree:
1. Counting vertices in full m-arr tree: 
   1. a full m-arr tree with i internal vertices contains
   n = mi + 1 vertices.
   2. a full m-arr tree with n vertices has i = (n-1) / m 
   internal vertices and l = (m-1)n + 1 / m leaves.
   3. a full m-arr tree with i internal vertices has n = mi + 1
   vertices and l = (m-1)i + 1 leaves.
   4. a full m-arr tree with l leaves has n = (ml - 1) / (m - 1) vertices
   and i = (l -1) / (m - 1) internal vertices.
   5. a full m-arr balanced tree of height h has more than m ^ h - 1 leaves.

2. Bound on the number of Leaves in an m-arr tree
   1. There are at the most m ^ h leaves in an m-arr tree of height h.
   2. if an m-arr tree of height h has l leaves, then h >= [log base m l].
   if the m-arr tree is full and balanced, then h = [log base m l].
   ([x] denotes smallest integer greater than or equal to x.)


# Traversal of M-arr Tree

1. Preorder Traversal: Visit the root node, then traverse the left subtree and finally traverse the right subtree.
2. Inorder Traversal: Traverse the left subtree, then visit the root node and finally traverse the right subtree.
3. Postorder Traversal: Traverse the left subtree, then traverse the right subtree and finally visit the root node.
4. Level-order Traversal: Traverse the tree level by level.