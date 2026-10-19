mod data;
use data::{RandomGenerator, TrainingExample, cross_entropy, dataset};
use std::io::Write;

#[derive(Clone)]
struct Parameter {
    values: Vec<f32>,
    gradients: Vec<f32>,
}
impl Parameter {
    fn new(values: Vec<f32>) -> Self {
        Self {
            gradients: vec![0.; values.len()],
            values,
        }
    }
    fn update(&mut self, rate: f32) {
        for (value, gradient) in self.values.iter_mut().zip(&self.gradients) {
            *value -= rate * gradient;
        }
    }
}
#[derive(Clone)]
struct Linear {
    inputs: usize,
    outputs: usize,
    weights: Parameter,
    input: Vec<f32>,
    output: Vec<f32>,
}
impl Linear {
    fn new(inputs: usize, outputs: usize, gain: f32, rng: &mut RandomGenerator) -> Self {
        Self {
            inputs,
            outputs,
            weights: Parameter::new(
                (0..inputs * outputs)
                    .map(|_| rng.normal() * gain / (inputs as f32).sqrt())
                    .collect(),
            ),
            input: vec![],
            output: vec![],
        }
    }
    fn forward(&mut self, input: Vec<f32>) -> Vec<f32> {
        let output = {
            let mut output = vec![0.; input.len() / self.inputs * self.outputs];
            for (row, result) in input
                .chunks_exact(self.inputs)
                .zip(output.chunks_exact_mut(self.outputs))
            {
                for (index, &value) in row.iter().enumerate() {
                    for (target, &weight) in result
                        .iter_mut()
                        .zip(&self.weights.values[index * self.outputs..(index + 1) * self.outputs])
                    {
                        *target += value * weight;
                    }
                }
            }

            output
        };
        self.input = input;
        self.output = output.clone();
        output
    }
    fn backward(&mut self, gradient: Vec<f32>) -> Vec<f32> {
        self.weights.gradients.fill(0.);
        let mut input_gradient = vec![0.; self.input.len()];
        for ((input, upstream), downstream) in self
            .input
            .chunks_exact(self.inputs)
            .zip(gradient.chunks_exact(self.outputs))
            .zip(input_gradient.chunks_exact_mut(self.inputs))
        {
            for index in 0..self.inputs {
                for target in 0..self.outputs {
                    self.weights.gradients[index * self.outputs + target] +=
                        input[index] * upstream[target];
                    downstream[index] +=
                        self.weights.values[index * self.outputs + target] * upstream[target];
                }
            }
        }
        input_gradient
    }
}
#[derive(Clone)]
struct BatchNorm {
    width: usize,
    gain: Parameter,
    bias: Parameter,
    running_mean: Vec<f32>,
    running_scale: Vec<f32>,
    momentum: f32,
    epsilon: f32,
    // Первая часть усредняет std, вторая — variance. Это разные алгоритмы.
    average_standard_deviation: bool,
    normalized: Vec<f32>,
    inverse_scale: Vec<f32>,
}
impl BatchNorm {
    fn new(width: usize, first_part: bool, output: bool) -> Self {
        Self {
            width,
            gain: Parameter::new(vec![if output { 0.1 } else { 1. }; width]),
            bias: Parameter::new(vec![0.; width]),
            running_mean: vec![0.; width],
            running_scale: vec![1.; width],
            momentum: if first_part { 0.001 } else { 0.1 },
            epsilon: if first_part { 0. } else { 1e-5 },
            average_standard_deviation: first_part,
            normalized: vec![],
            inverse_scale: vec![],
        }
    }
    fn statistics(input: &[f32], width: usize) -> (Vec<f32>, Vec<f32>) {
        let count = input.len() / width;
        assert!(count > 1);
        let mean = {
            let mut mean = vec![0.; width];
            for row in input.chunks_exact(width) {
                for (total, value) in mean.iter_mut().zip(row) {
                    *total += value / count as f32;
                }
            }

            mean
        };
        let mut variance = vec![0.; width];
        for row in input.chunks_exact(width) {
            for index in 0..width {
                variance[index] += (row[index] - mean[index]).powi(2) / (count - 1) as f32;
            }
        }
        (mean, variance)
    }
    fn forward(&mut self, input: Vec<f32>, training: bool) -> Vec<f32> {
        let (mean, scale) = if training {
            let (mean, variance) = Self::statistics(&input, self.width);
            let scale: Vec<_> = variance
                .iter()
                .map(|v| {
                    if self.average_standard_deviation {
                        v.sqrt()
                    } else {
                        *v
                    }
                })
                .collect();
            for index in 0..self.width {
                self.running_mean[index] =
                    (1. - self.momentum) * self.running_mean[index] + self.momentum * mean[index];
                self.running_scale[index] =
                    (1. - self.momentum) * self.running_scale[index] + self.momentum * scale[index];
            }
            (mean, scale)
        } else {
            (self.running_mean.clone(), self.running_scale.clone())
        };
        self.inverse_scale = scale
            .iter()
            .map(|v| {
                1. / if self.average_standard_deviation {
                    v.max(1e-12)
                } else {
                    (v + self.epsilon).sqrt()
                }
            })
            .collect();
        self.normalized = input
            .iter()
            .enumerate()
            .map(|(index, value)| {
                (value - mean[index % self.width]) * self.inverse_scale[index % self.width]
            })
            .collect();
        self.normalized
            .iter()
            .enumerate()
            .map(|(index, value)| {
                self.gain.values[index % self.width] * value + self.bias.values[index % self.width]
            })
            .collect()
    }
    fn backward(&mut self, gradient: Vec<f32>) -> Vec<f32> {
        let count = gradient.len() / self.width;
        self.gain.gradients.fill(0.);
        self.bias.gradients.fill(0.);
        for (index, value) in gradient.iter().enumerate() {
            self.gain.gradients[index % self.width] += value * self.normalized[index];
            self.bias.gradients[index % self.width] += value;
        }
        // Делитель n-1 соответствует torch.var/std с correction=1.
        gradient
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let column = index % self.width;
                self.gain.values[column]
                    * self.inverse_scale[column]
                    * (value
                        - self.bias.gradients[column] / count as f32
                        - self.normalized[index] * self.gain.gradients[column] / (count - 1) as f32)
            })
            .collect()
    }
}
#[derive(Clone)]
enum Layer {
    Linear(Linear),
    BatchNorm(BatchNorm),
    Tanh {
        output: Vec<f32>,
        gradient: Vec<f32>,
    },
}
impl Layer {
    fn forward(&mut self, input: Vec<f32>, training: bool) -> Vec<f32> {
        match self {
            Self::Linear(layer) => layer.forward(input),
            Self::BatchNorm(layer) => layer.forward(input, training),
            Self::Tanh { output, .. } => {
                *output = input.iter().map(|x| x.tanh()).collect();
                output.clone()
            }
        }
    }
    fn backward(&mut self, upstream: Vec<f32>) -> Vec<f32> {
        match self {
            Self::Linear(layer) => layer.backward(upstream),
            Self::BatchNorm(layer) => layer.backward(upstream),
            Self::Tanh { output, gradient } => {
                *gradient = upstream.clone();
                upstream
                    .iter()
                    .zip(output)
                    .map(|(g, h)| g * (1. - *h * *h))
                    .collect()
            }
        }
    }
}
#[derive(Clone)]
struct Model {
    embeddings: Parameter,
    layers: Vec<Layer>,
    vocabulary_size: usize,
}
impl Model {
    fn new(vocabulary_size: usize, deep: bool, rng: &mut RandomGenerator) -> Self {
        Self {
            embeddings: Parameter::new((0..vocabulary_size * 10).map(|_| rng.normal()).collect()),
            layers: {
                let mut layers = vec![];

                if deep {
                    let mut inputs = 30;
                    for outputs in [100, 100, 100, 100, 100, vocabulary_size] {
                        layers.push(Layer::Linear(Linear::new(inputs, outputs, 1., rng)));
                        layers.push(Layer::BatchNorm(BatchNorm::new(
                            outputs,
                            false,
                            outputs == vocabulary_size,
                        )));
                        if outputs != vocabulary_size {
                            layers.push(Layer::Tanh {
                                output: vec![],
                                gradient: vec![],
                            });
                        }
                        inputs = outputs;
                    }
                } else {
                    layers.push(Layer::Linear(Linear::new(30, 200, 5. / 3., rng)));
                    layers.push(Layer::BatchNorm(BatchNorm::new(200, true, false)));
                    layers.push(Layer::Tanh {
                        output: vec![],
                        gradient: vec![],
                    });
                    // notebook W2 = randn * 0.01; Linear обычно делит на sqrt(fan_in).
                    layers.push(Layer::Linear(Linear::new(
                        200,
                        vocabulary_size,
                        0.01 * 200f32.sqrt(),
                        rng,
                    )));
                    // Выходное смещение b2 хранится отдельно в основном цикле.
                }
                layers
            },
            vocabulary_size,
        }
    }
    fn forward(&mut self, batch: &[TrainingExample], training: bool) -> Vec<f32> {
        let values = {
            let mut values = Vec::with_capacity(batch.len() * 30);
            for example in batch {
                for &character in &example.context_character_indices {
                    values.extend_from_slice(
                        &self.embeddings.values[character * 10..character * 10 + 10],
                    );
                }
            }

            values
        };
        self.layers
            .iter_mut()
            .fold(values, |values, layer| layer.forward(values, training))
    }
    fn parameters(&mut self) -> Vec<&mut Parameter> {
        let mut result = vec![&mut self.embeddings];
        for layer in &mut self.layers {
            match layer {
                Layer::Linear(layer) => result.push(&mut layer.weights),
                Layer::BatchNorm(layer) => {
                    result.push(&mut layer.gain);
                    result.push(&mut layer.bias);
                }
                _ => {}
            }
        }
        result
    }
    fn gradients(&mut self, batch: &[TrainingExample], output_bias: &mut Parameter) -> f32 {
        let (gradient, loss) = {
            let mut gradient = vec![];
            let mut loss = 0.;
            output_bias.gradients.fill(0.);
            for (row, example) in self
                .forward(batch, true)
                .chunks_exact(self.vocabulary_size)
                .zip(batch)
            {
                let probabilities = {
                    let (value, mut probabilities) = cross_entropy(
                        &row.iter()
                            .zip(&output_bias.values)
                            .map(|(a, b)| a + b)
                            .collect::<Vec<_>>(),
                        example.target_character_index,
                    );
                    loss += value / batch.len() as f32;
                    probabilities[example.target_character_index] -= 1.;
                    for (index, p) in probabilities.iter_mut().enumerate() {
                        *p /= batch.len() as f32;
                        output_bias.gradients[index] += *p;
                    }

                    probabilities
                };
                gradient.extend(probabilities);
            }

            (gradient, loss)
        };
        let gradient = self
            .layers
            .iter_mut()
            .rev()
            .fold(gradient, |gradient, layer| layer.backward(gradient));
        self.embeddings.gradients.fill(0.);
        for (example, row) in batch.iter().zip(gradient.as_chunks::<30>().0.iter()) {
            for (position, character) in example.context_character_indices.iter().enumerate() {
                for coordinate in 0..10 {
                    self.embeddings.gradients[character * 10 + coordinate] +=
                        row[position * 10 + coordinate];
                }
            }
        }
        loss
    }
    fn loss(&mut self, examples: &[TrainingExample], bias: &Parameter) -> f64 {
        let mut total = 0.;
        for batch in examples.chunks(256) {
            for (row, example) in self
                .forward(batch, false)
                .chunks_exact(self.vocabulary_size)
                .zip(batch)
            {
                total += cross_entropy(
                    &row.iter()
                        .zip(&bias.values)
                        .map(|(a, b)| a + b)
                        .collect::<Vec<_>>(),
                    example.target_character_index,
                )
                .0 as f64;
            }
        }
        total / examples.len() as f64
    }
}
fn standard_deviation(values: &[f32]) -> f32 {
    let mean = values.iter().sum::<f32>() / values.len() as f32;
    (values.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / (values.len() - 1) as f32).sqrt()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("output");
    std::fs::create_dir_all(&output)?;
    let text = include_str!("../../l247-52-generate-names-with-makemore-mlp/data/names.txt");
    let names: Vec<_> = text.lines().collect();
    let vocabulary = {
        let mut vocabulary: Vec<_> = text.chars().filter(|c| c.is_ascii_lowercase()).collect();
        vocabulary.sort_unstable();
        vocabulary.dedup();
        vocabulary.insert(0, '.');

        vocabulary
    };
    let names = {
        let mut names = names;
        RandomGenerator::new(42).shuffle(&mut names);
        names
    };
    let training = dataset(&names[..names.len() * 8 / 10], &vocabulary);
    let validation = dataset(
        &names[names.len() * 8 / 10..names.len() * 9 / 10],
        &vocabulary,
    );
    let test = dataset(&names[names.len() * 9 / 10..], &vocabulary);
    println!(
        "train/dev/test: {}/{}/{} contexts",
        training.len(),
        validation.len(),
        test.len()
    );
    // Первая сеть: 200 000 шагов; глубокая: 1001 (ячейка 14: break при i >= 1000).
    for (deep, steps, decay) in [(false, 200_000, 100_000), (true, 1001, 150_000)] {
        let prefix = if deep { "deep" } else { "shallow" };
        let (model, bias, curve, update_curves) = {
            let mut rng = RandomGenerator::new(2147483647);
            let mut model = Model::new(vocabulary.len(), deep, &mut rng);
            let mut bias = Parameter::new(vec![0.; vocabulary.len()]);
            println!(
                "{prefix}: {} parameters",
                model
                    .parameters()
                    .iter()
                    .map(|p| p.values.len())
                    .sum::<usize>()
                    + if deep { 0 } else { vocabulary.len() }
            );
            let mut losses = std::io::BufWriter::new(std::fs::File::create(
                output.join(format!("{prefix}_loss.csv")),
            )?);
            let mut ratios = std::io::BufWriter::new(std::fs::File::create(
                output.join(format!("{prefix}_updates.csv")),
            )?);
            writeln!(losses, "step,learning_rate,loss,log10_loss")?;
            writeln!(ratios, "step,parameter,log10_update_to_data_std")?;
            let mut curve = Vec::new();
            let mut update_curves = vec![Vec::new(); model.parameters().len()];
            for step in 0..steps {
                let loss = model.gradients(
                    &(0..32)
                        .map(|_| training[rng.index(training.len())])
                        .collect::<Vec<_>>(),
                    &mut bias,
                );
                assert!(loss.is_finite());
                let rate = if step < decay { 0.1 } else { 0.01 };
                for (index, parameter) in model.parameters().into_iter().enumerate() {
                    parameter.update(rate);
                    let ratio = rate * standard_deviation(&parameter.gradients)
                        / standard_deviation(&parameter.values);
                    writeln!(ratios, "{step},{index},{}", ratio.log10())?;
                    if deep && ratio.is_finite() && ratio > 0. {
                        update_curves[index].push((step as f32, ratio.log10()));
                    }
                }
                if !deep {
                    bias.update(rate);
                }
                writeln!(losses, "{step},{rate},{loss},{}", loss.log10())?;
                curve.push((step as f32, loss.log10()));
                if step % 10_000 == 0 {
                    println!("{prefix} {step}/{steps}: {loss:.4}");
                }
            }
            losses.flush()?;
            ratios.flush()?;

            (model, bias, curve, update_curves)
        };
        plot(
            &output.join(format!("{prefix}_loss.svg")),
            &curve,
            "log10 loss",
        )?;
        for (index, points) in update_curves.iter().enumerate() {
            if !points.is_empty() {
                plot(
                    &output.join(format!("{prefix}_parameter_{index}_updates.svg")),
                    points,
                    "log10(std(update)/std(parameter)); reference -3",
                )?;
            }
        }
        // Сохраняем диагностику последнего train-batch ДО eval, который меняет caches.
        diagnostics(&model, &output, prefix)?;
        {
            // Eval меняет только caches слоёв; параметры и статистики BN не обновляются.
            let mut model = model;
            {
                let mut metrics =
                    std::fs::File::create(output.join(format!("{prefix}_metrics.txt")))?;
                for (name, examples) in
                    [("train", &training), ("dev", &validation), ("test", &test)]
                {
                    let loss = model.loss(examples, &bias);
                    println!("{prefix} {name}: {loss:.6}");
                    writeln!(metrics, "{name}: {loss:.6}")?;
                }
            }
            if !deep {
                // Ячейка 9 вычисляет full-train статистики, но ячейка 10 их не использует.
                let (sums, squares) = {
                    let mut sums = vec![0f64; 200];
                    let mut squares = vec![0f64; 200];
                    for batch in training.chunks(256) {
                        model.forward(batch, false);
                        if let Layer::Linear(first) = &model.layers[0] {
                            for row in first.output.as_chunks::<200>().0 {
                                for i in 0..200 {
                                    sums[i] += row[i] as f64;
                                    squares[i] += (row[i] as f64).powi(2);
                                }
                            }
                        }
                    }

                    (sums, squares)
                };
                let mut calibration = std::fs::File::create(output.join("calibration.csv"))?;
                writeln!(calibration, "neuron,mean,std")?;
                for i in 0..200 {
                    let n = training.len() as f64;
                    writeln!(
                        calibration,
                        "{i},{},{}",
                        sums[i] / n,
                        ((squares[i] - sums[i] * sums[i] / n) / (n - 1.))
                            .max(0.)
                            .sqrt()
                    )?;
                }
            }
            let mut samples = std::fs::File::create(output.join(format!("{prefix}_samples.txt")))?;
            let mut rng = RandomGenerator::new(2147483657);
            for _ in 0..20 {
                let name = {
                    let mut context = [0; 3];
                    let mut name = String::new();
                    for _ in 0..100 {
                        let probabilities = cross_entropy(
                            &model
                                .forward(
                                    &[TrainingExample {
                                        context_character_indices: context,
                                        target_character_index: 0,
                                    }],
                                    false,
                                )
                                .iter()
                                .zip(&bias.values)
                                .map(|(a, b)| a + b)
                                .collect::<Vec<_>>(),
                            0,
                        )
                        .1;
                        let draw = rng.uniform();
                        let selected = {
                            let mut cumulative = 0.;
                            let mut selected = vocabulary.len() - 1;
                            for (i, p) in probabilities.iter().enumerate() {
                                cumulative += p;
                                if draw < cumulative {
                                    selected = i;
                                    break;
                                }
                            }

                            selected
                        };
                        name.push(vocabulary[selected]);
                        if selected == 0 {
                            break;
                        }
                        context = [context[1], context[2], selected];
                    }
                    if !name.ends_with('.') {
                        name.push_str(" [лимит 100 символов]");
                    }

                    name
                };
                writeln!(samples, "{name}")?;
            }
        }
    }
    bonus(&output)?;
    Ok(())
}
fn plot(path: &std::path::Path, points: &[(f32, f32)], title: &str) -> std::io::Result<()> {
    let min_x = points.iter().map(|p| p.0).fold(f32::INFINITY, f32::min);
    let max_x = points.iter().map(|p| p.0).fold(f32::NEG_INFINITY, f32::max);
    let (min_y, max_y) = {
        let min_y = points.iter().map(|p| p.1).fold(f32::INFINITY, f32::min);
        let max_y = points.iter().map(|p| p.1).fold(f32::NEG_INFINITY, f32::max);
        if title.contains("reference -3") {
            (min_y.min(-3.), max_y.max(-3.))
        } else {
            (min_y, max_y)
        }
    };
    let svg = {
        let mut svg = format!(
            "<svg xmlns='http://www.w3.org/2000/svg' width='900' height='500'><rect width='900' height='500' fill='white'/><text x='50' y='25'>{title}; x=[{min_x:.3},{max_x:.3}], y=[{min_y:.3},{max_y:.3}]</text><polyline fill='none' stroke='royalblue' points='"
        );
        for &(x, y) in points {
            svg.push_str(&format!(
                "{:.2},{:.2} ",
                50. + 800. * (x - min_x) / (max_x - min_x).max(1e-12),
                450. - 400. * (y - min_y) / (max_y - min_y).max(1e-12)
            ));
        }
        svg.push_str("'/>");
        if title.contains("reference -3") && min_y <= -3. && max_y >= -3. {
            svg.push_str(&format!(
                "<path d='M50 {}H850' stroke='black' stroke-dasharray='5 5'/>",
                450. - 400. * (-3. - min_y) / (max_y - min_y).max(1e-12),
            ));
        }
        svg.push_str("</svg>");

        svg
    };
    std::fs::write(path, svg)
}
fn histogram(values: &[f32]) -> Vec<(f32, f32)> {
    let min = values.iter().copied().fold(f32::INFINITY, f32::min);
    let width = ((values.iter().copied().fold(f32::NEG_INFINITY, f32::max) - min) / 50.).max(1e-12);
    let bins = {
        let mut bins = [0usize; 50];
        for value in values {
            bins[(((value - min) / width) as usize).min(49)] += 1;
        }

        bins
    };
    bins.iter()
        .enumerate()
        .map(|(i, count)| {
            (
                min + (i as f32 + 0.5) * width,
                *count as f32 / (values.len() as f32 * width),
            )
        })
        .collect()
}
fn diagnostics(model: &Model, output: &std::path::Path, prefix: &str) -> std::io::Result<()> {
    plot(
        &output.join(format!("{prefix}_embedding_gradient.svg")),
        &histogram(&model.embeddings.gradients),
        "embedding gradient density",
    )?;
    {
        let mut weight_statistics =
            std::fs::File::create(output.join(format!("{prefix}_weight_statistics.csv")))?;
        writeln!(
            weight_statistics,
            "parameter,gradient_std,data_std,gradient_to_data_std"
        )?;
        for (index, parameter) in model.clone().parameters().iter().enumerate() {
            let gradient_std = standard_deviation(&parameter.gradients);
            let data_std = standard_deviation(&parameter.values);
            writeln!(
                weight_statistics,
                "{index},{gradient_std},{data_std},{}",
                gradient_std / data_std
            )?;
        }
    }
    let mut file = std::fs::File::create(output.join(format!("{prefix}_diagnostics.csv")))?;
    writeln!(file, "layer,kind,mean,std,saturated_fraction")?;
    for (index, layer) in model.layers.iter().enumerate() {
        for (kind, values) in match layer {
            Layer::Tanh { output, gradient } => {
                vec![("activation", output), ("activation_gradient", gradient)]
            }
            Layer::Linear(linear) => vec![("weight_gradient", &linear.weights.gradients)],
            _ => vec![],
        } {
            writeln!(
                file,
                "{index},{kind},{},{},{}",
                values.iter().sum::<f32>() / values.len() as f32,
                standard_deviation(values),
                values.iter().filter(|v| v.abs() > 0.97).count() as f32 / values.len() as f32,
            )?;
            let points = histogram(values);
            plot(
                &output.join(format!("{prefix}_{index}_{kind}.svg")),
                &points,
                kind,
            )?;
            let mut histogram =
                std::fs::File::create(output.join(format!("{prefix}_{index}_{kind}.csv")))?;
            writeln!(histogram, "bin_center,density")?;
            for (x, y) in points {
                writeln!(histogram, "{x},{y}")?;
            }
        }
    }
    Ok(())
}
fn bonus(output: &std::path::Path) -> std::io::Result<()> {
    // Локальный HTML slider и таблица всех положений x0 с шагом 0.5.
    {
        let mut file = std::fs::File::create(output.join("batch_coupling.csv"))?;
        writeln!(file, "x0,example,input,normalized")?;
        let original = {
            let mut rng = RandomGenerator::new(2147483648);
            let original: Vec<_> = (0..5).map(|_| rng.normal() * 5.).collect();

            original
        };
        std::fs::write(
        output.join("batch_coupling.html"),
        (r##"<!doctype html><meta charset="utf-8"><title>BatchNorm: связь примеров</title>
<style>body{font:18px sans-serif;max-width:950px;margin:30px auto}svg{border:1px solid #ccc}label{display:block}</style>
<h1>BatchNorm: изменение одного примера</h1>
<label>x₀: <input id="slider" type="range" min="-30" max="30" step="0.5" value="0"><output id="number"></output></label>
<p>Синие точки — входы, красные — нормализованные значения. Линии связывают один пример до и после нормализации. Кривые — нормальные плотности с исходными mean/std и с mean=0, std=1.</p>
<svg id="chart" viewBox="0 0 900 400"></svg><pre id="values"></pre>
<script>
const original=ORIGINAL;
const slider=document.getElementById('slider'),chart=document.getElementById('chart');
function render(){
 const x=[...original];x[0]=Number(slider.value);
 const mean=x.reduce((a,b)=>a+b)/5;
 const std=Math.sqrt(x.reduce((a,b)=>a+(b-mean)**2,0)/4);
 const z=x.map(a=>(a-mean)/std);
 const px=a=>450+a*12, py=a=>340-a*180;
 let svg='<path d="M30 340H870" stroke="black"/>';
 for(const [mu,sigma,color] of [[mean,std,'blue'],[0,1,'red']]){
   let points=[];for(let a=-35;a<=35;a+=.1)points.push(px(a)+','+py(Math.exp(-.5*((a-mu)/sigma)**2)/(sigma*Math.sqrt(2*Math.PI))));
   svg+='<polyline fill="none" stroke="'+color+'" points="'+points.join(' ')+'"/>';
 }
 for(let i=0;i<5;i++){
   svg+='<path d="M'+px(x[i])+' 90L'+px(z[i])+' 300" stroke="#aaa"/>';
   svg+='<circle cx="'+px(x[i])+'" cy="90" r="6" fill="blue"/><circle cx="'+px(z[i])+'" cy="300" r="6" fill="red"/>';
 }
 chart.innerHTML=svg;
 document.getElementById('number').textContent=x[0];
 document.getElementById('values').textContent='mean='+mean.toFixed(4)+' std='+std.toFixed(4)+'\ninput: '+x.map(a=>a.toFixed(3)).join(', ')+'\nnormalized: '+z.map(a=>a.toFixed(3)).join(', ');
}
slider.addEventListener('input',render);render();
</script>"##).replace("ORIGINAL", &format!("{original:?}")),
    )?;
        for step in 0..=120 {
            let values = {
                let mut values = original.clone();
                values[0] = -30. + step as f32 * 0.5;

                values
            };
            let (mean, variance) = BatchNorm::statistics(&values, 1);
            for (i, x) in values.iter().enumerate() {
                writeln!(
                    file,
                    "{},{i},{x},{}",
                    values[0],
                    (x - mean[0]) / variance[0].sqrt()
                )?;
            }
        }
    }
    let mut file = std::fs::File::create(output.join("linear_statistics.csv"))?;
    writeln!(file, "batch_norm,tensor,std")?;
    for normalized in [false, true] {
        let (input, mut linear, values, upstream) = {
            let mut rng = RandomGenerator::new(2147483647);
            let input: Vec<_> = (0..1000).map(|_| rng.normal()).collect();
            let mut linear = Linear::new(1000, 1000, 1000f32.sqrt(), &mut rng);
            let values = linear.forward(input.clone());
            let upstream: Vec<_> = (0..1000).map(|_| rng.normal()).collect();
            (input, linear, values, upstream)
        };
        let (out, gradient) = if normalized {
            let mut bn = BatchNorm::new(1, false, false);
            (
                bn.forward(values.clone(), true),
                bn.backward(upstream.clone()),
            )
        } else {
            (values.clone(), upstream.clone())
        };
        let input_gradient = linear.backward(gradient.clone());
        let linear = linear;
        for (name, values) in [
            ("input", &input),
            ("weight", &linear.weights.values),
            ("linear_output", &values),
            ("output", &out),
            ("output_gradient", &upstream),
            ("linear_gradient", &gradient),
            ("input_gradient", &input_gradient),
            ("weight_gradient", &linear.weights.gradients),
        ] {
            writeln!(file, "{normalized},{name},{}", standard_deviation(values))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_norm_backward_matches_finite_differences() {
        for first_part in [false, true] {
            let input = vec![0.2, -1.1, 0.7, 0.3, -0.4, 1.2, 2.1, -0.7];
            let upstream = vec![0.3, -0.2, 0.4, 0.8, -0.1, 0.5, 0.2, -0.6];
            let (layer, analytic) = {
                let mut layer = BatchNorm::new(2, first_part, false);
                layer.gain.values = vec![1.3, 0.6];
                layer.forward(input.clone(), true);
                let analytic = layer.backward(upstream.clone());

                (layer, analytic)
            };
            for index in 0..input.len() {
                let plus = {
                    let mut plus = input.clone();
                    plus[index] += 0.001;
                    plus
                };
                let minus = {
                    let mut minus = input.clone();
                    minus[index] -= 0.001;
                    minus
                };
                let objective = |values: Vec<f32>| {
                    let mut copy = layer.clone();
                    copy.forward(values, true)
                        .iter()
                        .zip(&upstream)
                        .map(|(x, g)| x * g)
                        .sum::<f32>()
                };
                let numeric = (objective(plus) - objective(minus)) / 0.002;
                assert!(
                    (numeric - analytic[index]).abs() < 0.0005,
                    "{first_part} {index}: {numeric} vs {}",
                    analytic[index]
                );
            }
        }
    }

    #[test]
    fn full_network_gradients_include_embeddings_and_all_layer_types() {
        let batch = dataset(&["aba", "baa"], &['.', 'a', 'b']);
        for deep in [false, true] {
            let (mut model, bias) = {
                let mut model = Model::new(3, deep, &mut RandomGenerator::new(55));
                let mut bias = Parameter::new(vec![0.; 3]);
                model.gradients(&batch, &mut bias);
                (model, bias)
            };
            for tensor in 0..model.parameters().len() {
                let (index, analytic) = {
                    let p = &model.parameters()[tensor];
                    let index = p
                        .gradients
                        .iter()
                        .enumerate()
                        .max_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
                        .unwrap()
                        .0;
                    (index, p.gradients[index])
                };
                let objective = |offset| {
                    let mut copy = model.clone();
                    copy.parameters()[tensor].values[index] += offset;
                    copy.gradients(&batch, &mut bias.clone())
                };
                let numeric = (objective(0.001) - objective(-0.001)) / 0.002;
                assert!(
                    (numeric - analytic).abs() < 0.003,
                    "deep={deep} tensor={tensor}: {numeric} vs {analytic}"
                );
            }
        }
    }

    #[test]
    fn eval_uses_frozen_statistics_and_is_independent_of_companions() {
        let mut layer = BatchNorm::new(2, false, false);
        layer.forward(vec![1., 2., 3., 7., -2., 4.], true);
        let mean = layer.running_mean.clone();
        let variance = layer.running_scale.clone();
        assert_eq!(
            layer.forward(vec![1., 2.], false),
            &layer.forward(vec![1., 2., 100., -30.], false)[..2],
        );
        assert_eq!(mean, layer.running_mean);
        assert_eq!(variance, layer.running_scale);
        // torch correction=1: variance([1,3]) = 2, а не 1.
        assert_eq!(BatchNorm::statistics(&[1., 3.], 1), (vec![2.], vec![2.]));
        assert!(
            BatchNorm::new(2, false, false)
                .forward(vec![3.; 8], true)
                .iter()
                .all(|x| x.is_finite())
        );
    }

    #[test]
    fn sgd_learns_and_dataset_resets_context() {
        let batch = dataset(&["ab", "ab", "ab", "ab"], &['.', 'a', 'b']);
        assert_eq!(batch[3].context_character_indices, [0; 3]);
        for deep in [false, true] {
            let mut model = Model::new(3, deep, &mut RandomGenerator::new(42));
            let mut bias = Parameter::new(vec![0.; 3]);
            let initial = model.gradients(&batch, &mut bias);
            for _ in 0..100 {
                model.gradients(&batch, &mut bias);
                for p in model.parameters() {
                    p.update(0.1);
                }
                if !deep {
                    bias.update(0.1);
                }
            }
            let final_loss = model.gradients(&batch, &mut bias);
            assert!(
                final_loss < initial * 0.5,
                "{deep}: {initial} -> {final_loss}"
            );
        }
    }
}
