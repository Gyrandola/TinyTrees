# Inference Tests Documentation

Test suite for the `InferenceTree` component.

## Test Coverage

### Basic Tree Operations 

1. **test_inference_tree_creation**: Verify that a tree is created correctly with proper node count and non-empty state
2. **test_empty_tree**: Ensure an empty tree behaves correctly (no root, zero length, is_empty returns true)
3. **test_single_leaf_tree**: Verify single-node trees work properly with leaf extraction
4. **test_get_node_at_index**: Test accessing nodes by index and boundary conditions
5. **test_tree_with_many_features**: Verify the tree handles feature indices up to 255 (u8)

### Classification Prediction 

1. **test_predict_empty_tree**: Predictions on empty trees return `Ok(None)`
2. **test_predict_single_leaf**: Direct leaf prediction works
3. **test_predict_left_branch**: Traversal using `feature_value <= threshold` goes left correctly
4. **test_predict_right_then_left**: Multi-level traversal with correct branching
5. **test_predict_right_then_right**: Multi-level traversal to rightmost leaf
6. **test_predict_boundary_threshold_equal_goes_left**: Boundary test that `<=` (not `<`) is used for thresholds
7. **test_predict_deep_tree**: Deep tree traversal (7 nodes, 3 levels)
8. **test_predict_with_negative_features**: Negative feature values work correctly
9. **test_predict_with_float_features**: High-precision floating-point comparisons work

### Regression Prediction

1. **test_predict_regression_float_predictions**: Regression tree with continuous `f32` predictions (2.5, 7.8)
2. **test_predict_regression_deep_tree**: Multi-level regression traversal with values (1.5, 3.7, 6.2)
3. **test_predict_regression_negative_continuous_values**: Regression supports negative predictions (-15.5, 42.3)

### Generic Data Types 

1. **test_predict_string_predictions**: Generic prediction type `L` works with `&str` ("cat", "dog")

### Tree Reusability 

1. **test_multiple_predictions_same_tree**: Same tree instance can make multiple predictions with different feature sets

### Error Handling

1. **test_predict_feature_index_out_of_bounds**: Tree correctly returns `InferenceError::FeatureIndexOutOfBounds` when accessing invalid feature indices
2. **test_predict_invalid_node_index**: Tree correctly returns `InferenceError::InvalidNodeIndex` when traversal points to nonexistent nodes
3. **test_error_display_invalid_node_index**: Verify error message for malformed trees
4. **test_error_display_feature_index_out_of_bounds**: Verify error message for missing features


## Running Tests

Run all inference tests:
```bash
cargo test --test inference_tests
```

Run a specific test:
```bash
cargo test --test inference_tests test_name
```

Run with output:
```bash
cargo test --test inference_tests -- --nocapture
```