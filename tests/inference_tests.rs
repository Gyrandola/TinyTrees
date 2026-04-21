#![cfg(test)]

use tinytrees::inference::tree::InferenceTree;
use tinytrees::node::Node;
use tinytrees::inference::error::InferenceError;

fn create_simple_tree() -> Vec<Node<i32>> {
    vec![
        Node::new_split(0, 5.0, 1, 2),  // root: feature 0, threshold 5.0
        Node::new_leaf(0),              // left: prediction 0
        Node::new_split(1, 3.0, 3, 4),  // right: feature 1, threshold 3.0
        Node::new_leaf(1),              // left of split 2: prediction 1
        Node::new_leaf(2),              // right of split 2: prediction 2
    ]
}

#[test]
fn test_inference_tree_creation() {
    let nodes = create_simple_tree();
    let tree = InferenceTree::new(&nodes);

    assert_eq!(tree.len(), 5);
    assert!(!tree.is_empty());
    assert!(tree.root().is_some());
}

#[test]
fn test_empty_tree() {
    let nodes: Vec<Node<i32>> = vec![];
    let tree = InferenceTree::new(&nodes);

    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
    assert!(tree.root().is_none());
}

#[test]
fn test_single_leaf_tree() {
    let nodes = vec![Node::new_leaf(42)];
    let tree = InferenceTree::new(&nodes);

    assert!(!tree.is_empty());
    assert_eq!(tree.len(), 1);

    let root = tree.root().unwrap();
    assert!(root.is_leaf());
    assert_eq!(root.get_prediction(), Some(&42));
}

#[test]
fn test_get_node_at_index() {
    let nodes = create_simple_tree();
    let tree = InferenceTree::new(&nodes);

    // Root node (split)
    let root = tree.get_node(0).unwrap();
    assert!(!root.is_leaf());

    // Left child (leaf with prediction 0)
    let left_child = tree.get_node(1).unwrap();
    assert!(left_child.is_leaf());
    assert_eq!(left_child.get_prediction(), Some(&0));

    // Out of bounds
    assert!(tree.get_node(100).is_none());
}

#[test]
fn test_predict_empty_tree() {
    let nodes: Vec<Node<i32>> = vec![];
    let tree = InferenceTree::new(&nodes);
    let features = vec![1.0, 2.0];

    let result = tree.predict(&features);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_predict_single_leaf() {
    let nodes = vec![Node::new_leaf(99)];
    let tree = InferenceTree::new(&nodes);
    let features = vec![1.0, 2.0];

    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&99));
}

#[test]
fn test_predict_left_branch() {
    let nodes = create_simple_tree();
    let tree = InferenceTree::new(&nodes);

    // Feature 0 = 3.0, which is <= 5.0, so go left to leaf 0
    let features = vec![3.0, 10.0];
    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&0));
}

#[test]
fn test_predict_right_then_left() {
    let nodes = create_simple_tree();
    let tree = InferenceTree::new(&nodes);

    // Feature 0 = 7.0, which is > 5.0, so go right
    // Feature 1 = 1.0, which is <= 3.0, so go left to leaf 1
    let features = vec![7.0, 1.0];
    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&1));
}

#[test]
fn test_predict_right_then_right() {
    let nodes = create_simple_tree();
    let tree = InferenceTree::new(&nodes);

    // Feature 0 = 7.0, which is > 5.0, so go right
    // Feature 1 = 5.0, which is > 3.0, so go right to leaf 2
    let features = vec![7.0, 5.0];
    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&2));
}

#[test]
fn test_predict_boundary_threshold_equal_goes_left() {
    let nodes = create_simple_tree();
    let tree = InferenceTree::new(&nodes);

    // Feature 0 = 5.0, which is == 5.0, so go left (using <=)
    let features = vec![5.0, 10.0];
    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&0));
}

#[test]
fn test_predict_feature_index_out_of_bounds() {
    // Create a tree that uses feature index 5
    let nodes = vec![
        Node::new_split(5, 2.0, 1, 2),
        Node::new_leaf(0),
        Node::new_leaf(1),
    ];
    let tree = InferenceTree::new(&nodes);

    // Only provide 4 features, but tree needs feature index 5
    let features = vec![1.0, 2.0, 3.0, 4.0];
    let result = tree.predict(&features);

    // Should fail when trying to access feature 5
    assert!(matches!(result, Err(InferenceError::FeatureIndexOutOfBounds)));
}

#[test]
fn test_predict_invalid_node_index() {
    // Create a malformed tree: split points to invalid index
    let nodes = vec![
        Node::new_split(0, 5.0, 1, 999), // right child at invalid index 999
        Node::new_leaf(0),
    ];
    let tree = InferenceTree::new(&nodes);

    // Try to go right (which would access index 999)
    let features = vec![10.0];
    let result = tree.predict(&features);

    assert!(matches!(result, Err(InferenceError::InvalidNodeIndex)));
}

#[test]
fn test_predict_deep_tree() {
    // Create a deeper tree: all left splits
    let nodes = vec![
        Node::new_split(0, 5.0, 1, 2),   // 0: split on feature 0
        Node::new_split(1, 10.0, 3, 4),  // 1: split on feature 1
        Node::new_leaf(99),              // 2: right branch (leaf)
        Node::new_split(2, 2.0, 5, 6),   // 3: split on feature 2
        Node::new_leaf(88),              // 4: right branch (leaf)
        Node::new_leaf(77),              // 5: left branch (leaf)
        Node::new_leaf(66),              // 6: right branch (leaf)
    ];
    let tree = InferenceTree::new(&nodes);

    // All features <= threshold: 3.0 <= 5.0, 8.0 <= 10.0, 1.0 <= 2.0
    let features = vec![3.0, 8.0, 1.0];
    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&77));
}

#[test]
fn test_predict_with_negative_features() {
    let nodes = vec![
        Node::new_split(0, 0.0, 1, 2),
        Node::new_leaf(0),
        Node::new_leaf(1),
    ];
    let tree = InferenceTree::new(&nodes);

    let features = vec![-5.5];
    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&0));
}

#[test]
fn test_predict_with_float_features() {
    let nodes = vec![
        Node::new_split(0, 3.14159, 1, 2),
        Node::new_leaf(0),
        Node::new_leaf(1),
    ];
    let tree = InferenceTree::new(&nodes);

    let features = vec![3.14158]; // Just below threshold
    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&0));

    let features = vec![3.14160]; // Just above threshold
    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&1));
}

#[test]
fn test_predict_string_predictions() {
    let nodes = vec![
        Node::new_split(0, 5.0, 1, 2),
        Node::new_leaf("cat"),
        Node::new_leaf("dog"),
    ];
    let tree = InferenceTree::new(&nodes);

    let features = vec![3.0];
    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&"cat"));

    let features = vec![10.0];
    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&"dog"));
}

#[test]
fn test_multiple_predictions_same_tree() {
    let nodes = create_simple_tree();
    let tree = InferenceTree::new(&nodes);

    // Make multiple predictions with same tree
    let test_cases = vec![
        (vec![3.0, 1.0], 0),
        (vec![7.0, 1.0], 1),
        (vec![7.0, 5.0], 2),
        (vec![5.0, 3.0], 0),
    ];

    for (features, expected) in test_cases {
        let result = tree.predict(&features).unwrap();
        assert_eq!(result, Some(&expected));
    }
}

#[test]
fn test_tree_with_many_features() {
    // Tree that uses many features
    let nodes = vec![
        Node::new_split(0, 1.0, 1, 2),
        Node::new_split(5, 2.0, 3, 4),
        Node::new_leaf(0),
        Node::new_leaf(1),
        Node::new_leaf(2),
    ];
    let tree = InferenceTree::new(&nodes);

    // Need at least 6 features (indices 0-5)
    let features = vec![0.5, 99.0, 88.0, 77.0, 66.0, 1.5];
    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&1));
}

#[test]
fn test_predict_regression_float_predictions() {
    // Tree for regression with continuous predictions
    let nodes: Vec<Node<f32>> = vec![
        Node::new_split(0, 5.0, 1, 2),
        Node::new_leaf(2.5),    // left: predicted value 2.5
        Node::new_leaf(7.8),    // right: predicted value 7.8
    ];
    let tree = InferenceTree::new(&nodes);

    let features = vec![3.0];
    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&2.5));

    let features = vec![10.0];
    let result = tree.predict(&features).unwrap();
    assert_eq!(result, Some(&7.8));
}

#[test]
fn test_predict_regression_deep_tree() {
    // Deeper regression tree with multiple splits
    let nodes: Vec<Node<f32>> = vec![
        Node::new_split(0, 5.0, 1, 2),
        Node::new_split(1, 10.0, 3, 4),
        Node::new_leaf(1.5),
        Node::new_leaf(3.7),
        Node::new_leaf(6.2),
    ];
    let tree = InferenceTree::new(&nodes);

    // Left -> Left: 3.0 <= 5.0, 8.0 <= 10.0
    let features = vec![3.0, 8.0];
    assert_eq!(tree.predict(&features).unwrap(), Some(&3.7));

    // Left -> Right: 3.0 <= 5.0, 12.0 > 10.0
    let features = vec![3.0, 12.0];
    assert_eq!(tree.predict(&features).unwrap(), Some(&6.2));

    // Right: 7.0 > 5.0
    let features = vec![7.0, 5.0];
    assert_eq!(tree.predict(&features).unwrap(), Some(&1.5));
}

#[test]
fn test_predict_regression_negative_continuous_values() {
    // Regression predictions can be negative
    let nodes: Vec<Node<f32>> = vec![
        Node::new_split(0, 0.0, 1, 2),
        Node::new_leaf(-15.5),
        Node::new_leaf(42.3),
    ];
    let tree = InferenceTree::new(&nodes);

    let features = vec![-10.0];
    assert_eq!(tree.predict(&features).unwrap(), Some(&-15.5));

    let features = vec![5.0];
    assert_eq!(tree.predict(&features).unwrap(), Some(&42.3));
}

#[test]
fn test_error_display_invalid_node_index() {
    let error = InferenceError::InvalidNodeIndex;
    assert_eq!(error.to_string(), "Invalid node index. Tree is malformed.");
}

#[test]
fn test_error_display_feature_index_out_of_bounds() {
    let error = InferenceError::FeatureIndexOutOfBounds;
    assert_eq!(error.to_string(), "Feature index out of bounds.");
}
