// Machine Learning Library with Neural Networks and Backpropagation
// Implements linear regression, logistic regression, and multi-layer perceptrons

use std::f64::consts::E;

// ========== MATRIX OPERATIONS ==========
#[derive(Debug, Clone)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    fn from_vec(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(rows * cols, data.len());
        Matrix { rows, cols, data }
    }

    fn zeros(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols)
    }

    fn ones(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![1.0; rows * cols],
        }
    }

    fn random(rows: usize, cols: usize, scale: f64) -> Self {
        let mut data = Vec::with_capacity(rows * cols);
        for i in 0..rows * cols {
            let pseudo_random = (i as f64 * 12.9898).sin() * 43758.5453;
            data.push((pseudo_random.fract() - 0.5) * 2.0 * scale);
        }
        Matrix { rows, cols, data }
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    fn add(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }

    fn sub(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }

    fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);
        
        let mut result = Matrix::new(self.rows, other.cols);
        
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        
        result
    }

    fn hadamard(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .collect();
        
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }

    fn scale(&self, scalar: f64) -> Matrix {
        let data: Vec<f64> = self.data.iter().map(|x| x * scalar).collect();
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }

    fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j));
            }
        }
        result
    }

    fn map<F>(&self, f: F) -> Matrix
    where
        F: Fn(f64) -> f64,
    {
        let data: Vec<f64> = self.data.iter().map(|&x| f(x)).collect();
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }

    fn sum(&self) -> f64 {
        self.data.iter().sum()
    }

    fn mean(&self) -> f64 {
        self.sum() / (self.rows * self.cols) as f64
    }
}

// ========== ACTIVATION FUNCTIONS ==========
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + E.powf(-x))
}

fn sigmoid_derivative(x: f64) -> f64 {
    let s = sigmoid(x);
    s * (1.0 - s)
}

fn relu(x: f64) -> f64 {
    x.max(0.0)
}

fn relu_derivative(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else {
        0.0
    }
}

fn tanh(x: f64) -> f64 {
    x.tanh()
}

fn tanh_derivative(x: f64) -> f64 {
    let t = tanh(x);
    1.0 - t * t
}

// ========== LINEAR REGRESSION ==========
struct LinearRegression {
    weights: Matrix,
    bias: f64,
    learning_rate: f64,
}

impl LinearRegression {
    fn new(features: usize, learning_rate: f64) -> Self {
        LinearRegression {
            weights: Matrix::random(features, 1, 0.1),
            bias: 0.0,
            learning_rate,
        }
    }

    fn predict(&self, x: &Matrix) -> Matrix {
        x.multiply(&self.weights).map(|v| v + self.bias)
    }

    fn train(&mut self, x: &Matrix, y: &Matrix, epochs: usize) -> Vec<f64> {
        let m = x.rows as f64;
        let mut losses = Vec::new();

        for epoch in 0..epochs {
            let predictions = self.predict(x);
            let errors = predictions.sub(y);

            let loss = errors.hadamard(&errors).sum() / (2.0 * m);
            losses.push(loss);

            if epoch % 100 == 0 {
                println!("Epoch {}: Loss = {:.6}", epoch, loss);
            }

            let x_transpose = x.transpose();
            let gradient = x_transpose.multiply(&errors).scale(1.0 / m);
            let bias_gradient = errors.sum() / m;

            self.weights = self.weights.sub(&gradient.scale(self.learning_rate));
            self.bias -= self.learning_rate * bias_gradient;
        }

        losses
    }
}

// ========== LOGISTIC REGRESSION ==========
struct LogisticRegression {
    weights: Matrix,
    bias: f64,
    learning_rate: f64,
}

impl LogisticRegression {
    fn new(features: usize, learning_rate: f64) -> Self {
        LogisticRegression {
            weights: Matrix::random(features, 1, 0.1),
            bias: 0.0,
            learning_rate,
        }
    }

    fn predict(&self, x: &Matrix) -> Matrix {
        x.multiply(&self.weights)
            .map(|v| sigmoid(v + self.bias))
    }

    fn train(&mut self, x: &Matrix, y: &Matrix, epochs: usize) -> Vec<f64> {
        let m = x.rows as f64;
        let mut losses = Vec::new();

        for epoch in 0..epochs {
            let predictions = self.predict(x);
            let errors = predictions.sub(y);

            let loss = errors.hadamard(&errors).sum() / (2.0 * m);
            losses.push(loss);

            if epoch % 100 == 0 {
                println!("Epoch {}: Loss = {:.6}", epoch, loss);
            }

            let x_transpose = x.transpose();
            let gradient = x_transpose.multiply(&errors).scale(1.0 / m);
            let bias_gradient = errors.sum() / m;

            self.weights = self.weights.sub(&gradient.scale(self.learning_rate));
            self.bias -= self.learning_rate * bias_gradient;
        }

        losses
    }

    fn classify(&self, x: &Matrix) -> Matrix {
        self.predict(x).map(|p| if p >= 0.5 { 1.0 } else { 0.0 })
    }
}

// ========== NEURAL NETWORK ==========
struct Layer {
    weights: Matrix,
    biases: Matrix,
    activation: fn(f64) -> f64,
    activation_derivative: fn(f64) -> f64,
}

impl Layer {
    fn new(
        input_size: usize,
        output_size: usize,
        activation: fn(f64) -> f64,
        activation_derivative: fn(f64) -> f64,
    ) -> Self {
        Layer {
            weights: Matrix::random(input_size, output_size, 0.5),
            biases: Matrix::zeros(1, output_size),
            activation,
            activation_derivative,
        }
    }

    fn forward(&self, input: &Matrix) -> (Matrix, Matrix) {
        let z = input.multiply(&self.weights).add(&self.biases);
        let a = z.map(self.activation);
        (z, a)
    }
}

struct NeuralNetwork {
    layers: Vec<Layer>,
    learning_rate: f64,
}

impl NeuralNetwork {
    fn new(learning_rate: f64) -> Self {
        NeuralNetwork {
            layers: Vec::new(),
            learning_rate,
        }
    }

    fn add_layer(
        &mut self,
        input_size: usize,
        output_size: usize,
        activation: fn(f64) -> f64,
        activation_derivative: fn(f64) -> f64,
    ) {
        self.layers.push(Layer::new(
            input_size,
            output_size,
            activation,
            activation_derivative,
        ));
    }

    fn forward(&self, input: &Matrix) -> Vec<(Matrix, Matrix)> {
        let mut layer_outputs = Vec::new();
        let mut current_input = input.clone();

        for layer in &self.layers {
            let (z, a) = layer.forward(&current_input);
            layer_outputs.push((z, a.clone()));
            current_input = a;
        }

        layer_outputs
    }

    fn backward(
        &mut self,
        x: &Matrix,
        y: &Matrix,
        layer_outputs: &[(Matrix, Matrix)],
    ) {
        let m = x.rows as f64;
        let num_layers = self.layers.len();

        let last_activation = &layer_outputs[num_layers - 1].1;
        let mut delta = last_activation.sub(y);

        for i in (0..num_layers).rev() {
            let (z, a) = &layer_outputs[i];
            
            let activation_grad = z.map(self.layers[i].activation_derivative);
            delta = delta.hadamard(&activation_grad);

            let prev_activation = if i == 0 {
                x.clone()
            } else {
                layer_outputs[i - 1].1.clone()
            };

            let weight_gradient = prev_activation.transpose().multiply(&delta).scale(1.0 / m);
            let bias_gradient = Matrix::from_vec(
                1,
                delta.cols,
                (0..delta.cols)
                    .map(|j| {
                        (0..delta.rows).map(|i| delta.get(i, j)).sum::<f64>() / m
                    })
                    .collect(),
            );

            self.layers[i].weights = self.layers[i]
                .weights
                .sub(&weight_gradient.scale(self.learning_rate));
            self.layers[i].biases = self.layers[i]
                .biases
                .sub(&bias_gradient.scale(self.learning_rate));

            if i > 0 {
                delta = delta.multiply(&self.layers[i].weights.transpose());
            }
        }
    }

    fn train(&mut self, x: &Matrix, y: &Matrix, epochs: usize) -> Vec<f64> {
        let mut losses = Vec::new();

        for epoch in 0..epochs {
            let layer_outputs = self.forward(x);
            let predictions = &layer_outputs[self.layers.len() - 1].1;

            let loss = predictions
                .sub(y)
                .hadamard(&predictions.sub(y))
                .sum()
                / (2.0 * x.rows as f64);
            losses.push(loss);

            if epoch % 100 == 0 {
                println!("Epoch {}: Loss = {:.6}", epoch, loss);
            }

            self.backward(x, y, &layer_outputs);
        }

        losses
    }

    fn predict(&self, x: &Matrix) -> Matrix {
        let layer_outputs = self.forward(x);
        layer_outputs[self.layers.len() - 1].1.clone()
    }
}

// ========== MAIN ==========
fn main() {
    println!("=== Machine Learning Library Demo ===\n");

    // Example 1: Linear Regression
    println!("=== Example 1: Linear Regression ===");
    println!("Training a model to fit y = 2x + 3\n");

    let x_train = Matrix::from_vec(5, 1, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let y_train = Matrix::from_vec(5, 1, vec![5.0, 7.0, 9.0, 11.0, 13.0]);

    let mut linear_model = LinearRegression::new(1, 0.01);
    linear_model.train(&x_train, &y_train, 1000);

    let x_test = Matrix::from_vec(3, 1, vec![6.0, 7.0, 8.0]);
    let predictions = linear_model.predict(&x_test);
    
    println!("\nPredictions:");
    for i in 0..x_test.rows {
        println!("  x = {:.1} => y = {:.2}", x_test.get(i, 0), predictions.get(i, 0));
    }
    println!("  (Expected: 15.0, 17.0, 19.0)\n");

    // Example 2: Logistic Regression
    println!("\n=== Example 2: Logistic Regression ===");
    println!("Binary classification problem\n");

    let x_class = Matrix::from_vec(
        6,
        2,
        vec![
            1.0, 1.0,
            2.0, 2.0,
            3.0, 3.0,
            4.0, 1.0,
            5.0, 2.0,
            6.0, 1.0,
        ],
    );
    let y_class = Matrix::from_vec(6, 1, vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);

    let mut logistic_model = LogisticRegression::new(2, 0.1);
    logistic_model.train(&x_class, &y_class, 1000);

    let classifications = logistic_model.classify(&x_class);
    println!("\nClassifications:");
    for i in 0..x_class.rows {
        println!(
            "  ({:.0}, {:.0}) => Class {} (Expected: {})",
            x_class.get(i, 0),
            x_class.get(i, 1),
            classifications.get(i, 0) as i32,
            y_class.get(i, 0) as i32
        );
    }

    // Example 3: Neural Network (XOR problem)
    println!("\n\n=== Example 3: Neural Network (XOR Problem) ===");
    println!("Training a 2-layer network to solve XOR\n");

    let x_xor = Matrix::from_vec(4, 2, vec![0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]);
    let y_xor = Matrix::from_vec(4, 1, vec![0.0, 1.0, 1.0, 0.0]);

    let mut nn = NeuralNetwork::new(0.5);
    nn.add_layer(2, 4, tanh, tanh_derivative);
    nn.add_layer(4, 1, sigmoid, sigmoid_derivative);

    nn.train(&x_xor, &y_xor, 2000);

    let nn_predictions = nn.predict(&x_xor);
    println!("\nXOR Predictions:");
    for i in 0..x_xor.rows {
        let pred = nn_predictions.get(i, 0);
        let pred_class = if pred >= 0.5 { 1 } else { 0 };
        println!(
            "  ({:.0}, {:.0}) => {:.4} (Class {}, Expected: {})",
            x_xor.get(i, 0),
            x_xor.get(i, 1),
            pred,
            pred_class,
            y_xor.get(i, 0) as i32
        );
    }

    // Example 4: Multi-class Neural Network
    println!("\n\n=== Example 4: Neural Network (Regression) ===");
    println!("Training network to approximate f(x) = x^2\n");

    let x_square = Matrix::from_vec(10, 1, vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]);
    let y_square = x_square.map(|x| x * x);

    let mut nn_reg = NeuralNetwork::new(0.3);
    nn_reg.add_layer(1, 8, relu, relu_derivative);
    nn_reg.add_layer(8, 8, relu, relu_derivative);
    nn_reg.add_layer(8, 1, |x| x, |_| 1.0);

    nn_reg.train(&x_square, &y_square, 1000);

    let reg_predictions = nn_reg.predict(&x_square);
    println!("\nRegression Predictions:");
    for i in 0..x_square.rows {
        println!(
            "  f({:.1}) = {:.4} (Expected: {:.4})",
            x_square.get(i, 0),
            reg_predictions.get(i, 0),
            y_square.get(i, 0)
        );
    }

    println!("\n✓ Machine Learning demonstrations complete!");
    println!("\nKey features demonstrated:");
    println!("  • Custom matrix operations with proper bounds checking");
    println!("  • Linear regression with gradient descent");
    println!("  • Logistic regression for binary classification");
    println!("  • Multi-layer neural network with backpropagation");
    println!("  • Multiple activation functions (sigmoid, tanh, ReLU)");
    println!("  • XOR problem solved with hidden layers");
}
